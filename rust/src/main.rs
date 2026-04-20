use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct AnalysisResponse {
    front_image_bytes: usize,
    left_image_bytes: usize,
    symmetry_score: f64,
    jawline_projection_index: f64,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/analyze", post(analyze));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind listener");

    println!("Rust MVP backend (axum) listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("server failed");
}

async fn health() -> &'static str {
    "ok"
}

async fn analyze(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut front_image_bytes = None;
    let mut left_image_bytes = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let field_name = field.name().map(str::to_owned);
        let data = field
            .bytes()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        match field_name.as_deref() {
            Some("front_image") => front_image_bytes = Some(data.len()),
            Some("left_image") => left_image_bytes = Some(data.len()),
            _ => {}
        }
    }

    let front = front_image_bytes.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "front_image is required".to_string(),
        )
    })?;
    let left = left_image_bytes
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "left_image is required".to_string()))?;

    Ok(Json(calculate_measurements(front, left)))
}

fn calculate_measurements(front_image_bytes: usize, left_image_bytes: usize) -> AnalysisResponse {
    let max_size = front_image_bytes.max(left_image_bytes).max(1);
    let size_gap = front_image_bytes.abs_diff(left_image_bytes) as f64;
    let symmetry_score =
        ((100.0 - (size_gap / max_size as f64 * 100.0)) * 100.0).round() / 100.0;
    let jawline_projection_index =
        (((front_image_bytes + left_image_bytes) as f64 / (2.0 * max_size as f64)) * 100.0 * 100.0)
            .round()
            / 100.0;

    AnalysisResponse {
        front_image_bytes,
        left_image_bytes,
        symmetry_score,
        jawline_projection_index,
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_measurements;

    #[test]
    fn calculates_expected_measurements() {
        let result = calculate_measurements(200, 100);
        assert_eq!(result.front_image_bytes, 200);
        assert_eq!(result.left_image_bytes, 100);
        assert_eq!(result.symmetry_score, 50.0);
        assert_eq!(result.jawline_projection_index, 75.0);
    }
}

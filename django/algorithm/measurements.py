"""Simple QOVES-like placeholder measurements from two uploaded images."""


def extract_measurements(front_image, left_image):
    front_bytes = len(front_image.read())
    left_bytes = len(left_image.read())
    max_size = max(front_bytes, left_bytes, 1)
    size_gap = abs(front_bytes - left_bytes)
    symmetry_score = round(100 - (size_gap / max_size * 100), 2)
    jawline_projection_index = round(((front_bytes + left_bytes) / (2 * max_size)) * 100, 2)

    return {
        "front_image_bytes": front_bytes,
        "left_image_bytes": left_bytes,
        "symmetry_score": symmetry_score,
        "jawline_projection_index": jawline_projection_index,
    }

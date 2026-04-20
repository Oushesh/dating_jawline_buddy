# dating_jawline_buddy
This is supposed to be an app in the future where you can get the male/female dimorphism of your face. A bit similar to qoves research.

## MVP structure

- `/django`: Django web MVP where users upload **front** and **left side** photos.
  - `/django/algorithm`: all measurements extraction logic.
- `/rust`: equivalent backend API implementation.
  - For this MVP, **Axum** is used over Actix because it is lightweight, ergonomic, and quick to ship with for a small service.
- `/iOS App`: placeholder folder for future iOS migration.

## Run Django MVP

```bash
pip install -r requirements.txt
cd django
python manage.py migrate
python manage.py runserver
```

## Run Rust MVP backend

```bash
cd rust
cargo run
```

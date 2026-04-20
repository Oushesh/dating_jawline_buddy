from django.core.files.uploadedfile import SimpleUploadedFile
from django.test import TestCase


class UploadAndAnalyzeViewTests(TestCase):
    def test_index_page_loads(self):
        response = self.client.get("/")
        self.assertEqual(response.status_code, 200)
        self.assertContains(response, "Jawline Buddy MVP (Django)")

    def test_upload_returns_measurements(self):
        front = SimpleUploadedFile("front.jpg", b"front-bytes", content_type="image/jpeg")
        left = SimpleUploadedFile("left.jpg", b"left-side-bytes", content_type="image/jpeg")

        response = self.client.post("/", {"front_image": front, "left_image": left})

        self.assertEqual(response.status_code, 200)
        self.assertContains(response, "Estimated measurements")
        self.assertContains(response, "symmetry_score")

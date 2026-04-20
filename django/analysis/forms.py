from django import forms


class FaceUploadForm(forms.Form):
    front_image = forms.FileField(
        label="Front image",
        widget=forms.ClearableFileInput(attrs={"accept": "image/*"}),
    )
    left_image = forms.FileField(
        label="Left side image",
        widget=forms.ClearableFileInput(attrs={"accept": "image/*"}),
    )

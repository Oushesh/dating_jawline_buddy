from django.shortcuts import render

from algorithm.measurements import extract_measurements

from .forms import FaceUploadForm


def upload_and_analyze(request):
    form = FaceUploadForm(request.POST or None, request.FILES or None)
    measurements = None

    if request.method == "POST" and form.is_valid():
        measurements = extract_measurements(
            form.cleaned_data["front_image"],
            form.cleaned_data["left_image"],
        )

    return render(
        request,
        "analysis/index.html",
        {"form": form, "measurements": measurements},
    )

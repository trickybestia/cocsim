from typing import Annotated, List
from io import BytesIO

from fastapi import FastAPI, Form, UploadFile
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import Response
from pydantic import BaseModel
from PIL import Image

from cocsim.map_editor_gui.compose_base_images import compose_base_images

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=("http://localhost:5173"),
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.post("/api/compose-base-images")
async def compose_base_images_api(
    left: list[UploadFile], right: list[UploadFile]
):
    print(left, right)
    left_images = []

    for image in left:
        left_images.append(Image.open(BytesIO(await image.read())))

    right_images = []

    for image in right:
        right_images.append(Image.open(BytesIO(await image.read())))

    result = BytesIO()

    compose_base_images(left_images, right_images).save(result, "jpeg")

    return Response(result.getvalue(), media_type="image/jpeg")

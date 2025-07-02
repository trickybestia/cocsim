from io import BytesIO

from fastapi import FastAPI, UploadFile
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import Response
from PIL import Image

from cocsim.buildings.building import BUILDINGS
from cocsim.compose_base_images import compose_base_images, reverse_projection

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
    left_images = []

    for image in left:
        left_images.append(Image.open(BytesIO(await image.read())))

    right_images = []

    for image in right:
        right_images.append(Image.open(BytesIO(await image.read())))

    result = BytesIO()

    compose_base_images(left_images, right_images).save(result, "jpeg")

    return Response(result.getvalue(), media_type="image/jpeg")


@app.post("/api/reverse-projection")
async def reverse_projection_api(image: UploadFile):
    pil_image = Image.open(BytesIO(await image.read()))

    result = BytesIO()

    reverse_projection(pil_image).save(result, "jpeg")

    return Response(result.getvalue(), media_type="image/jpeg")


@app.get("/api/get-building-types")
async def get_building_types():
    result = []

    for building in BUILDINGS:
        building_dto = {
            "name": building.__name__,
            "width": building.width(),
            "height": building.height(),
            "levels": building.levels(),
            "options": [
                {"name": option.name, "values": option.values}
                for option in building.options()
            ],
        }

        result.append(building_dto)

    return result

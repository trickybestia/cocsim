from io import BytesIO

from fastapi import FastAPI, UploadFile
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import Response
from PIL import Image

from cocsim.buildings.building import BUILDINGS
from cocsim.compose_base_images import compose_base_images, reverse_projection
from cocsim.consts import *
from cocsim.dto_game_renderer import DTOGameRenderer
from cocsim.game import Game
from cocsim.units import Barbarian
from cocsim.utils import load_test_map, load_test_map_raw

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


@app.get("/api/get-showcase-attack-base-image")
def get_showcase_attack_base_image():
    map, base_image = load_test_map_raw("single_player/goblin_gauntlet")

    return Response(base_image, media_type="image/jpeg")


@app.get("/api/get-showcase-attack")
def get_showcase_attack():
    map, base_image = load_test_map("single_player/goblin_gauntlet")

    game = Game(map)

    for y in range(19):
        game.units.append(Barbarian(game, 1, 0.5, y + 0.5))
    for x in range(1, 25):
        game.units.append(Barbarian(game, 1, x + 0.5, 0.5))

    renderer = DTOGameRenderer(10)

    renderer.draw(game)

    while not game.done:
        game.tick(1 / FPS)
        renderer.draw(game)

    renderer.finish(game)

    return renderer.result

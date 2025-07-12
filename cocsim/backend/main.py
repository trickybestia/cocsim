from io import BytesIO

from fastapi import FastAPI, UploadFile, WebSocket
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import Response
from PIL import Image
from starlette.concurrency import iterate_in_threadpool

from cocsim.attack_optimizer import AttackPlanExecutor, AttackPlanOptimizer
from cocsim.buildings.building import BUILDINGS
from cocsim.compose_base_images import compose_base_images, reverse_projection
from cocsim.consts import *
from cocsim.dto_game_renderer import DTOGameRenderer
from cocsim.game import Game
from cocsim.units import UNITS, Barbarian, create_units_model
from cocsim.utils import load_test_map, load_test_map_raw, round_floats

from ..map_model import create_map_model

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=("http://localhost:5173"),
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.post("/api/compose-base-images")
def compose_base_images_api(left: list[UploadFile], right: list[UploadFile]):
    left_images = []

    for image in left:
        left_images.append(Image.open(BytesIO(image.file.read())))

    right_images = []

    for image in right:
        right_images.append(Image.open(BytesIO(image.file.read())))

    result = BytesIO()

    compose_base_images(left_images, right_images).save(result, "jpeg")

    return Response(result.getvalue(), media_type="image/jpeg")


@app.post("/api/reverse-projection")
def reverse_projection_api(image: UploadFile):
    pil_image = Image.open(BytesIO(image.file.read()))

    result = BytesIO()

    reverse_projection(pil_image).save(result, "jpeg")

    return Response(result.getvalue(), media_type="image/jpeg")


@app.get("/api/get-building-types")
def get_building_types():
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


@app.get("/api/get-unit-types")
def get_unit_types():
    result = []

    for unit in UNITS:
        unit_dto = {
            "name": unit.__name__,
            "levels": unit.levels(),
            "housingSpace": unit.housing_space(),
        }

        result.append(unit_dto)

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

    renderer = DTOGameRenderer(1)

    renderer.draw(game)

    while not game.done:
        game.tick(1 / FPS)
        renderer.draw(game)

    renderer.finish(game)

    return round_floats(renderer.result, 2)


OPTIMIZE_ATTACK_ITERATIONS = 2


@app.websocket("/api/optimize-attack")
async def optimize_attack(websocket: WebSocket):
    await websocket.accept()

    map_dict = await websocket.receive_json()
    map = create_map_model()(**map_dict)

    units_dict = await websocket.receive_json()
    units = create_units_model()(**{"units": units_dict})

    await websocket.send_json(
        {
            "type": "progress",
            "progress": "Attack optimization process started...",
        }
    )

    optimizer = AttackPlanOptimizer(map, units)

    async for i, score, attack_plan in iterate_in_threadpool(optimizer.run()):
        await websocket.send_json(
            {
                "type": "progress",
                "progress": f"Gen. #{i} best plan finished in {MAX_ATTACK_DURATION - score:.2f} seconds",
            }
        )

        if i >= OPTIMIZE_ATTACK_ITERATIONS - 1:
            break

    await websocket.send_json(
        {
            "type": "progress",
            "progress": "Attack optimization done, rendering result...",
        }
    )

    game = Game(map)
    attack_plan_executor = AttackPlanExecutor(game, attack_plan)

    renderer = DTOGameRenderer(1)

    attack_plan_executor.tick()
    renderer.draw(game)

    while not game.done:
        attack_plan_executor.tick()  # no problem calling it twice on first loop iteration
        game.tick(1 / FPS)
        renderer.draw(game)

    renderer.finish(game)

    await websocket.send_json(
        {"type": "result", "result": round_floats(renderer.result, 2)}
    )

    await websocket.close()

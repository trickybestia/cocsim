import json
from pathlib import Path

import pygame

from .spin_timer import SpinTimer
from .units.barbarian import Barbarian
from .consts import *
from .game import Game, MapBuilding


def load_test_map(name: str) -> list[MapBuilding]:
    path = Path("test_maps") / (name + ".json")

    return json.loads(path.read_text())


def main():
    pygame.init()

    screen = pygame.display.set_mode(
        (MAP_SIZE * PIXELS_PER_TILE, MAP_SIZE * PIXELS_PER_TILE)
    )

    map = load_test_map("practice_giant_smash")

    game = Game(map)

    game.screen = screen

    bb1 = Barbarian(game)
    bb1.x = 1
    bb1.y = 1

    bb2 = Barbarian(game)
    bb2.x = 11
    bb2.y = 1

    game.units = [bb1, bb2]

    timer = SpinTimer(1 / FPS)

    def on_tick(delta_t: float | None):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                timer.stop()

                return

        game.draw()

        if not game.done and delta_t is not None:
            game.tick(delta_t)

        pygame.display.update()

    timer.on_tick = on_tick

    timer.run()


main()

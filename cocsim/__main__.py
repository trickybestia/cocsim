import pygame

from .utils import load_test_map
from .spin_timer import SpinTimer
from .units.barbarian import Barbarian
from .consts import *
from .game import Game


def main():
    pygame.init()
    pygame.display.set_caption("cocsim")

    map, base_image = load_test_map("single_player/goblin_gauntlet")

    game = Game(map, base_image)

    screen = pygame.display.set_mode(
        (game.total_size * PIXELS_PER_TILE, game.total_size * PIXELS_PER_TILE)
    )

    game.screen = screen

    game.units = []

    for y in range(19):
        game.units.append(Barbarian(game, 0.5, y + 0.5))
    for x in range(1, 25):
        game.units.append(Barbarian(game, x + 0.5, 0.5))

    """for x in range(25, 32):
        game.units.append(Barbarian(game, x + 0.5, 0.5))
    for y in range(1, 32):
        game.units.append(Barbarian(game, 31.5, y + 0.5))
    for x in range(18, 31):
        game.units.append(Barbarian(game, x + 0.5, 31.5))

    for x in range(7, 18):
        game.units.append(Barbarian(game, x + 0.5, 31.5))
    for y in range(19, 32):
        game.units.append(Barbarian(game, 0.5, y + 0.5))"""

    timer = SpinTimer(1 / FPS)

    def on_tick(delta_t: float | None):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                timer.stop()

                return

        game.draw()

        if not game.done and delta_t is not None:
            delta_t = min(delta_t, 2 * 1 / FPS)
            game.tick(delta_t)
            pygame.display.set_caption(f"cocsim | {game.progress_info()}")

        pygame.display.update()

    timer.on_tick = on_tick

    timer.run()


main()

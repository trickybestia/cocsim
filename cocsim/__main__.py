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

    bb1 = Barbarian(game, 1.0, 1.0)
    bb2 = Barbarian(game, 11.0, 1.0)

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
            pygame.display.set_caption(f"cocsim | {game.progress_info()}")

        pygame.display.update()

    timer.on_tick = on_tick

    timer.run()


main()

import pygame

from .spin_timer import SpinTimer
from .units.barbarian import Barbarian
from .consts import *
from .game import Game
from .buildings import TownHall


def main():
    pygame.init()

    screen = pygame.display.set_mode(
        (MAP_WIDTH * PIXELS_PER_TILE, MAP_HEIGHT * PIXELS_PER_TILE)
    )

    game = Game()

    th = TownHall(game, 10, 10, 0)

    game.screen = screen
    game.buildings = [th]

    game.compute_collision()
    game.compute_occupied_tiles()
    game.compute_drop_zone()

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

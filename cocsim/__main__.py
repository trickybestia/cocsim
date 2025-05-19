import pygame

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

    th = TownHall(game, 10, 10)

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

    while True:
        game.draw()

        pygame.display.update()

        input()

        game.tick(0.05)


main()

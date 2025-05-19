import pygame

from .utils import compute_drop_zone
from .consts import *
from .game import Game


def main():
    pygame.init()

    screen = pygame.display.set_mode(
        (MAP_WIDTH * PIXELS_PER_CELL, MAP_HEIGHT * PIXELS_PER_CELL)
    )

    game = Game()

    game.screen = screen
    game.buildings = [[False] * MAP_HEIGHT for _ in range(MAP_WIDTH)]
    game.buildings[10][10] = True
    game.drop_zone = compute_drop_zone(game.buildings)

    game.draw()

    pygame.display.update()

    input()


main()

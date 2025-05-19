import pygame

from .consts import *
from .game import Game
from .buildings import TownHall


def main():
    pygame.init()

    screen = pygame.display.set_mode(
        (MAP_WIDTH * PIXELS_PER_CELL, MAP_HEIGHT * PIXELS_PER_CELL)
    )

    game = Game()

    th = TownHall(game)
    th.x = 10
    th.y = 10

    game.screen = screen
    game.buildings = [th]

    game.compute_occupied_tiles()
    game.compute_drop_zone()

    game.draw()

    pygame.display.update()

    input()


main()

import pygame

BASE_WIDTH = 44
BASE_HEIGHT = 44
MAP_BORDERS = 3

MAP_WIDTH = BASE_WIDTH + MAP_BORDERS * 2
MAP_HEIGHT = BASE_HEIGHT + MAP_BORDERS * 2

PIXELS_PER_CELL = 20

CELL_COLOR = pygame.color.Color(40, 40, 40)
CELL_INVERTED_COLOR = pygame.color.Color(10, 10, 10)
BORDER_CELL_COLOR = pygame.color.Color(30, 60, 0)
BORDER_CELL_INVERTED_COLOR = pygame.color.Color(20, 30, 0)


def draw_grid(screen: pygame.Surface):
    for tile_x in range(MAP_WIDTH):
        for tile_y in range(MAP_HEIGHT):
            border_cell = (
                tile_y < MAP_BORDERS
                or tile_x < MAP_BORDERS
                or tile_y >= BASE_HEIGHT + MAP_BORDERS
                or tile_x >= BASE_WIDTH + MAP_BORDERS
            )
            invert_color = (tile_y ^ tile_x) & 1

            if border_cell:
                fill_color = (
                    BORDER_CELL_INVERTED_COLOR
                    if invert_color
                    else BORDER_CELL_COLOR
                )
            else:
                fill_color = CELL_INVERTED_COLOR if invert_color else CELL_COLOR

            pygame.draw.rect(
                screen,
                fill_color,
                (
                    tile_x * PIXELS_PER_CELL,
                    tile_y * PIXELS_PER_CELL,
                    PIXELS_PER_CELL,
                    PIXELS_PER_CELL,
                ),
            )

            invert_color = not invert_color


def main():
    pygame.init()

    screen = pygame.display.set_mode(
        (MAP_WIDTH * PIXELS_PER_CELL, MAP_HEIGHT * PIXELS_PER_CELL)
    )

    draw_grid(screen)

    pygame.display.update()

    input()


main()

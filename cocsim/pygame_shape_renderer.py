import pygame

from .consts import *
from .shapes import Shape


def render(surface: pygame.Surface, shapes: list[Shape]):
    for shape in shapes:
        match shape[0]:
            case "rect":
                pygame.draw.rect(
                    surface,
                    shape[5],
                    pygame.Rect(
                        int(shape[1] * PIXELS_PER_TILE),
                        int(shape[2] * PIXELS_PER_TILE),
                        int(shape[3] * PIXELS_PER_TILE),
                        int(shape[4] * PIXELS_PER_TILE),
                    ),
                )
            case "circle":
                pygame.draw.circle(
                    surface,
                    shape[4],
                    (
                        int(shape[1] * PIXELS_PER_TILE),
                        int(shape[2] * PIXELS_PER_TILE),
                    ),
                    int(shape[3] * PIXELS_PER_TILE),
                )
            case "line":
                pygame.draw.line(
                    surface,
                    shape[6],
                    (
                        int(shape[1] * PIXELS_PER_TILE),
                        int(shape[2] * PIXELS_PER_TILE),
                    ),
                    (
                        int(shape[3] * PIXELS_PER_TILE),
                        int(shape[4] * PIXELS_PER_TILE),
                    ),
                    int(shape[5] * PIXELS_PER_TILE),
                )

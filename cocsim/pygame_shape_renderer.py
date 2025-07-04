import PIL.Image
import pygame

from . import game
from .consts import *
from .shapes import Shape

BASE_IMAGE_LAYER_INDEX = 2
BASE_IMAGE_LAYER_ALPHA = 150


class PygameShapeRenderer:
    surface: pygame.Surface
    base_image: pygame.Surface | None
    background: pygame.Surface | None  # grid + base image
    collision: pygame.Surface | None

    def __init__(
        self, surface: pygame.Surface, base_image: pygame.Surface | None
    ):
        self.surface = surface
        self.base_image = base_image
        self.background = None
        self.collision = None

    def draw(self, game: "game.Game"):
        if self.background is None:
            self.background = pygame.Surface(self.surface.get_size())

            self._render_layer(self.background, game.draw_grid())

            if self.base_image is not None:
                self.background.blit(self.base_image, (0, 0))

        self.surface.blit(self.background, (0, 0))

        if self.collision is None or game.need_redraw_collision():
            self.collision = pygame.Surface(
                self.surface.get_size(), pygame.SRCALPHA, 32
            )
            self.collision.set_alpha(150)

            self._render_layer(self.collision, game.draw_collision())

        self.surface.blit(self.collision, (0, 0))

        self._render_layer(self.surface, game.draw_entities())

    @staticmethod
    def preprocess_base_image(
        image: PIL.Image.Image, total_size: int
    ) -> pygame.Surface:
        image = image.resize(
            (
                total_size * PIXELS_PER_TILE,
                total_size * PIXELS_PER_TILE,
            )
        )

        result = pygame.image.frombytes(image.tobytes(), image.size, image.mode)
        result.set_alpha(BASE_IMAGE_LAYER_ALPHA)

        return result

    @staticmethod
    def _render_layer(surface: pygame.Surface, shapes: list[Shape]):
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

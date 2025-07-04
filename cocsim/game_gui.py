from typing import Callable

import PIL.Image
import pygame

from cocsim.consts import *
from cocsim.game import Game
from cocsim.pygame_shape_renderer import PygameShapeRenderer
from cocsim.spin_timer import SpinTimer


class GameGui:
    before_tick: Callable[[], None] | None
    game: Game
    base_image: pygame.Surface | None

    def __init__(self, game: Game, image: PIL.Image.Image | None):
        self.before_tick = None
        self.game = game

        if image is not None:
            self.base_image = PygameShapeRenderer.preprocess_base_image(
                image, game.total_size
            )
        else:
            self.base_image = None

    def run(self):
        pygame.init()
        pygame.display.set_caption("cocsim")

        screen = pygame.display.set_mode(
            (
                self.game.total_size * PIXELS_PER_TILE,
                self.game.total_size * PIXELS_PER_TILE,
            )
        )
        renderer = PygameShapeRenderer(screen, self.base_image)

        timer = SpinTimer(1 / FPS)

        def on_tick(delta_t: float | None):
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    timer.stop()

                    return

            renderer.draw(self.game)

            if not self.game.done and delta_t is not None:
                if self.before_tick is not None:
                    self.before_tick()

                delta_t = min(delta_t, 2 * 1 / FPS)

                self.game.tick(delta_t)

                pygame.display.set_caption(
                    f"cocsim | {self.game.progress_info()}"
                )

            pygame.display.update()

        timer.on_tick = on_tick

        timer.run()

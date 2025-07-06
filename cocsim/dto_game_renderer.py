from typing import TypedDict

from . import game
from .shapes import Shape


class Frame(TypedDict):
    timeElapsed: float
    progressInfo: str
    totalBaseSize: int
    grid: list[Shape] | None  # None if not changed
    collision: list[Shape] | None  # None if not changed
    entities: list[Shape]


class DTOGameRenderer:
    result: list[Frame]
    ticks_per_draw: int

    _ticks_since_last_draw: int | None

    def __init__(self, ticks_per_draw: int):
        self.result = []
        self.ticks_per_draw = ticks_per_draw
        self._ticks_since_last_draw = None

    def draw(self, game: "game.Game"):
        if (
            self._ticks_since_last_draw is None
            or self._ticks_since_last_draw == self.ticks_per_draw
        ):
            self._ticks_since_last_draw = 0

            self._draw(game)

        self._ticks_since_last_draw += 1

    def finish(self, game: "game.Game"):
        if self.result[-1]["timeElapsed"] != game.time_elapsed:
            self._draw(game)

    def _draw(self, game: "game.Game"):
        frame: Frame = {
            "timeElapsed": game.time_elapsed,
            "progressInfo": game.progress_info(),
            "entities": game.draw_entities(),
            "totalBaseSize": game.total_size,
        }

        if len(self.result) == 0:
            frame["grid"] = game.draw_grid()
        else:
            frame["grid"] = None

        if game.need_redraw_collision():
            frame["collision"] = game.draw_collision()
        else:
            frame["collision"] = None

        self.result.append(frame)

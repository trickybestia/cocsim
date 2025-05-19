from .collider import Collider
from cocsim.consts import *


class RectCollider(Collider):
    x: float
    y: float
    width: float
    height: float

    def __init__(self, x: float, y: float, width: float, height: float):
        super().__init__()

        self.x = x
        self.y = y
        self.width = width
        self.height = height

    @classmethod
    def from_center(
        cls, center_x: float, center_y: float, width: float, height: float
    ) -> "RectCollider":
        return cls(center_x - width / 2, center_y - height / 2, width, height)

    def __contains__(self, item):
        x, y = (
            item[0] / COLLISION_TILES_PER_MAP_TILE,
            item[1] / COLLISION_TILES_PER_MAP_TILE,
        )

        return (
            self.x <= x <= self.x + self.width
            and self.y <= y <= self.y + self.height
        )

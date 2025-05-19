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

    def get_attack_area(self, attack_range: float) -> "RectCollider":
        return RectCollider(
            self.x - attack_range,
            self.y - attack_range,
            self.width + attack_range * 2,
            self.height + attack_range * 2,
        )

    def get_nearest_point(self, x: float, y: float) -> tuple[float, float]:
        if (x, y) in self:
            return x, y

        if self.x <= x <= self.x + self.width:
            if y <= self.y:
                return x, self.y
            else:
                return x, self.y + self.height
        elif self.y <= y <= self.y + self.height:
            if x <= self.x:
                return self.x, y
            else:
                return self.x + self.width, y
        elif x <= self.x:
            if y <= self.y:
                return self.x, self.y
            else:
                return self.x, self.y + self.height
        else:
            if y <= self.y:
                return self.x + self.width, self.y
            else:
                return self.x + self.width, self.y + self.height

    def __contains__(self, item):
        x, y = item

        return (
            self.x <= x <= self.x + self.width
            and self.y <= y <= self.y + self.height
        )

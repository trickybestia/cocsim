from typing import Literal, Type

from pydantic import BaseModel

from .. import game
from .building import BUILDINGS
from .colliders import ListCollider, RectCollider
from .passive_building import PassiveBuilding


class WallModel(BaseModel):
    name: Literal["Wall"]
    x: int
    y: int
    level: int


class Wall(PassiveBuilding):
    HEALTH = [
        300.0,
        500.0,
        700.0,
        900.0,
        1400.0,
        2000.0,
        2500.0,
        3000.0,
        3500.0,
        4000.0,
        5000.0,
        7000.0,
        9000.0,
        11000.0,
        12500.0,
        13500.0,
        14500.0,
        15500.0,
    ]

    @classmethod
    def width(cls) -> int:
        return 1

    @classmethod
    def height(cls) -> int:
        return 1

    @classmethod
    def levels(cls) -> int:
        return len(cls.HEALTH)

    @classmethod
    def model(cls) -> Type[WallModel]:
        return WallModel

    def __init__(self, game: "game.Game", x: int, y: int, level: int):
        super().__init__(
            game,
            x,
            y,
            self.HEALTH[level],
            RectCollider.from_center(
                x + self.width() / 2,
                y + self.height() / 2,
                self.width() * 0.65,
                self.height() * 0.65,
            ),
        )

    @classmethod
    def from_model(cls, game: "game.Game", model: WallModel) -> "Wall":
        return cls(game, model.x, model.y, model.level)

    def update_collision(self):
        self.collider = self._get_collider()

        super().update_collision()

    def _on_nearby_wall_destroyed(self, wall: "Wall"):
        self.update_collision()

    def _get_collider(self) -> ListCollider:
        """Gets collider for this wall with respect for connections to other walls."""
        result = ListCollider(
            [
                RectCollider.from_center(
                    self.x + self.width() / 2,
                    self.y + self.height() / 2,
                    self.width() * 0.65,
                    self.height() * 0.65,
                )
            ]
        )

        if (
            self.game.is_inside_map(self.x, self.y - 1)
            and isinstance(
                (up_wall := self.game.buildings_grid[self.x][self.y - 1]), Wall
            )
            and not up_wall.destroyed
        ):
            up_wall.on_destroyed.add(self._on_nearby_wall_destroyed)

            result.colliders.append(
                RectCollider(self.x + 0.175, self.y, 0.65, 0.5)
            )

        if (
            self.game.is_inside_map(self.x + 1, self.y)
            and isinstance(
                (right_wall := self.game.buildings_grid[self.x + 1][self.y]),
                Wall,
            )
            and not right_wall.destroyed
        ):
            right_wall.on_destroyed.add(self._on_nearby_wall_destroyed)

            result.colliders.append(
                RectCollider(self.x + 0.5, self.y + 0.175, 0.5, 0.65)
            )

        if (
            self.game.is_inside_map(self.x, self.y + 1)
            and isinstance(
                (down_wall := self.game.buildings_grid[self.x][self.y + 1]),
                Wall,
            )
            and not down_wall.destroyed
        ):
            down_wall.on_destroyed.add(self._on_nearby_wall_destroyed)

            result.colliders.append(
                RectCollider(self.x + 0.175, self.y + 0.5, 0.65, 0.5)
            )

        if (
            self.game.is_inside_map(self.x - 1, self.y)
            and isinstance(
                (left_wall := self.game.buildings_grid[self.x - 1][self.y]),
                Wall,
            )
            and not left_wall.destroyed
        ):
            left_wall.on_destroyed.add(self._on_nearby_wall_destroyed)

            result.colliders.append(
                RectCollider(self.x, self.y + 0.175, 0.5, 0.65)
            )

        return result


BUILDINGS.append(Wall)

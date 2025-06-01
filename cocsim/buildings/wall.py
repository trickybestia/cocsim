from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider, ListCollider


class Wall(PassiveBuilding):
    HEALTH = [
        300,
        500,
        700,
        900,
        1400,
        2000,
        2500,
        3000,
        3500,
        4000,
        5000,
        7000,
        9000,
        11000,
        12500,
        13500,
        14500,
        15500,
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

    def __init__(self, game: "game.Game", x: float, y: float, level: int):
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

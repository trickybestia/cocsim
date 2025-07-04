from dataclasses import dataclass
from enum import StrEnum
from typing import Literal, Type, Union

from pydantic import BaseModel

from .. import game, units
from ..shapes import *
from ..utils import distance, normalize
from .active_building import ActiveBuilding
from .building import BUILDINGS
from .colliders import RectCollider
from .option import Option


class AirSweeperRotation(StrEnum):
    Right = "right"
    RightUp = "right-up"
    Up = "up"
    LeftUp = "left-up"
    Left = "left"
    LeftDown = "left-down"
    Down = "down"
    RightDown = "right-down"


class AirSweeperModel(BaseModel):
    name: Literal["AirSweeper"]
    x: int
    y: int
    level: int
    rotation: AirSweeperRotation


@dataclass
class AirSweeperLevel:
    health: float
    push_strength: float


class AirSweeper(ActiveBuilding):
    MIN_ATTACK_DISTANCE = 1.0
    MAX_ATTACK_DISTANCE = 15.0
    ATTACK_COOLDOWN = 5.0

    LEVELS = (
        AirSweeperLevel(750.0, 1.6),
        AirSweeperLevel(800.0, 2.0),
        AirSweeperLevel(850.0, 2.4),
        AirSweeperLevel(900.0, 2.8),
        AirSweeperLevel(950.0, 3.2),
        AirSweeperLevel(1000.0, 3.6),
        AirSweeperLevel(1050.0, 4.0),
    )

    ROTATION_OPTION = Option(
        "rotation",
        [member.value for member in AirSweeperRotation],
    )

    target: Union["units.Unit", None]
    remaining_attack_cooldown: Union[float, None]

    level: int
    rotation: AirSweeperRotation

    @classmethod
    def width(cls) -> int:
        return 2

    @classmethod
    def height(cls) -> int:
        return 2

    @classmethod
    def levels(cls) -> int:
        return len(cls.LEVELS)

    @classmethod
    def options(cls) -> list[Option]:
        return super().options() + [cls.ROTATION_OPTION]

    @classmethod
    def model(cls) -> Type[AirSweeperModel]:
        return AirSweeperModel

    def __init__(
        self,
        game: "game.Game",
        x: int,
        y: int,
        level: int,
        rotation: AirSweeperRotation,
    ):
        super().__init__(
            game,
            x,
            y,
            self.LEVELS[level].health,
            RectCollider.from_center(
                x + self.width() / 2,
                y + self.height() / 2,
                self.width() * 0.65,
                self.height() * 0.65,
            ),
        )

        self.level = level
        self.rotation = rotation

        self.target = None
        self.remaining_attack_cooldown = None

    @classmethod
    def from_model(
        cls, game: "game.Game", model: AirSweeperModel
    ) -> "AirSweeper":
        return cls(game, model.x, model.y, model.level, model.rotation)

    def tick(self, delta_t: float):
        i = 0

        while i != len(self.projectiles):
            self.projectiles[i].tick(delta_t)

            if self.projectiles[i].movement_completed:
                del self.projectiles[i]
            else:
                i += 1

        if self.destroyed:
            self.target = None
            self.remaining_attack_cooldown = None

            return

        center_x, center_y = self.center

        if (
            self.target is None
            or self.target.dead
            or not (
                self.MIN_ATTACK_DISTANCE
                <= distance(center_x, center_y, self.target.x, self.target.y)
                <= self.MAX_ATTACK_DISTANCE
            )
        ):
            self.target = self._find_target()

            if self.target is None:
                self.remaining_attack_cooldown = None
            else:
                self.remaining_attack_cooldown = self.ATTACK_COOLDOWN

        if self.target is not None:
            self.remaining_attack_cooldown = max(
                0.0, self.remaining_attack_cooldown - delta_t
            )

            if self.remaining_attack_cooldown == 0.0:
                target_distance = distance(
                    self.center[0], self.center[1], self.target.x, self.target.y
                )

                self.projectiles.append(
                    self.projectile_type()(
                        self,
                        target_distance / self.projectile_speed(),
                        self.target,
                    )
                )
                self.remaining_attack_cooldown = self.ATTACK_COOLDOWN

    def draw(self, shapes: list[Shape]): ...

    def _find_target(self) -> Union["units.Unit", None]:
        center_x, center_y = self.center

        min_distance = None
        min_distance_target = None

        for unit in self.game.units:
            if unit.dead:
                continue

            if not isinstance(unit, units.AirUnit):
                continue

            current_distance = distance(center_x, center_y, unit.x, unit.y)

            if not (
                self.MIN_ATTACK_DISTANCE
                <= current_distance
                <= self.MAX_ATTACK_DISTANCE
            ):
                continue

            if min_distance is None or current_distance < min_distance:
                min_distance = current_distance
                min_distance_target = unit

        return min_distance_target


BUILDINGS.append(AirSweeper)

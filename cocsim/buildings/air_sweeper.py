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


BUILDINGS.append(AirSweeper)

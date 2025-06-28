from dataclasses import dataclass
from enum import StrEnum
from typing import Literal, Type

from pydantic import BaseModel

from .. import game, units
from .building import BUILDINGS
from .colliders import RectCollider
from .option import Option
from .projectile_active_building import ProjectileActiveBuilding


class XBowTargetType(StrEnum):
    Ground = "Ground"
    AirGround = "Air & Ground"


class XBowModel(BaseModel):
    name: Literal["XBow"]
    x: int
    y: int
    level: int
    target: XBowTargetType


@dataclass(frozen=True)
class XBowLevel:
    health: float
    attack_damage: float


class XBow(ProjectileActiveBuilding):
    LEVELS = (
        XBowLevel(1500.0, 7.68),
        XBowLevel(1900.0, 8.96),
        XBowLevel(2300.0, 10.24),
        XBowLevel(2700.0, 10.88),
        XBowLevel(3100.0, 12.16),
        XBowLevel(3400.0, 14.08),
        XBowLevel(3700.0, 16.64),
        XBowLevel(4000.0, 19.84),
        XBowLevel(4200.0, 23.68),
        XBowLevel(4400.0, 26.24),
        XBowLevel(4600.0, 28.8),
        XBowLevel(4800.0, 30.08),
    )

    TARGET_TYPE_OPTION = Option(
        "target", [member.value for member in XBowTargetType]
    )

    target_type_: Type["units.Unit"] | None
    level: int

    @classmethod
    def width(cls) -> int:
        return 3

    @classmethod
    def height(cls) -> int:
        return 3

    @classmethod
    def levels(cls) -> int:
        return len(cls.LEVELS)

    @classmethod
    def options(cls) -> list[Option]:
        return super().options() + [cls.TARGET_TYPE_OPTION]

    @classmethod
    def model(cls) -> Type[XBowModel]:
        return XBowModel

    @classmethod
    def attack_cooldown(cls) -> float:
        return 0.128

    def max_attack_distance(self) -> float:
        if self.target_type_ is None:
            return 11.5

        return 14.0

    def target_type(self) -> Type["units.Unit"] | None:
        return self.target_type_

    def attack_damage(self):
        return self.LEVELS[self.level].attack_damage

    def projectile_speed(self) -> float:
        if self.level == 0:
            return 23.0
        if self.level == 1:
            return 24.0

        return 25.0

    def __init__(
        self,
        game: "game.Game",
        x: int,
        y: int,
        level: int,
        target: XBowTargetType,
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

        match target:
            case XBowTargetType.Ground:
                self.target_type_ = units.GroundUnit
            case XBowTargetType.AirGround:
                self.target_type_ = None

    @classmethod
    def from_model(cls, game: "game.Game", model: XBowModel) -> "XBow":
        return cls(game, model.x, model.y, model.level, model.target)


BUILDINGS.append(XBow)

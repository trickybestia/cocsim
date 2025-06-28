from dataclasses import dataclass
from typing import Literal, Type

from pydantic import BaseModel

from .building import BUILDINGS
from .projectile_active_building import ProjectileActiveBuilding
from .. import game, units
from .colliders import RectCollider


class CannonModel(BaseModel):
    name: Literal["Cannon"]
    x: int
    y: int
    level: int


@dataclass(frozen=True)
class CannonLevel:
    health: float
    attack_damage: float


class Cannon(ProjectileActiveBuilding):
    LEVELS = (
        CannonLevel(420.0, 7.2),
        CannonLevel(470.0, 8.8),
        CannonLevel(520.0, 12.0),
        CannonLevel(570.0, 15.2),
        CannonLevel(620.0, 20.0),
        CannonLevel(670.0, 24.8),
        CannonLevel(730.0, 32.0),
        CannonLevel(800.0, 38.4),
        CannonLevel(880.0, 44.8),
        CannonLevel(960.0, 51.2),
        CannonLevel(1060.0, 59.2),
        CannonLevel(1160.0, 68.0),
        CannonLevel(1260.0, 76.0),
        CannonLevel(1380.0, 80.0),
        CannonLevel(1500.0, 84.0),
        CannonLevel(1620.0, 88.0),
        CannonLevel(1740.0, 92.0),
        CannonLevel(1870.0, 100.0),
        CannonLevel(2000.0, 108.0),
        CannonLevel(2150.0, 120.0),
        CannonLevel(2250.0, 128.0),
    )

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
    def model(cls) -> Type[CannonModel]:
        return CannonModel

    @classmethod
    def attack_cooldown(cls) -> float:
        return 0.8

    def max_attack_distance(self) -> float:
        return 9.0

    def target_type(self) -> Type["units.Unit"] | None:
        return units.GroundUnit

    def attack_damage(self):
        return self.LEVELS[self.level].attack_damage

    def projectile_speed(self) -> float:
        return 12.0

    def __init__(self, game: "game.Game", x: int, y: int, level: int):
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

    @classmethod
    def from_model(cls, game: "game.Game", model: CannonModel) -> "Cannon":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(Cannon)

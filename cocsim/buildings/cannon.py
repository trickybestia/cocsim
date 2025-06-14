from dataclasses import dataclass
from typing import Type

from .building import BUILDINGS
from .projectile_active_building import ProjectileActiveBuilding
from .. import game, units
from .colliders import RectCollider
from cocsim.consts import *


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
    def attack_range(cls) -> float:
        return 9.0

    @classmethod
    def attack_cooldown(cls) -> float:
        return 0.8

    @classmethod
    def projectile_speed(cls):
        return 15.5

    @classmethod
    def target_type(cls) -> Type["units.Unit"] | None:
        return units.GroundUnit

    def attack_damage(self):
        return self.LEVELS[self.level].attack_damage

    def __init__(self, game: "game.Game", x: float, y: float, level: int):
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


BUILDINGS.append(Cannon)

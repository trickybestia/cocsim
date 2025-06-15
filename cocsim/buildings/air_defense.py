from dataclasses import dataclass
from typing import Type

from ..utils import compute_projectile_speed
from .building import BUILDINGS
from .projectile_active_building import (
    ProjectileActiveBuilding,
    Projectile,
    TargetProjectile,
)
from .. import game, units
from .colliders import RectCollider


@dataclass(frozen=True)
class AirDefenseLevel:
    health: float
    attack_damage: float


class AirDefense(ProjectileActiveBuilding):
    LEVELS = (
        AirDefenseLevel(800.0, 80.0),
        AirDefenseLevel(850.0, 110.0),
        AirDefenseLevel(900.0, 140.0),
        AirDefenseLevel(950.0, 160.0),
        AirDefenseLevel(1000.0, 190.0),
        AirDefenseLevel(1050.0, 230.0),
        AirDefenseLevel(1100.0, 280.0),
        AirDefenseLevel(1210.0, 320.0),
        AirDefenseLevel(1300.0, 360.0),
        AirDefenseLevel(1400.0, 400.0),
        AirDefenseLevel(1500.0, 440.0),
        AirDefenseLevel(1650.0, 500.0),
        AirDefenseLevel(1750.0, 540.0),
        AirDefenseLevel(1850.0, 600.0),
        AirDefenseLevel(1950.0, 650.0),
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
        return 10.0

    @classmethod
    def attack_cooldown(cls) -> float:
        return 1.0

    @classmethod
    def projectile_speed(cls) -> float:
        return compute_projectile_speed(
            30.837,
            31.536,
            cls.attack_range(AirDefense),
            cls.attack_cooldown(),
            2.5,  # checked with lava hound
        )

    @classmethod
    def target_type(cls) -> Type["units.Unit"] | None:
        return units.AirUnit

    @classmethod
    def projectile_type(cls) -> Type[Projectile]:
        return TargetProjectile

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


BUILDINGS.append(AirDefense)

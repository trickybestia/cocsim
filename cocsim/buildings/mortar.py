from dataclasses import dataclass
from typing import Type

from ..utils import compute_projectile_speed
from .building import BUILDINGS
from .projectile_active_building import (
    ProjectileActiveBuilding,
    Projectile,
    SplashProjectile,
)
from .. import game, units
from .colliders import RectCollider


@dataclass(frozen=True)
class MortarLevel:
    health: float
    attack_damage: float


class Mortar(ProjectileActiveBuilding):
    LEVELS = (
        MortarLevel(400.0, 20.0),
        MortarLevel(450.0, 25.0),
        MortarLevel(500.0, 40.0),
        MortarLevel(550.0, 35.0),
        MortarLevel(600.0, 45.0),
        MortarLevel(650.0, 55.0),
        MortarLevel(700.0, 75.0),
        MortarLevel(800.0, 100.0),
        MortarLevel(950.0, 125.0),
        MortarLevel(1100.0, 150.0),
        MortarLevel(1300.0, 175.0),
        MortarLevel(1500.0, 190.0),
        MortarLevel(1700.0, 210.0),
        MortarLevel(1950.0, 240.0),
        MortarLevel(2150.0, 270.0),
        MortarLevel(2300.0, 300.0),
        MortarLevel(2450.0, 330.0),
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
    def min_attack_distance(cls):
        return 4.0

    @classmethod
    def max_attack_distance(cls):
        return 11.0

    @classmethod
    def attack_cooldown(cls) -> float:
        return 5.0

    @classmethod
    def projectile_speed(cls) -> float:
        return compute_projectile_speed(
            16.839,
            18.614,
            cls.max_attack_distance(),
            cls.attack_cooldown(),
            1.5,  # checked with giant
        )

    @classmethod
    def target_type(cls) -> Type["units.Unit"] | None:
        return units.GroundUnit

    @classmethod
    def projectile_type(cls) -> Type[Projectile]:
        return SplashProjectile

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


BUILDINGS.append(Mortar)

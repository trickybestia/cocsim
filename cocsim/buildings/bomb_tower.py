from dataclasses import dataclass
from typing import Type

from .building import BUILDINGS
from .splash_projectile_active_building import SplashProjectileActiveBuilding
from .. import game, units
from .colliders import RectCollider


@dataclass(frozen=True)
class BombTowerLevel:
    health: float
    attack_damage: float
    death_damage: float


class BombTower(SplashProjectileActiveBuilding):
    DEATH_DAMAGE_RADIUS = 2.75

    LEVELS = (
        BombTowerLevel(650.0, 26.4, 150.0),
        BombTowerLevel(700.0, 30.8, 180.0),
        BombTowerLevel(750.0, 35.2, 220.0),
        BombTowerLevel(850.0, 44.0, 260.0),
        BombTowerLevel(1050.0, 52.8, 300.0),
        BombTowerLevel(1300.0, 61.6, 350.0),
        BombTowerLevel(1600.0, 70.4, 400),
        BombTowerLevel(1900.0, 79.2, 450.0),
        BombTowerLevel(2300.0, 92.4, 500.0),
        BombTowerLevel(2500.0, 103.4, 550.0),
        BombTowerLevel(2700.0, 114.4, 600.0),
        BombTowerLevel(2900.0, 125.4, 650.0),
    )

    level: int
    death_damage_cooldown: float
    death_damage_apllied: bool

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
    def attack_cooldown(cls) -> float:
        return 1.1

    @classmethod
    def splash_attack_radius(cls):
        return 1.5

    def max_attack_distance(self):
        return 6.0

    def target_type(self) -> Type["units.Unit"] | None:
        return units.GroundUnit

    def attack_damage(self):
        return self.LEVELS[self.level].attack_damage

    def projectile_speed(self) -> float:
        return 8.0

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
        self.death_damage_cooldown = 1.0
        self.death_damage_apllied = False

    def tick(self, delta_t):
        super().tick(delta_t)

        if self.destroyed and not self.death_damage_apllied:
            self.death_damage_cooldown = max(
                0.0, self.death_damage_cooldown - delta_t
            )

            if self.death_damage_cooldown == 0.0:
                self.death_damage_apllied = True

                self.splash_attack(
                    self.center[0],
                    self.center[1],
                    self.DEATH_DAMAGE_RADIUS,
                    self.LEVELS[self.level].death_damage,
                )


BUILDINGS.append(BombTower)

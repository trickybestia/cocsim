from dataclasses import dataclass
from typing import Type

from ..utils import compute_projectile_speed
from .building import BUILDINGS
from .splash_projectile_active_building import SplashProjectileActiveBuilding
from .. import game, units
from .colliders import RectCollider


@dataclass(frozen=True)
class WizardTowerLevel:
    health: float
    attack_damage: float


class WizardTower(SplashProjectileActiveBuilding):
    LEVELS = (
        WizardTowerLevel(620.0, 14.3),
        WizardTowerLevel(650.0, 16.9),
        WizardTowerLevel(680.0, 20.8),
        WizardTowerLevel(730.0, 26.0),
        WizardTowerLevel(840.0, 31.2),
        WizardTowerLevel(960.0, 41.6),
        WizardTowerLevel(1200.0, 52.0),
        WizardTowerLevel(1440.0, 58.5),
        WizardTowerLevel(1600.0, 65.0),
        WizardTowerLevel(1900.0, 80.6),
        WizardTowerLevel(2120.0, 91.0),
        WizardTowerLevel(2240.0, 101.4),
        WizardTowerLevel(2500.0, 109.2),
        WizardTowerLevel(2800.0, 117.0),
        WizardTowerLevel(3000.0, 123.5),
        WizardTowerLevel(3150.0, 132.6),
        WizardTowerLevel(3300.0, 143.0),
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
    def max_attack_distance(cls):
        return 7.0

    @classmethod
    def attack_cooldown(cls) -> float:
        return 1.3

    @classmethod
    def projectile_speed(cls) -> float:
        return compute_projectile_speed(
            28.431,
            29.578,
            cls.max_attack_distance(),
            cls.attack_cooldown(),
            1.5,  # checked with giant
        )

    @classmethod
    def target_type(cls) -> Type["units.Unit"] | None:
        return None

    @classmethod
    def splash_attack_radius(cls):
        return 1.0

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


BUILDINGS.append(WizardTower)

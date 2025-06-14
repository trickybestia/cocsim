from dataclasses import dataclass
from typing import Type

from .building import BUILDINGS
from .projectile_active_building import ProjectileActiveBuilding
from .. import game, units
from .colliders import RectCollider


@dataclass(frozen=True)
class ArcherTowerLevel:
    health: float
    attack_damage: float


class ArcherTower(ProjectileActiveBuilding):
    LEVELS = (
        ArcherTowerLevel(380.0, 5.5),
        ArcherTowerLevel(420.0, 7.5),
        ArcherTowerLevel(460.0, 9.5),
        ArcherTowerLevel(500.0, 12.5),
        ArcherTowerLevel(540.0, 15.0),
        ArcherTowerLevel(580.0, 17.5),
        ArcherTowerLevel(630.0, 21.0),
        ArcherTowerLevel(690.0, 24.0),
        ArcherTowerLevel(750.0, 28.0),
        ArcherTowerLevel(810.0, 31.5),
        ArcherTowerLevel(890.0, 35.0),
        ArcherTowerLevel(970.0, 37.0),
        ArcherTowerLevel(1050.0, 39.0),
        ArcherTowerLevel(1130.0, 41.0),
        ArcherTowerLevel(1230.0, 42.5),
        ArcherTowerLevel(1310.0, 45.0),
        ArcherTowerLevel(1390.0, 50.0),
        ArcherTowerLevel(1510.0, 55.0),
        ArcherTowerLevel(1600.0, 60.0),
        ArcherTowerLevel(1700.0, 67.5),
        ArcherTowerLevel(1800.0, 72.5),
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
        return 0.5

    @classmethod
    def projectile_speed(cls) -> float:
        return 25.0

    @classmethod
    def target_type(cls) -> Type["units.Unit"] | None:
        return None

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


BUILDINGS.append(ArcherTower)

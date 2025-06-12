from dataclasses import dataclass
from typing import Union

import pygame.draw

from ..utils import distance
from .building import BUILDINGS
from .active_building import ActiveBuilding
from .. import game, units
from .colliders import RectCollider
from cocsim.consts import *


@dataclass(frozen=True)
class CannonLevel:
    health: float
    attack_damage: float


@dataclass
class CannonDelayedDamage:
    time_left: float
    target: "units.Unit"


class Cannon(ActiveBuilding):
    ATTACK_COOLDOWN = 0.8
    ATTACK_RANGE = 9.0
    CANNONBALL_SPEED = 13.3

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

    target: Union["units.Unit", None]
    attack_cooldown: Union[float, None]
    delayed_damage: list[CannonDelayedDamage]  # cannonball simulation

    @classmethod
    def width(cls) -> int:
        return 3

    @classmethod
    def height(cls) -> int:
        return 3

    @classmethod
    def levels(cls) -> int:
        return len(cls.LEVELS)

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

        self.target = None
        self.attack_cooldown = None
        self.delayed_damage = []

    def tick(self, delta_t: float):
        i = 0

        while i != len(self.delayed_damage):
            self.delayed_damage[i].time_left = max(
                0.0, self.delayed_damage[i].time_left - delta_t
            )

            if self.delayed_damage[i].time_left == 0.0:
                target = self.delayed_damage[i].target

                if not target.dead:
                    target.apply_damage(self.LEVELS[self.level].attack_damage)

                del self.delayed_damage[i]
            else:
                i += 1

        if self.destroyed:
            self.target = None
            self.attack_cooldown = None

            return

        center_x, center_y = self.center

        if (
            self.target is None
            or self.target.dead
            or distance(center_x, center_y, self.target.x, self.target.y)
            > self.ATTACK_RANGE
        ):
            self.target = self._find_target()

            if self.target is None:
                self.attack_cooldown = None
            else:
                self.attack_cooldown = self.ATTACK_COOLDOWN

        if self.target is not None:
            self.attack_cooldown = max(0.0, self.attack_cooldown - delta_t)

            if self.attack_cooldown == 0.0:
                target_distance = distance(
                    self.center[0], self.center[1], self.target.x, self.target.y
                )
                self.delayed_damage.append(
                    CannonDelayedDamage(
                        target_distance / self.CANNONBALL_SPEED, self.target
                    )
                )
                self.attack_cooldown = self.ATTACK_COOLDOWN

    def draw(self):
        if not self.destroyed and self.target is not None:
            pygame.draw.line(
                self.game.screen,
                pygame.Color(255, 0, 0),
                (
                    self.center[0] * PIXELS_PER_TILE,
                    self.center[1] * PIXELS_PER_TILE,
                ),
                (
                    self.target.x * PIXELS_PER_TILE,
                    self.target.y * PIXELS_PER_TILE,
                ),
            )

    def _find_target(self) -> Union["units.Unit", None]:
        center_x, center_y = self.center

        min_distance = None
        min_distance_target = None

        for unit in self.game.units:
            if unit.dead or not isinstance(unit, units.GroundUnit):
                continue

            current_distance = distance(center_x, center_y, unit.x, unit.y)

            if current_distance > self.ATTACK_RANGE:
                continue

            if min_distance is None or current_distance < min_distance:
                min_distance = current_distance
                min_distance_target = unit

        return min_distance_target


BUILDINGS.append(Cannon)

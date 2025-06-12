from collections import namedtuple
from typing import Union

import pygame

from .air_unit import AirUnit
from .. import game, buildings

from cocsim.utils import distance, normalize
from cocsim.consts import *

DragonLevel = namedtuple("DragonLevel", ("health", "attack_damage"))


class Dragon(AirUnit):
    ATTACK_COOLDOWN = 1.25
    SPEED = 2.0

    LEVELS = (
        DragonLevel(1900.0, 175.0),
        DragonLevel(2100.0, 200.0),
        DragonLevel(2300.0, 225.0),
        DragonLevel(2700.0, 262.5),
        DragonLevel(3100.0, 300.0),
        DragonLevel(3400.0, 337.5),
        DragonLevel(3900.0, 387.5),
        DragonLevel(4200.0, 412.5),
        DragonLevel(4500.0, 437.5),
        DragonLevel(4900.0, 462.5),
        DragonLevel(5300.0, 487.5),
        DragonLevel(5700.0, 512.5),
    )

    level: int

    waypoints: Union[list[tuple[float, float]], None]
    attack_cooldown: Union[float, None]

    @classmethod
    def levels(cls):
        return len(cls.LEVELS)

    @property
    def attack_range(self) -> float:
        return 1.0

    def __init__(self, game: "game.Game", level: int, x: float, y: float):
        super().__init__(game, x, y, self.LEVELS[level].health)

        self.level = level

        self.waypoints = None
        self.attack_cooldown = None

    def draw(self):
        if not self.dead:
            pygame.draw.circle(
                self.game.screen,
                pygame.Color(255, 0, 0),
                (self.x * PIXELS_PER_TILE, self.y * PIXELS_PER_TILE),
                5,
            )

    def tick(self, delta_t: float):
        if self.dead:
            self.target = None
            self.waypoints = None
            self.attack_cooldown = None

        if self.target is None or self.target.destroyed:
            self.attack_cooldown = None

            self.target, self.waypoints = (
                self.game.pathfinder.find_best_air_path(self, None)
            )

        if self.target is not None:
            if (
                distance(
                    self.x, self.y, self.waypoints[0][0], self.waypoints[0][1]
                )
                <= DISTANCE_TO_WAYPOINT_EPS
            ):
                if len(self.waypoints) == 1:
                    if self.attack_cooldown is None:
                        self.attack_cooldown = self.ATTACK_COOLDOWN
                    elif self.attack_cooldown == 0.0:
                        self.attack_cooldown = self.ATTACK_COOLDOWN

                        self._attack(self.target)
                    else:
                        self.attack_cooldown = max(
                            0, self.attack_cooldown - delta_t
                        )
                else:
                    self.waypoints.pop(0)

                    self._move(delta_t)
            else:
                self._move(delta_t)

    def _attack(self, target: "buildings.Building"):
        target.apply_damage(self.LEVELS[self.level].attack_damage)

    def _move(self, delta_t: float):
        if (
            distance(self.x, self.y, self.waypoints[0][0], self.waypoints[0][1])
            <= DISTANCE_TO_WAYPOINT_EPS
        ):
            return

        direction_x = self.waypoints[0][0] - self.x
        direction_y = self.waypoints[0][1] - self.y

        direction_x, direction_y = normalize(direction_x, direction_y)

        self.x += direction_x * self.SPEED * delta_t
        self.y += direction_y * self.SPEED * delta_t

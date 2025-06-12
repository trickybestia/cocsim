from collections import namedtuple
from typing import Union

import pygame

from .air_unit import AirUnit
from .. import game, buildings

from cocsim.utils import distance, normalize
from cocsim.consts import *

BalloonLevel = namedtuple(
    "BalloonLevel", ("health", "attack_damage", "death_damage")
)


class Balloon(AirUnit):
    ATTACK_COOLDOWN = 3.0
    SPEED = 1.3

    LEVELS = (
        BalloonLevel(150, 75, 25),
        BalloonLevel(180, 96, 32),
        BalloonLevel(216, 144, 48),
        BalloonLevel(280, 216, 72),
        BalloonLevel(390, 324, 108),
        BalloonLevel(545, 486, 162),
        BalloonLevel(690, 594, 214),
        BalloonLevel(840, 708, 268),
        BalloonLevel(940, 768, 322),
        BalloonLevel(1040, 828, 352),
        BalloonLevel(1140, 870, 375),
    )

    level: int

    waypoints: Union[list[tuple[float, float]], None]
    attack_cooldown: Union[float, None]

    @classmethod
    def levels(cls):
        return len(cls.LEVELS)

    @property
    def attack_range(self) -> float:
        return -0.5

    def __init__(self, game: "game.Game", level: int, x: float, y: float):
        super().__init__(game, x, y)

        self.level = level

        self.waypoints = None
        self.attack_cooldown = None

    def draw(self):
        if not self.dead:
            pygame.draw.circle(
                self.game.screen,
                pygame.Color(0, 0, 0),
                (self.x * PIXELS_PER_TILE, self.y * PIXELS_PER_TILE),
                5,
            )

    def tick(self, delta_t: float):
        if self.target is None or self.target.destroyed:
            self.target = None
            self.waypoints = None
            self.attack_cooldown = None

            self.target, self.waypoints = (
                self.game.pathfinder.find_best_air_path(
                    self, buildings.ActiveBuilding
                )
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

from typing import Union
import pygame

from .unit import Unit
from .. import game, buildings

from cocsim.utils import distance, normalize
from cocsim.consts import *


class Barbarian(Unit):
    ATTACK_DAMAGE = 8
    ATTACK_COOLDOWN = 1.0
    SPEED = 2.0

    waypoints: Union[list[tuple[float, float]], None]
    attack_cooldown: Union[float, None]

    @property
    def attack_range(self) -> float:
        return 0.4

    def __init__(self, game: "game.Game", x: float, y: float):
        super().__init__(game, x, y)

        self.waypoints = None
        self.attack_cooldown = None

    def draw(self):
        if not self.dead:
            pygame.draw.circle(
                self.game.screen,
                pygame.Color(255, 255, 255),
                (self.x * PIXELS_PER_TILE, self.y * PIXELS_PER_TILE),
                5,
            )

    def tick(self, delta_t: float):
        if self.target is None or self.target.destroyed:
            self.target = None
            self.waypoints = None
            self.attack_cooldown = None

            self.target, self.waypoints = (
                self.game.pathfinder.find_best_ground_path(self, None)
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
        target.apply_damage(self.ATTACK_DAMAGE)

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

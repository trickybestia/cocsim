from dataclasses import dataclass
from typing import Union

from cocsim.consts import *
from cocsim.shapes import *
from cocsim.utils import distance, normalize

from .. import buildings, game
from .ground_unit import GroundUnit


@dataclass(frozen=True)
class BarbarianLevel:
    health: float
    attack_damage: float


class Barbarian(GroundUnit):
    ATTACK_COOLDOWN = 1.0
    SPEED = 2.0

    LEVELS = (
        BarbarianLevel(45.0, 8.0),
        BarbarianLevel(54.0, 11.0),
        BarbarianLevel(65.0, 14.0),
        BarbarianLevel(85.0, 18.0),
        BarbarianLevel(105.0, 23.0),
        BarbarianLevel(125.0, 26.0),
        BarbarianLevel(160.0, 30.0),
        BarbarianLevel(205.0, 34.0),
        BarbarianLevel(230.0, 38.0),
        BarbarianLevel(250.0, 42.0),
        BarbarianLevel(270.0, 45.0),
        BarbarianLevel(290.0, 48.0),
    )

    level: int

    waypoints: Union[list[tuple[float, float]], None]
    attack_cooldown: Union[float, None]

    @classmethod
    def levels(cls):
        return len(cls.LEVELS)

    @classmethod
    def housing_space(cls):
        return 1

    @property
    def attack_range(self) -> float:
        return 0.4

    def __init__(self, game: "game.Game", level: int, x: float, y: float):
        super().__init__(game, x, y, self.LEVELS[level].health)

        self.level = level

        self.waypoints = None
        self.attack_cooldown = None

    def draw(self, shapes: list[Shape]):
        if not self.dead:
            shapes.append(circle(self.x, self.y, 0.25, "white"))

    def tick(self, delta_t: float):
        if self.dead:
            self.target = None
            self.waypoints = None
            self.attack_cooldown = None

            return

        if self.target is None or self.target.destroyed:
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

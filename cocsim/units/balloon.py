from dataclasses import dataclass
from typing import Union

from cocsim.consts import *
from cocsim.shapes import *
from cocsim.utils import distance, normalize

from .. import buildings, game
from .air_unit import AirUnit


@dataclass(frozen=True)
class BalloonLevel:
    health: float
    attack_damage: float
    death_damage: float


class Balloon(AirUnit):
    ATTACK_COOLDOWN = 3.0
    SPEED = 1.3

    LEVELS = (
        BalloonLevel(150.0, 75.0, 25.0),
        BalloonLevel(180.0, 96.0, 32.0),
        BalloonLevel(216.0, 144.0, 48.0),
        BalloonLevel(280.0, 216.0, 72.0),
        BalloonLevel(390.0, 324.0, 108.0),
        BalloonLevel(545.0, 486.0, 162.0),
        BalloonLevel(690.0, 594.0, 214.0),
        BalloonLevel(840.0, 708.0, 268.0),
        BalloonLevel(940.0, 768.0, 322.0),
        BalloonLevel(1040.0, 828.0, 352.0),
        BalloonLevel(1140.0, 870.0, 375.0),
    )

    level: int

    waypoints: Union[list[tuple[float, float]], None]
    attack_cooldown: Union[float, None]

    @classmethod
    def levels(cls):
        return len(cls.LEVELS)

    @classmethod
    def housing_space(cls):
        return 5

    @property
    def attack_range(self) -> float:
        return -0.5

    def __init__(self, game: "game.Game", level: int, x: float, y: float):
        super().__init__(game, x, y, self.LEVELS[level].health)

        self.level = level

        self.waypoints = None
        self.attack_cooldown = None

    def draw(self, shapes: list[Shape]):
        if not self.dead:
            shapes.append(circle(self.x, self.y, 0.25, "black"))

    def tick(self, delta_t: float):
        if self.dead:
            self.target = None
            self.waypoints = None
            self.attack_cooldown = None

        if self.target is None or self.target.destroyed:
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

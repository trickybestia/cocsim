from dataclasses import dataclass
from typing import Literal, Type, Union

from pydantic import BaseModel

from cocsim.consts import *
from cocsim.shapes import *
from cocsim.utils import distance, normalize

from .. import buildings, game
from .air_unit import AirUnit
from .unit import UNITS


class DragonModel(BaseModel):
    name: Literal["Dragon"]
    level: int


@dataclass(frozen=True)
class DragonLevel:
    health: float
    attack_damage: float


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

    @classmethod
    def model(cls) -> Type[DragonModel]:
        return DragonModel

    @classmethod
    def housing_space(cls):
        return 20

    @property
    def attack_range(self) -> float:
        return 1.0

    def __init__(self, game: "game.Game", level: int, x: float, y: float):
        super().__init__(game, x, y, self.LEVELS[level].health)

        self.level = level

        self.waypoints = None
        self.attack_cooldown = None

    @classmethod
    def from_model(
        cls, game: "game.Game", model: DragonModel, x: float, y: float
    ) -> "Dragon":
        return cls(game, model.level, x, y)

    def draw(self, shapes: list[Shape]):
        if not self.dead:
            shapes.append(circle(self.x, self.y, 0.25, "red"))

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


UNITS.append(Dragon)

from dataclasses import dataclass
from typing import Type, Union

import pygame

from .active_building import ActiveBuilding
from .. import game, units
from .colliders import Collider
from ..utils import distance, normalize
from cocsim.consts import *


@dataclass
class Projectile:
    time_left: float
    target: "units.Unit"
    rel_position: tuple[float, float]  # position relative to target
    rel_speed: tuple[float, float]


class ProjectileActiveBuilding(ActiveBuilding):
    target: Union["units.Unit", None]
    remaining_attack_cooldown: Union[float, None]
    projectiles: list[Projectile]

    def __init__(
        self,
        game: "game.Game",
        x: int,
        y: int,
        health: float,
        collider: Collider,
    ):
        super().__init__(game, x, y, health, collider)

        self.target = None
        self.remaining_attack_cooldown = None
        self.projectiles = []

    @classmethod
    def projectile_speed(cls) -> float: ...

    @classmethod
    def attack_cooldown(cls) -> float: ...

    @classmethod
    def attack_range(cls) -> float: ...

    @classmethod
    def target_type(cls) -> Type["units.Unit"] | None: ...

    def attack_damage(self) -> float: ...

    def tick(self, delta_t: float):
        i = 0

        while i != len(self.projectiles):
            projectile = self.projectiles[i]

            projectile.time_left = max(0.0, projectile.time_left - delta_t)
            projectile.rel_position = (
                projectile.rel_position[0] + projectile.rel_speed[0] * delta_t,
                projectile.rel_position[1] + projectile.rel_speed[1] * delta_t,
            )

            if projectile.time_left == 0.0:
                target = projectile.target

                if not target.dead:
                    target.apply_damage(self.attack_damage())

                del self.projectiles[i]
            else:
                i += 1

        if self.destroyed:
            self.target = None
            self.remaining_attack_cooldown = None

            return

        center_x, center_y = self.center

        if (
            self.target is None
            or self.target.dead
            or distance(center_x, center_y, self.target.x, self.target.y)
            > self.attack_range()
        ):
            self.target = self._find_target()

            if self.target is None:
                self.remaining_attack_cooldown = None
            else:
                self.remaining_attack_cooldown = self.attack_cooldown()

        if self.target is not None:
            self.remaining_attack_cooldown = max(
                0.0, self.remaining_attack_cooldown - delta_t
            )

            if self.remaining_attack_cooldown == 0.0:
                target_distance = distance(
                    self.center[0], self.center[1], self.target.x, self.target.y
                )
                speed_normalized = normalize(
                    self.target.x - self.center[0],
                    self.target.y - self.center[1],
                )
                self.projectiles.append(
                    Projectile(
                        target_distance / self.projectile_speed(),
                        self.target,
                        (
                            self.center[0] - self.target.x,
                            self.center[1] - self.target.y,
                        ),
                        (
                            speed_normalized[0] * self.projectile_speed(),
                            speed_normalized[1] * self.projectile_speed(),
                        ),
                    )
                )
                self.remaining_attack_cooldown = self.attack_cooldown()

    def draw(self):
        if not self.destroyed and self.target is not None:
            for projectile in self.projectiles:
                pygame.draw.circle(
                    self.game.screen,
                    pygame.Color(255, 0, 0),
                    (
                        (projectile.target.x + projectile.rel_position[0])
                        * PIXELS_PER_TILE,
                        (projectile.target.y + projectile.rel_position[1])
                        * PIXELS_PER_TILE,
                    ),
                    2,
                )

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
            if unit.dead:
                continue

            if self.target_type() is not None and not isinstance(
                unit, self.target_type()
            ):
                continue

            current_distance = distance(center_x, center_y, unit.x, unit.y)

            if current_distance > self.attack_range():
                continue

            if min_distance is None or current_distance < min_distance:
                min_distance = current_distance
                min_distance_target = unit

        return min_distance_target

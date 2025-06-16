from typing import Type, Union

import pygame

from .active_building import ActiveBuilding
from .. import game, units
from .colliders import Collider
from ..utils import distance, normalize
from cocsim.consts import *


class Projectile:
    building: "ProjectileActiveBuilding"
    time_left: float

    movement_completed: bool

    def __init__(self, building: "ProjectileActiveBuilding", time_left: float):
        self.building = building
        self.time_left = time_left

        self.movement_completed = False

    def tick(self, delta_t: float):
        assert not self.movement_completed

        self.time_left = max(0.0, self.time_left - delta_t)


class TargetProjectile(Projectile):
    target: "units.Unit"
    rel_position: tuple[float, float]  # position relative to target
    rel_speed: tuple[float, float]

    def __init__(
        self,
        building: "ProjectileActiveBuilding",
        time_left: float,
        target: "units.Unit",
    ):
        super().__init__(building, time_left)

        self.target = target

        speed_normalized = normalize(
            self.target.x - building.center[0],
            self.target.y - building.center[1],
        )

        self.rel_position = (
            building.center[0] - self.target.x,
            building.center[1] - self.target.y,
        )
        self.rel_speed = (
            speed_normalized[0] * building.projectile_speed(),
            speed_normalized[1] * building.projectile_speed(),
        )

    def tick(self, delta_t: float):
        super().tick(delta_t)

        self.rel_position = (
            self.rel_position[0] + self.rel_speed[0] * delta_t,
            self.rel_position[1] + self.rel_speed[1] * delta_t,
        )

        if self.time_left == 0.0:
            self.movement_completed = True

            if not self.target.dead:
                self.target.apply_damage(self.building.attack_damage())


class SplashProjectile(Projectile):
    SPLASH_ATTACK_RADIUS = 1.5  # now hardcoded for Mortar

    target: tuple[float, float]
    position: tuple[float, float]
    speed: tuple[float, float]

    def __init__(
        self,
        building: "ProjectileActiveBuilding",
        time_left: float,
        target: "units.Unit",
    ):
        super().__init__(building, time_left)

        self.target = (target.x, target.y)

        speed_normalized = normalize(
            target.x - building.center[0],
            target.y - building.center[1],
        )

        self.position = building.center
        self.speed = (
            speed_normalized[0] * building.projectile_speed(),
            speed_normalized[1] * building.projectile_speed(),
        )

    def tick(self, delta_t: float):
        super().tick(delta_t)

        self.position = (
            self.position[0] + self.speed[0] * delta_t,
            self.position[1] + self.speed[1] * delta_t,
        )

        if self.time_left == 0.0:
            self.movement_completed = True

            self.building.splash_attack(
                self.target[0],
                self.target[1],
                self.SPLASH_ATTACK_RADIUS,
                self.building.attack_damage(),
            )


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
    def min_attack_distance(cls) -> float:
        return 0.0

    @classmethod
    def max_attack_distance(cls) -> float: ...

    @classmethod
    def target_type(cls) -> Type["units.Unit"] | None: ...

    @classmethod
    def projectile_type(cls) -> Type[Projectile]: ...

    def attack_damage(self) -> float: ...

    def tick(self, delta_t: float):
        i = 0

        while i != len(self.projectiles):
            self.projectiles[i].tick(delta_t)

            if self.projectiles[i].movement_completed:
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
            or not (
                self.min_attack_distance()
                <= distance(center_x, center_y, self.target.x, self.target.y)
                <= self.max_attack_distance()
            )
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

                self.projectiles.append(
                    self.projectile_type()(
                        self,
                        target_distance / self.projectile_speed(),
                        self.target,
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

            if not (
                self.min_attack_distance()
                <= current_distance
                <= self.max_attack_distance()
            ):
                continue

            if min_distance is None or current_distance < min_distance:
                min_distance = current_distance
                min_distance_target = unit

        return min_distance_target

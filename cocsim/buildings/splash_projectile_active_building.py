from typing import Type

import pygame

from ..utils import compute_projectile_speed, normalize
from .projectile_active_building import ProjectileActiveBuilding, Projectile
from .. import game, units
from .colliders import Collider
from cocsim.consts import *


class SplashProjectile(Projectile):
    building: "SplashProjectileActiveBuilding"
    target: tuple[float, float]
    position: tuple[float, float]
    speed: tuple[float, float]

    def __init__(
        self,
        building: "SplashProjectileActiveBuilding",
        time_left: float,
        target: "units.Unit",
    ):
        super().__init__(time_left)

        self.building = building
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
                self.building.splash_attack_radius(),
                self.building.attack_damage(),
            )

    def draw(self):
        pygame.draw.circle(
            self.building.game.screen,
            pygame.Color(255, 0, 0),
            (
                self.position[0] * PIXELS_PER_TILE,
                self.position[1] * PIXELS_PER_TILE,
            ),
            3,
        )


class SplashProjectileActiveBuilding(ProjectileActiveBuilding):
    def __init__(
        self,
        game: "game.Game",
        x: int,
        y: int,
        health: float,
        collider: Collider,
    ):
        super().__init__(game, x, y, health, collider)

    @classmethod
    def projectile_type(cls) -> Type[Projectile]:
        return SplashProjectile

    @classmethod
    def splash_attack_radius(cls) -> float: ...

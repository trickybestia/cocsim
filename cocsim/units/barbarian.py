from typing import Union
import pygame

import heapq

from .unit import Unit
from .. import game, buildings

from cocsim.utils import distance, normalize, check_intersection
from cocsim.consts import *


class Barbarian(Unit):
    ATTACK_DAMAGE = 8
    ATTACK_COOLDOWN = 1.0
    SPEED = 2.0
    RANGE = 0.4

    target: Union["buildings.Building", None]
    waypoints: Union[list[tuple[float, float]], None]
    attack_cooldown: Union[float, None]

    def __init__(self, game: "game.Game"):
        super().__init__(game)

        self.target = None
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

            self._find_target()

            if self.target is not None:
                self._find_path(self.target)

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

    def _find_target(self):
        nearest_target = None
        nearest_target_distance = None

        for building in self.game.buildings:
            if not building.destroyed and building.collider is not None:
                nearest_point = building.collider.get_attack_area(
                    self.RANGE
                ).get_nearest_point(self.x, self.y)
                distance_to_building = distance(
                    self.x,
                    self.y,
                    nearest_point[0],
                    nearest_point[1],
                )

                if (
                    nearest_target is None
                    or distance_to_building < nearest_target_distance
                ):
                    nearest_target = building
                    nearest_target_distance = distance_to_building

        self.target = nearest_target

    def _find_path(self, building: "buildings.Building"):
        def get_neighbors(x: int, y: int) -> list[tuple[int, int]]:
            result = []

            for neighbor_x, neighbor_y in (
                (x, y - 1),
                (x + 1, y),
                (x, y + 1),
                (x - 1, y),
            ):
                if (
                    0 <= neighbor_x < MAP_SIZE * COLLISION_TILES_PER_MAP_TILE
                    and 0
                    <= neighbor_y
                    < MAP_SIZE * COLLISION_TILES_PER_MAP_TILE
                    and not self.game.collision[neighbor_x][neighbor_y]
                ):
                    result.append((neighbor_x, neighbor_y))

            return result

        nearest_point = building.collider.get_attack_area(
            self.RANGE
        ).get_nearest_point(self.x, self.y)
        nearest_point_x = int(nearest_point[0] * COLLISION_TILES_PER_MAP_TILE)
        nearest_point_y = int(nearest_point[1] * COLLISION_TILES_PER_MAP_TILE)

        def get_tile_to_check_priority(x: int, y: int) -> int:
            return abs(x - nearest_point_x) + abs(y - nearest_point_y)

        BIG_NUMBER = 1000000000

        distances = [
            [BIG_NUMBER] * MAP_SIZE * COLLISION_TILES_PER_MAP_TILE
            for _ in range(MAP_SIZE * COLLISION_TILES_PER_MAP_TILE)
        ]
        checked_tiles = [
            [False] * MAP_SIZE * COLLISION_TILES_PER_MAP_TILE
            for _ in range(MAP_SIZE * COLLISION_TILES_PER_MAP_TILE)
        ]

        start_x = int(self.x * COLLISION_TILES_PER_MAP_TILE)
        start_y = int(self.y * COLLISION_TILES_PER_MAP_TILE)

        tiles_to_check = []  # heapq's heap
        heapq.heappush(tiles_to_check, (0, start_x, start_y))

        distances[start_x][start_y] = 0
        checked_tiles[start_x][start_y] = True

        while (
            len(tiles_to_check) != 0
            and distances[nearest_point_x][nearest_point_y] == BIG_NUMBER
        ):
            _, x, y = heapq.heappop(tiles_to_check)

            for neighbor_x, neighbor_y in get_neighbors(x, y):
                distances[neighbor_x][neighbor_y] = min(
                    distances[neighbor_x][neighbor_y], distances[x][y] + 1
                )

                if not checked_tiles[neighbor_x][neighbor_y]:
                    heapq.heappush(
                        tiles_to_check,
                        (
                            get_tile_to_check_priority(neighbor_x, neighbor_y),
                            neighbor_x,
                            neighbor_y,
                        ),
                    )

                    checked_tiles[neighbor_x][neighbor_y] = True

        collision_waypoints = [(nearest_point_x, nearest_point_y)]

        while collision_waypoints[-1] != (start_x, start_y):
            x, y = collision_waypoints[-1]

            for neighbor_x, neighbor_y in get_neighbors(x, y):
                if distances[neighbor_x][neighbor_y] == distances[x][y] - 1:
                    collision_waypoints.append((neighbor_x, neighbor_y))

                    break

        collision_waypoints = self._simplify_waypoints(collision_waypoints)

        self.waypoints = [
            (x / COLLISION_TILES_PER_MAP_TILE, y / COLLISION_TILES_PER_MAP_TILE)
            for x, y in collision_waypoints
        ][::-1]

    def _simplify_waypoints(
        self, waypoints: list[tuple[int, int]]
    ) -> list[tuple[int, int]]:
        result = [waypoints[0]]

        for i in range(1, len(waypoints)):
            waypoint = waypoints[i]

            if check_intersection(
                result[-1][0],
                result[-1][1],
                waypoint[0],
                waypoint[1],
                self.game.collision,
            ):
                result.append(waypoints[i - 1])

        return result

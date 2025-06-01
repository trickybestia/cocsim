from .building import Building
from .. import game
from .colliders import Collider
from cocsim.consts import *


class SimpleBuilding(Building):
    def __init__(
        self,
        game: "game.Game",
        x: int,
        y: int,
        health: int,
        collider: Collider,
    ):
        super().__init__(game)

        self.x = x
        self.y = y
        self.health = health
        self.collider = collider

        self.on_destroyed.append(self.update_collision)

    def apply_damage(self, damage: int):
        assert not self.destroyed

        self.health = max(0, self.health - damage)

        if self.destroyed:
            for handler in self.on_destroyed:
                handler(self)

    def update_collision(self, *args):
        for x in range(self.width() * COLLISION_TILES_PER_MAP_TILE):
            for y in range(self.height() * COLLISION_TILES_PER_MAP_TILE):
                abs_x = self.x * COLLISION_TILES_PER_MAP_TILE + x
                abs_y = self.y * COLLISION_TILES_PER_MAP_TILE + y

                if (
                    self.destroyed
                    or (
                        abs_x / COLLISION_TILES_PER_MAP_TILE,
                        abs_y / COLLISION_TILES_PER_MAP_TILE,
                    )
                    not in self.collider
                ):
                    self.game.collision_grid[abs_x][abs_y] = None
                else:
                    self.game.collision_grid[abs_x][abs_y] = self

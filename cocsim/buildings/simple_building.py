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

    def apply_damage(self, damage: int):
        assert not self.destroyed

        self.health = max(0, self.health - damage)

        if self.destroyed:
            self.game.building_destroyed(self)
            self.update_collision()

    def occupy_tiles(self):
        for x in range(self.x, self.x + self.width()):
            for y in range(self.y, self.y + self.height()):
                self.game.occupied_tiles[x][y] = True

    def update_collision(self):
        for x in range(self.width() * COLLISION_TILES_PER_MAP_TILE):
            for y in range(self.height() * COLLISION_TILES_PER_MAP_TILE):
                abs_x = self.x * COLLISION_TILES_PER_MAP_TILE + x
                abs_y = self.y * COLLISION_TILES_PER_MAP_TILE + y

                if self.destroyed:
                    self.game.collision[abs_x][abs_y] = False
                else:
                    self.game.collision[abs_x][abs_y] = (
                        abs_x / COLLISION_TILES_PER_MAP_TILE,
                        abs_y / COLLISION_TILES_PER_MAP_TILE,
                    ) in self.collider

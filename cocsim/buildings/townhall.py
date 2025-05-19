from .building import Building
from .. import game
from .colliders import RectCollider
from cocsim.consts import *


class TownHall(Building):
    WIDTH = 4
    HEIGHT = 4

    COLLIDER = RectCollider.from_center(2, 2, WIDTH * 0.8, HEIGHT * 0.8)

    def __init__(self, game: "game.Game"):
        super().__init__(game)

    def occupy_tiles(self):
        for x in range(self.x, self.x + self.WIDTH):
            for y in range(self.y, self.y + self.HEIGHT):
                self.game.occupied_tiles[x][y] = True

    def update_collision(self):
        for x in range(self.WIDTH * COLLISION_TILES_PER_MAP_TILE):
            for y in range(self.HEIGHT * COLLISION_TILES_PER_MAP_TILE):
                abs_x = self.x * COLLISION_TILES_PER_MAP_TILE + x
                abs_y = self.y * COLLISION_TILES_PER_MAP_TILE + y

                if self.destroyed:
                    self.game.collision[abs_x][abs_y] = False
                else:
                    self.game.collision[abs_x][abs_y] = (x, y) in self.COLLIDER

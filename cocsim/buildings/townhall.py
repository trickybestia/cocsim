from .building import Building
from .. import game


class TownHall(Building):
    def __init__(self, game: "game.Game"):
        super().__init__(game)

    def get_occupied_tiles(self) -> list[tuple[int, int]]:
        return [
            (x, y)
            for x in range(self.x, self.x + 4)
            for y in range(self.y, self.y + 4)
        ]

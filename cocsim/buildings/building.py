from .. import game
from .colliders import Collider


class Building:
    game: "game.Game"
    destroyed: bool
    collider: Collider | None
    x: int
    y: int

    def __init__(self, game: "game.Game"):
        self.game = game
        self.destroyed = False
        self.collider = None
        self.x = 0
        self.y = 0

    def tick(self, delta_t: float): ...

    def draw(self): ...

    def occupy_tiles(self):
        """Occupy tiles for troops drop zone calculation. Called once."""
        ...

    def update_collision(self):
        """Update collision for this building. Can be called multiple times.
        Need check for self.destroyed."""
        ...

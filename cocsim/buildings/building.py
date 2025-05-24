from .. import game
from .colliders import Collider


class Building:
    game: "game.Game"
    health: int
    collider: Collider | None
    x: int
    y: int

    @classmethod
    def width(cls) -> int: ...

    @classmethod
    def height(cls) -> int: ...

    @classmethod
    def levels(cls) -> int:
        """Returns levels count."""

    @property
    def destroyed(self) -> bool:
        return self.health == 0

    def __init__(self, game: "game.Game"):
        self.game = game
        self.collider = None
        self.x = 0
        self.y = 0

    def tick(self, delta_t: float): ...

    def draw(self): ...

    def apply_damage(self, damage: int):
        """Apply damage to this building. Called by units when they attack."""

    def occupy_tiles(self):
        """Occupy tiles for troops drop zone calculation. Called once."""

    def update_collision(self):
        """Update collision for this building. Can be called multiple times.
        Need check for self.destroyed."""

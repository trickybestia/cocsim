from typing import Union

from .. import buildings, game


class Unit:
    game: "game.Game"
    health: float
    x: float
    y: float

    target: Union["buildings.Building", None]

    @property
    def dead(self) -> bool:
        return self.health == 0.0

    @classmethod
    def levels(cls) -> int:
        """Returns levels count."""

    @classmethod
    def housing_space(cls) -> int:
        """Returns unit Housing Space."""

    @property
    def attack_range(self) -> float: ...

    def __init__(self, game: "game.Game", x: float, y: float, health: float):
        self.game = game
        self.health = health
        self.x = x
        self.y = y

        self.target = None

    def apply_damage(self, damage: float):
        assert not self.dead

        self.health = max(0.0, self.health - damage)

    def tick(self, delta_t: float): ...

    def draw(self): ...

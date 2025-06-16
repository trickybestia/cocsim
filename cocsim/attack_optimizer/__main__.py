from typing import Type

from cocsim.units import Unit, Balloon, Dragon, Barbarian
from cocsim.utils import load_test_map
from cocsim.game_gui import GameGui
from cocsim.game import Game

from .attack_optimizer import AttackOptimizer

MAP_PATH = "single_player/goblin_gauntlet"
HOUSING_SPACE = 20
UNITS: list[tuple[Type[Unit], int]] = [
    (Balloon, 1),
    (Dragon, 1),
    (Barbarian, 1),
]


def main():
    map, base_image = load_test_map(MAP_PATH)

    optimizer = AttackOptimizer(map, HOUSING_SPACE, UNITS)

    game = Game(map, None)

    game.units = []

    gui = GameGui(game)

    gui.run()


main()

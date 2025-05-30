from cocsim.utils import load_test_map
from cocsim.game import Game
from cocsim.consts import *
from cocsim.units import Barbarian


def single_player_goblin_gauntlet_0():
    map, base_image = load_test_map("single_player/goblin_gauntlet")
    game = Game(map)

    game.units = [Barbarian(game, 0.0, 0.0)]

    game.tick(1 / FPS)

    expected_target = ...

    assert game.units[0].target == expected_target


def main():
    single_player_goblin_gauntlet_0()


main()

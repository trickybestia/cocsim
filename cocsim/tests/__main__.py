from cocsim.utils import load_test_map
from cocsim.game import Game
from cocsim.consts import *
from cocsim.units import Barbarian


def single_player_goblin_gauntlet_0():
    """Targeting test"""

    map, base_image = load_test_map("single_player/goblin_gauntlet")
    game = Game(map, base_image)

    left_cannon = game.buildings_grid[7][7]
    right_cannon = game.buildings_grid[19][16]
    bottom_goblin_hut = game.buildings_grid[12][18]

    left_cannon_units = []
    right_cannon_units = []
    bottom_goblin_hut_units = []

    for y in range(19):
        left_cannon_units.append(Barbarian(game, 0, 0.5, y + 0.5))
    for x in range(1, 25):
        left_cannon_units.append(Barbarian(game, 0, x + 0.5, 0.5))

    for x in range(25, 32):
        right_cannon_units.append(Barbarian(game, 0, x + 0.5, 0.5))
    for y in range(1, 32):
        right_cannon_units.append(Barbarian(game, 0, 31.5, y + 0.5))
    for x in range(18, 31):
        right_cannon_units.append(Barbarian(game, 0, x + 0.5, 31.5))

    for x in range(7, 18):
        bottom_goblin_hut_units.append(Barbarian(game, 0, x + 0.5, 31.5))
    for y in range(19, 32):
        bottom_goblin_hut_units.append(Barbarian(game, 0, 0.5, y + 0.5))

    game.units = [
        *left_cannon_units,
        *right_cannon_units,
        *bottom_goblin_hut_units,
    ]

    game.tick(1 / FPS)

    for unit in left_cannon_units:
        assert unit.target == left_cannon

    for unit in right_cannon_units:
        assert unit.target == right_cannon

    for unit in bottom_goblin_hut_units:
        assert unit.target == bottom_goblin_hut


def main():
    single_player_goblin_gauntlet_0()


main()

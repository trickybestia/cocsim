from cocsim.utils import load_test_map
from cocsim.game_gui import GameGui
from cocsim.units import Dragon, Barbarian
from cocsim.game import Game


def main():
    map, base_image = load_test_map("single_player/goblin_gauntlet")

    game = Game(map, base_image)

    for y in range(19):
        game.units.append(Barbarian(game, 1, 0.5, y + 0.5))
    for x in range(1, 25):
        game.units.append(Barbarian(game, 1, x + 0.5, 0.5))

    gui = GameGui(game)

    gui.run()


main()

from typing import Type

from cocsim.game import Game
from cocsim.game_gui import GameGui
from cocsim.units import Balloon, Barbarian, Dragon, Unit
from cocsim.utils import load_test_map

from .army import Army
from .army_optimizer import ArmyOptimizer
from .attack_plan_executor import AttackPlanExecutor
from .attack_plan_optimizer import AttackPlanOptimizer

MAP_PATH = "single_player/goblin_gauntlet"
HOUSING_SPACE = 20
UNITS: list[tuple[Type[Unit], int]] = [
    (Balloon, 1),
    (Dragon, 1),
    (Barbarian, 1),
]


def main():
    map, base_image = load_test_map(MAP_PATH)

    optimizer = AttackPlanOptimizer(map, Army(UNITS))

    try:
        for i, score, attack_plan in optimizer.run():
            print(i, score, attack_plan)
    except KeyboardInterrupt:
        ...

    game = Game(map, base_image)
    attack_plan_executor = AttackPlanExecutor(game, attack_plan)

    gui = GameGui(game)

    gui.before_tick = attack_plan_executor.tick

    gui.run()


main()

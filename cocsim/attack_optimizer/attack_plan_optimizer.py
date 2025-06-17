from typing import Generator
from cocsim.map import Map
from cocsim.game import Game
from cocsim.consts import *

from .army import Army
from .attack_plan import AttackPlan
from .attack_plan_executor import AttackPlanExecutor


class AttackPlanOptimizer:
    map: Map
    army: Army

    def __init__(self, map: Map, army: Army):
        self.map = map
        self.army = army

    def run(self) -> Generator[tuple[int, float, AttackPlan], None, None]:
        i = 0

        best_score = None

        while True:
            attack_plan = AttackPlan.randomize(self.army.units)
            score = self._score_attack_plan(attack_plan)

            if best_score is None or score > best_score:
                best_score = score

                yield i, best_score, attack_plan

            i += 1

    def _score_attack_plan(self, attack_plan: AttackPlan) -> float:
        game = Game(self.map, None)
        attack_plan_executor = AttackPlanExecutor(game, attack_plan)

        while not game.done:
            attack_plan_executor.tick()
            game.tick(1 / FPS)

        return game.time_left

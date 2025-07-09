from random import choice
from typing import Generator

from cocsim.consts import *
from cocsim.game import Game
from cocsim.map_model import MapModel
from cocsim.units import UnitsModel

from .attack_plan import AttackPlan
from .attack_plan_executor import AttackPlanExecutor

POPULATION_SIZE = 20
NEW_POPULATION_SIZE = 40
NEW_RANDOM_PLANS = 5


class AttackPlanOptimizer:
    map: MapModel
    units: UnitsModel

    def __init__(self, map: MapModel, units: UnitsModel):
        self.map = map
        self.units = units

    def run(self) -> Generator[tuple[int, float, AttackPlan], None, None]:
        population = [
            AttackPlan.randomize(self.units) for _ in range(POPULATION_SIZE)
        ]

        i = 0
        best_score = None

        while True:
            new_population = []

            for _ in range(NEW_RANDOM_PLANS):
                new_population.append(AttackPlan.randomize(self.units))

            while len(new_population) < NEW_POPULATION_SIZE:
                a = choice(population)
                b = choice(population)

                new_population.append(AttackPlan.merge(a, b).mutate())

            new_population.sort(key=self._score_attack_plan, reverse=True)

            score = self._score_attack_plan(new_population[0])

            if best_score is None or score > best_score:
                best_score = score

                yield i, best_score, new_population[0]

            population = new_population[:POPULATION_SIZE]
            i += 1

    def _score_attack_plan(self, attack_plan: AttackPlan) -> float:
        game = Game(self.map)
        attack_plan_executor = AttackPlanExecutor(game, attack_plan)

        while not game.done:
            attack_plan_executor.tick()
            game.tick(1 / FPS)

        return game.time_left

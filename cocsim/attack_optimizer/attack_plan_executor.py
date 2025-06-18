from cocsim.game import Game

from .attack_plan import AttackPlanUnit, AttackPlan


class AttackPlanExecutor:
    game: Game
    units: list[AttackPlanUnit]

    def __init__(self, game: Game, attack_plan: AttackPlan):
        self.game = game
        self.units = sorted(
            attack_plan.units, key=lambda unit: unit.drop_time, reverse=True
        )

    def tick(self):
        while (
            len(self.units) != 0
            and self.units[-1].drop_time <= self.game.time_elapsed
        ):
            unit = self.units.pop()
            x, y = unit.cartesian_pos(self.game)

            self.game.units.append(unit.unit(self.game, unit.level, x, y))

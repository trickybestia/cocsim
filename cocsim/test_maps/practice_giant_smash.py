from cocsim.game import Game

from .test_map import TestMap


class PracticeGiantSmash(TestMap):
    def load(self, game: Game):
        return super().load(game)

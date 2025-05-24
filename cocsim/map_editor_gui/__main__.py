from cocsim.consts import *
from cocsim.buildings import BUILDINGS


def main():
    for building in BUILDINGS:
        print(
            building.__name__,
            building.width(),
            building.height(),
            building.levels(),
        )


main()

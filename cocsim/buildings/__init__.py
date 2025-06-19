from .active_building import ActiveBuilding
from .building import Building, BUILDINGS
from .passive_building import PassiveBuilding
from .projectile_active_building import ProjectileActiveBuilding
from .simple_building import SimpleBuilding
from .splash_projectile_active_building import SplashProjectileActiveBuilding

from .air_defense import AirDefense
from .archer_tower import ArcherTower
from .army_camp import ArmyCamp
from .barracks import Barracks
from .bomb_tower import BombTower
from .builders_hut import BuildersHut
from .cannon import Cannon
from .clan_castle import ClanCastle
from .dark_elixir_storage import DarkElixirStorage
from .elixir_collector import ElixirCollector
from .elixir_storage import ElixirStorage
from .goblin_hut import GoblinHut
from .gold_mine import GoldMine
from .gold_storage import GoldStorage
from .laboratory import Laboratory
from .mortar import Mortar
from .townhall import TownHall
from .wall import Wall
from .wizard_tower import WizardTower

BUILDINGS_DICT = dict((building.__name__, building) for building in BUILDINGS)

# TODO

* Add attack optimizer (finally).
* Make Balloon apply damage to buildings on death. [balloon.py](../../cocsim/units/balloon.py)
* Add Ctrl+S shortcut to [cocsim.map_editor_gui](../../cocsim/map_editor_gui/). Or simple terminal text prompt to ask if user wants to save changed map.
* Add other buildings.
* Add other units.
* Check if different levels of wizard towers have different projectile speed. If so, major rework is required. [wizard_tower.py](../../cocsim/buildings/wizard_tower.py)
* Tune defensive buildings first attack delay. Currently it is the same as attack cooldown, but in fact it is wrong. Mostly noticeable on [mortar.py](../../cocsim/buildings/mortar.py). It's attack cooldown is 5 secs, but first projectile is launched ~1.3 secs after target is noticed (launch game to check).
* Research for a need to tune units first attack delay. Not sure if units behave like buildings. See point above.
* Add defensive units.
    * Make Balloon splash attack. [balloon.py](../../cocsim/units/baloon.py)
    * Make Dragon splash attack. [dragon.py](../../cocsim/units/dragon.py)
* Fix invalid ground units pathfinding. [pathfinder.py](../../cocsim/pathfinder.py)
* Write tests.
    * Add other test maps.

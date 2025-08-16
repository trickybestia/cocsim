# TODO

- Add other buildings.
- Add other units.
- Implement Lightning Spell stun buildings. [lightning_spell.rs](../../backend/cocsim/src/spells/lightning_spell.rs)
- Implement X-Bow running out of ammo. [x_bow.rs](../../backend/cocsim/src/buildings/x_bow.rs)
- Tune defensive buildings first attack delay. Currently it is the same as attack cooldown, but in fact it is wrong. Mostly noticeable on [mortar.rs](../../backend/cocsim/src/buildings/mortar.rs). It's attack cooldown is 5 secs, but first projectile is launched ~1.3 secs after target is noticed (launch game to check).
- Research for a need to tune units first attack delay. Not sure if units behave like buildings. See point above.
- Add defensive units.
  - Make Dragon splash attack. [dragon.rs](../../backend/cocsim/src/units/dragon.rs)
- Add ground units.

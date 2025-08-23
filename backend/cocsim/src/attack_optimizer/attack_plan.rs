use std::hash::Hash;

use arbitrary::Arbitrary;
use nalgebra::Vector2;
use rand::{
    Rng,
    seq::IndexedRandom,
};

use crate::{
    SpellModelEnum,
    UnitModelEnum,
    ValidatedMap,
    attack_optimizer::{
        AttackPlanUnitGroup,
        attack_plan_executor::{
            AttackPlanExecutorAction,
            Spawnable,
        },
        attack_plan_spell_group::AttackPlanSpellGroup,
    },
    consts::{
        MAX_UNIT_DROP_TIME,
        SPELL_DROP_COOLDOWN,
        SPELL_GROUP_DROP_COOLDOWN,
        UNIT_DROP_COOLDOWN,
        UNIT_GROUP_DROP_COOLDOWN,
    },
    with_housing_space::WithCount,
};

const POSITION_QUANTS_PER_TILE: i32 = 2;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AttackPlanPosition {
    restricted_to_drop_zone: bool,
    x: i32,
    y: i32,
}

impl AttackPlanPosition {
    pub fn new_randomized(
        map: &ValidatedMap,
        restricted_to_drop_zone: bool,
        rng: &mut impl Rng,
    ) -> Self {
        let tile = if restricted_to_drop_zone {
            *map.drop_zone_free_tiles()
                .choose(rng)
                .expect("At least one free tile expected")
        } else {
            (
                rng.random_range(0..map.map_size().total_size()) as usize,
                rng.random_range(0..map.map_size().total_size()) as usize,
            )
        };

        Self {
            restricted_to_drop_zone,
            x: tile.0 as i32 * POSITION_QUANTS_PER_TILE
                + rng.random_range(0..POSITION_QUANTS_PER_TILE),
            y: tile.1 as i32 * POSITION_QUANTS_PER_TILE
                + rng.random_range(0..POSITION_QUANTS_PER_TILE),
        }
    }

    fn neighbors(&self, map: &ValidatedMap, radius: usize) -> Vec<Self> {
        let radius = radius as i32;

        let mut result = Vec::new();

        for x in (self.x - radius)..=(self.x + radius) {
            for y in (self.y - radius)..=(self.y + radius) {
                let drop_zone_tile_position = Vector2::new(x, y) / POSITION_QUANTS_PER_TILE;

                if map.map_size().is_inside_map(drop_zone_tile_position)
                    && (!self.restricted_to_drop_zone
                        || map.drop_zone()[(
                            drop_zone_tile_position.x as usize,
                            drop_zone_tile_position.y as usize,
                        )])
                {
                    result.push(Self {
                        restricted_to_drop_zone: self.restricted_to_drop_zone,
                        x,
                        y,
                    });
                }
            }
        }

        result
    }

    pub fn to_position(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y).cast() / POSITION_QUANTS_PER_TILE as f32
            + Vector2::from_element((POSITION_QUANTS_PER_TILE as f32).recip() / 2.0)
    }
}

const TIME_QUANTS_PER_SECOND: i32 = 2;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AttackPlanTime(i32);

impl AttackPlanTime {
    fn new_randomized(rng: &mut impl Rng) -> Self {
        Self(rng.random_range(0..=(TIME_QUANTS_PER_SECOND * MAX_UNIT_DROP_TIME as i32)))
    }

    fn neighbors(&self, radius: usize) -> Vec<Self> {
        let radius = radius as i32;

        let mut result = Vec::new();

        for time in (self.0 - radius)..=(self.0 + radius) {
            if time >= 0 && time <= (TIME_QUANTS_PER_SECOND * MAX_UNIT_DROP_TIME as i32) {
                result.push(Self(time));
            }
        }

        result
    }

    pub fn to_time(&self) -> f32 {
        self.0 as f32 / TIME_QUANTS_PER_SECOND as f32
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct AttackPlanParameters {
    pub positions: Vec<AttackPlanPosition>,
    pub times: Vec<AttackPlanTime>,
}

impl AttackPlanParameters {
    pub fn new_randomized(
        request: &AttackOptimizerRequest,
        map: &ValidatedMap,
        rng: &mut impl Rng,
    ) -> Self {
        let mut positions = Vec::new();
        let mut times = Vec::new();

        for _unit in &request.units {
            positions.push(AttackPlanPosition::new_randomized(map, true, rng));
            times.push(AttackPlanTime::new_randomized(rng));
        }

        for _spell in &request.spells {
            positions.push(AttackPlanPosition::new_randomized(map, false, rng));
            times.push(AttackPlanTime::new_randomized(rng));
        }

        Self { positions, times }
    }

    pub fn neighbors(
        &self,
        map: &ValidatedMap,
        radius: usize,
        count: usize,
        rng: &mut impl Rng,
    ) -> impl Iterator<Item = Self> {
        let positions_variants = self
            .positions
            .iter()
            .map(|p| p.neighbors(map, radius))
            .collect::<Vec<_>>();
        let times_variants = self
            .times
            .iter()
            .map(|t| t.neighbors(radius))
            .collect::<Vec<_>>();

        (0..count).map(move |_| {
            let positions = positions_variants
                .iter()
                .map(|variants| variants.choose(rng).unwrap().to_owned())
                .collect::<Vec<_>>();
            let times = times_variants
                .iter()
                .map(|variants| variants.choose(rng).unwrap().to_owned())
                .collect::<Vec<_>>();

            Self { positions, times }
        })
    }
}

pub struct AttackOptimizerRequest {
    pub units: Vec<WithCount<UnitModelEnum>>,
    pub spells: Vec<WithCount<SpellModelEnum>>,
}

enum UnitGroupOrSpell {
    UnitGroup(AttackPlanUnitGroup),
    Spell(AttackPlanSpellGroup),
}

impl UnitGroupOrSpell {
    pub fn drop_time(&self) -> f32 {
        match self {
            UnitGroupOrSpell::UnitGroup(unit_group) => unit_group.drop_time,
            UnitGroupOrSpell::Spell(spell) => spell.drop_time,
        }
    }
}

#[derive(Clone, Arbitrary, Debug)]
pub struct AttackPlan {
    pub units: Vec<AttackPlanUnitGroup>,
    pub spells: Vec<AttackPlanSpellGroup>,
}

impl AttackPlan {
    pub fn new_randomized(
        units: &[WithCount<UnitModelEnum>],
        spells: &[WithCount<SpellModelEnum>],
        rng: &mut impl Rng,
    ) -> Self {
        Self {
            units: units
                .iter()
                .map(|unit| {
                    AttackPlanUnitGroup::new_randomized(unit.value.clone(), unit.count, rng)
                })
                .collect(),
            spells: spells
                .iter()
                .map(|spell| {
                    AttackPlanSpellGroup::new_randomized(spell.value.clone(), spell.count, rng)
                })
                .collect(),
        }
    }

    pub fn merge(a: &Self, b: &Self, rng: &mut impl Rng) -> Self {
        let units = a
            .units
            .iter()
            .zip(b.units.iter())
            .map(|(a, b)| (*[a, b].choose(rng).unwrap()).clone())
            .collect();
        let spells = a
            .spells
            .iter()
            .zip(b.spells.iter())
            .map(|(a, b)| (*[a, b].choose(rng).unwrap()).clone())
            .collect();

        Self { units, spells }
    }

    pub fn mutate(&self, rng: &mut impl Rng, temperature: f32) -> Self {
        let units = self
            .units
            .iter()
            .map(|unit| unit.mutate(rng, temperature))
            .collect();
        let spells = self
            .spells
            .iter()
            .map(|spell| spell.mutate(rng, temperature))
            .collect();

        Self { units, spells }
    }

    pub fn executor_actions(&self, map: &ValidatedMap) -> Vec<AttackPlanExecutorAction> {
        let mut groups = self
            .units
            .iter()
            .cloned()
            .map(UnitGroupOrSpell::UnitGroup)
            .chain(self.spells.iter().cloned().map(UnitGroupOrSpell::Spell))
            .collect::<Vec<_>>();

        // sort reversed by drop_time key
        groups.sort_unstable_by(|a, b| b.drop_time().total_cmp(&a.drop_time()));

        let mut result = Vec::new();
        let mut next_drop_time = 0.0f32;

        for unit_group_or_spell in groups {
            match unit_group_or_spell {
                UnitGroupOrSpell::UnitGroup(unit_group) => {
                    let position = unit_group.cartesian_position(&map.map_size(), map.drop_zone());

                    next_drop_time = next_drop_time.max(unit_group.drop_time);

                    for _ in 0..unit_group.count {
                        result.push(AttackPlanExecutorAction {
                            spawnable: Spawnable::Unit(unit_group.unit_model.clone()),
                            position,
                            drop_time: next_drop_time,
                        });

                        next_drop_time += UNIT_DROP_COOLDOWN;
                    }

                    next_drop_time += UNIT_GROUP_DROP_COOLDOWN;
                }
                UnitGroupOrSpell::Spell(spell) => {
                    let position = spell.cartesian_position(&map.map_size());

                    next_drop_time = next_drop_time.max(spell.drop_time);

                    for _ in 0..spell.count {
                        result.push(AttackPlanExecutorAction {
                            spawnable: Spawnable::Spell(spell.spell_model.clone()),
                            position,
                            drop_time: next_drop_time,
                        });

                        next_drop_time += SPELL_DROP_COOLDOWN;
                    }

                    next_drop_time += SPELL_GROUP_DROP_COOLDOWN;
                }
            }
        }

        result.reverse();

        result
    }
}

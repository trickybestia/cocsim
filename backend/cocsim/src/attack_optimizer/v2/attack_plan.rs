use std::iter::repeat;

use nalgebra::Vector2;
use rand::{
    Rng,
    seq::IndexedRandom,
};

use crate::{
    ValidatedMap,
    attack_optimizer::Army,
    consts::MAX_UNIT_DROP_TIME,
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
pub struct AttackPlan {
    pub positions: Vec<AttackPlanPosition>,
    pub times: Vec<AttackPlanTime>,
}

impl AttackPlan {
    pub fn new_randomized(army: &Army, map: &ValidatedMap, rng: &mut impl Rng) -> Self {
        let mut positions = Vec::new();
        let mut times = Vec::new();

        for _unit in &army.units {
            positions.push(AttackPlanPosition::new_randomized(map, true, rng));
            times.push(AttackPlanTime::new_randomized(rng));
        }

        for _spell in &army.spells {
            positions.push(AttackPlanPosition::new_randomized(map, false, rng));
            times.push(AttackPlanTime::new_randomized(rng));
        }

        Self { positions, times }
    }

    pub fn neighbors(
        &self,
        map: &ValidatedMap,
        radius: usize,
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

        let all_variants = positions_variants
            .iter()
            .map(|variants| variants.len())
            .product::<usize>()
            * times_variants
                .iter()
                .map(|variants| variants.len())
                .product::<usize>();

        println!("All variants: {}", all_variants);

        repeat(()).map(move |_| {
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

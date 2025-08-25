use arbitrary::Arbitrary;

use crate::{
    spells::SpellModelEnum,
    units::UnitModelEnum,
    with_housing_space::WithCount,
};

#[derive(Debug, Clone, Arbitrary)]
pub struct Army {
    pub units: Vec<WithCount<UnitModelEnum>>,
    pub spells: Vec<WithCount<SpellModelEnum>>,
}

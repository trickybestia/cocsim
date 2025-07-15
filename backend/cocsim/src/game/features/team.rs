use std::ops::Not;

use shipyard::Component;

#[derive(Component, PartialEq, Eq)]
pub enum Team {
    Attack,
    Defense,
}

impl Not for Team {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Team::Attack => Team::Defense,
            Team::Defense => Team::Attack,
        }
    }
}

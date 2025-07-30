use std::{
    fmt::Display,
    ops::Deref,
};

use arbitrary::Arbitrary;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
#[serde(try_from = "usize", into = "usize")]
pub struct LevelIndex<const MAX: usize>(usize);

impl<const MAX: usize> LevelIndex<MAX> {
    pub fn new(value: usize) -> Result<Self, LevelIndexError> {
        if value <= MAX {
            Ok(Self(value))
        } else {
            Err(LevelIndexError {
                max: MAX,
                got: value,
            })
        }
    }
}

impl<const MAX: usize> Deref for LevelIndex<MAX> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX: usize> TryFrom<usize> for LevelIndex<MAX> {
    type Error = LevelIndexError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MAX: usize> From<LevelIndex<MAX>> for usize {
    fn from(value: LevelIndex<MAX>) -> Self {
        value.0
    }
}

impl<'a, const MAX: usize> Arbitrary<'a> for LevelIndex<MAX> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(u.int_in_range(0..=MAX)?))
    }
}

#[derive(Debug)]
pub struct LevelIndexError {
    pub max: usize,
    pub got: usize,
}

impl Display for LevelIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Level index is larger than MAX (MAX = {}, got = {})",
            self.max, self.got
        )?;

        Ok(())
    }
}

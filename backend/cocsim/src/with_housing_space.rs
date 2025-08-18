use std::{
    fmt::{
        Debug,
        Display,
    },
    ops::Deref,
};

use arbitrary::{
    Arbitrary,
    Unstructured,
};
use serde::{
    Deserialize,
    Serialize,
};

pub trait WithHousingSpace {
    fn housing_space(&self) -> usize;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "Vec<T>", into = "Box<[T]>")]
pub struct WithMaxHousingSpace<const MAX_HOUSING_SPACE: usize, T: Clone + Debug + WithHousingSpace>(
    Box<[T]>,
);

impl<const MAX_HOUSING_SPACE: usize, T: Clone + Debug + WithHousingSpace>
    WithMaxHousingSpace<MAX_HOUSING_SPACE, T>
{
    pub fn new(units: &[T]) -> Result<Self, HousingSpaceError> {
        let housing_space = units.iter().map(WithHousingSpace::housing_space).sum();

        if housing_space <= MAX_HOUSING_SPACE {
            Ok(Self(units.into()))
        } else {
            Err(HousingSpaceError {
                max: MAX_HOUSING_SPACE,
                got: housing_space,
            })
        }
    }
}

impl<const MAX_HOUSING_SPACE: usize, T: Clone + Debug + WithHousingSpace> Deref
    for WithMaxHousingSpace<MAX_HOUSING_SPACE, T>
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX_HOUSING_SPACE: usize, T: Clone + Debug + WithHousingSpace> Into<Box<[T]>>
    for WithMaxHousingSpace<MAX_HOUSING_SPACE, T>
{
    fn into(self) -> Box<[T]> {
        self.0
    }
}

impl<const MAX_HOUSING_SPACE: usize, T: Clone + Debug + WithHousingSpace> TryFrom<&[T]>
    for WithMaxHousingSpace<MAX_HOUSING_SPACE, T>
{
    type Error = HousingSpaceError;

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MAX_HOUSING_SPACE: usize, T: Clone + Debug + WithHousingSpace> TryFrom<Vec<T>>
    for WithMaxHousingSpace<MAX_HOUSING_SPACE, T>
{
    type Error = HousingSpaceError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl<'a, const MAX_HOUSING_SPACE: usize, T: Clone + Debug + WithHousingSpace + Arbitrary<'a>>
    Arbitrary<'a> for WithMaxHousingSpace<MAX_HOUSING_SPACE, T>
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut result = Vec::new();
        let mut housing_space = 0;

        loop {
            let value = T::arbitrary(u)?;

            if housing_space + value.housing_space() <= MAX_HOUSING_SPACE {
                housing_space += value.housing_space();
                result.push(value);
            } else {
                return Ok(Self(result.into_boxed_slice()));
            }
        }
    }
}

#[derive(Debug)]
pub struct HousingSpaceError {
    pub max: usize,
    pub got: usize,
}

impl Display for HousingSpaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Housing space is larger than MAX_HOUSING_SPACE (MAX_HOUSING_SPACE = {}, got = {})",
            self.max, self.got
        )?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WithCount<T: WithHousingSpace> {
    pub value: T,
    pub count: usize,
}

impl<T: WithHousingSpace> WithHousingSpace for WithCount<T> {
    fn housing_space(&self) -> usize {
        self.value.housing_space() * self.count
    }
}

impl<'a, T: WithHousingSpace + Arbitrary<'a>> Arbitrary<'a> for WithCount<T> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            value: T::arbitrary(u)?,
            count: 1,
        })
    }
}

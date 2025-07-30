mod air_defense;
mod air_sweeper;
mod archer_tower;
mod army_camp;
mod barracks;
mod bomb_tower;
mod builders_hut;
mod cannon;
mod clan_castle;
mod dark_elixir_drill;
mod dark_elixir_storage;
mod elixir_collector;
mod elixir_storage;
mod goblin_hut;
mod gold_mine;
mod gold_storage;
mod laboratory;
mod mortar;
mod town_hall;
pub mod utils;
mod wall;
mod wizard_tower;
mod x_bow;

pub use air_defense::*;
pub use air_sweeper::*;
use arbitrary::Arbitrary;
pub use archer_tower::*;
pub use army_camp::*;
pub use barracks::*;
pub use bomb_tower::*;
pub use builders_hut::*;
pub use cannon::*;
pub use clan_castle::*;
pub use dark_elixir_drill::*;
pub use dark_elixir_storage::*;
pub use elixir_collector::*;
pub use elixir_storage::*;
use enum_dispatch::enum_dispatch;
pub use goblin_hut::*;
pub use gold_mine::*;
pub use gold_storage::*;
pub use laboratory::*;
pub use mortar::*;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;
pub use town_hall::*;
pub use wall::*;
pub use wizard_tower::*;
pub use x_bow::*;

#[derive(Serialize)]
pub struct BuildingOption {
    pub name: &'static str,
    pub values: &'static [&'static str],
}

#[derive(Serialize)]
pub struct BuildingType {
    pub name: &'static str,
    pub size: Vector2<usize>,
    pub levels: usize,
    pub options: &'static [BuildingOption],
}

inventory::collect!(BuildingType);

#[enum_dispatch]
pub trait BuildingModel {
    fn r#type(&self) -> &'static BuildingType;

    fn position(&self) -> Vector2<usize>;

    fn create_building(&self, world: &mut World);
}

#[enum_dispatch(BuildingModel)]
#[derive(Serialize, Deserialize, Debug, Arbitrary)]
#[serde(tag = "name")]
pub enum BuildingModelEnum {
    #[serde(rename = "AirDefense")]
    AirDefenseModel,
    #[serde(rename = "AirSweeper")]
    AirSweeperModel,
    #[serde(rename = "ArmyCamp")]
    ArmyCampModel,
    #[serde(rename = "ArcherTower")]
    ArcherTowerModel,
    #[serde(rename = "Barracks")]
    BarracksModel,
    #[serde(rename = "BombTower")]
    BombTowerModel,
    #[serde(rename = "BuildersHut")]
    BuildersHutModel,
    #[serde(rename = "Cannon")]
    CannonModel,
    #[serde(rename = "ClanCastle")]
    ClanCastleModel,
    #[serde(rename = "DarkElixirDrill")]
    DarkElixirDrillModel,
    #[serde(rename = "DarkElixirStorage")]
    DarkElixirStorageModel,
    #[serde(rename = "ElixirCollector")]
    ElixirCollectorModel,
    #[serde(rename = "ElixirStorage")]
    ElixirStorageModel,
    #[serde(rename = "GoblinHut")]
    GoblinHutModel,
    #[serde(rename = "GoldMine")]
    GoldMineModel,
    #[serde(rename = "GoldStorage")]
    GoldStorageModel,
    #[serde(rename = "Laboratory")]
    LaboratoryModel,
    #[serde(rename = "Mortar")]
    MortarModel,
    #[serde(rename = "TownHall")]
    TownHallModel,
    #[serde(rename = "Wall")]
    WallModel,
    #[serde(rename = "WizardTower")]
    WizardTowerModel,
    #[serde(rename = "XBow")]
    XBowModel,
}

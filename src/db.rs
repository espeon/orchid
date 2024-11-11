// As TurboSQL is not your average SQL library (no exposed connections/pools/etc) we'll just have the schema here.

use serde::{Deserialize, Serialize};
use turbosql::Turbosql;

#[derive(Serialize, Deserialize)]
/// Bottom stream layout items. References an ID to another table.
pub enum BottomLayoutItems {
    /// Reference to a PkTeam
    PkTeamLayout(i64),
    /// Reference to a PkBadgeCollection
    PkBadgeLayout(i64),
}

#[derive(Serialize, Deserialize, Turbosql)]
pub struct PkTeam {
    rowid: Option<i64>,
    pub pokemon_ids: Vec<i64>,
    pub team_id: i64,
}

#[derive(Serialize, Deserialize)]
pub enum PkType {
    Bug,
    Dark,
    Dragon,
    Electric,
    Fairy,
    Fighting,
    Fire,
    Flying,
    Ghost,
    Grass,
    Ground,
    Ice,
    Normal,
    Poison,
    Psychic,
    Rock,
    Steel,
    Water,
}

pub struct Pokemon {
    pub rowid: Option<i64>,
    pub name: String,
    pub image: String,
    pub description: String,
    pub types: Vec<PkType>,
    pub abilities: Vec<String>,
    pub height: i32,
    pub weight: i32,
    pub gender: String,
    pub shiny: bool,
}

#[derive(Serialize, Deserialize, Turbosql)]
pub struct PkBadgeCollection {
    rowid: Option<i64>,
    /// Reference to a PkBadge
    pub badge_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Turbosql)]
pub struct PkBadge {
    rowid: Option<i64>,
    pub name: String,
    pub image: String,
    pub obtained: bool,
}

#[derive(Turbosql, Default)]
pub struct BottomLayout {
    rowid: Option<i64>, // rowid member required & enforced
    /// Layout items, in order from left to right.
    layout_items: Vec<BottomLayoutItems>,
}

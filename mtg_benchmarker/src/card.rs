// TODO: add support for super/subtypes
#[derive(Copy, Clone)]
pub enum CardType {
    Land,
    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery
}

// #[derive(Copy, Clone)]
// pub enum ManaType {
//     RED,
//     GREEN,
//     BLUE,
//     BLACK,
//     WHITE,
//     MULTICOLORED
// }

pub struct Card {
    name: String,
    mana_cost: String,
    colors: Vec<String>,
    color_identity: Vec<String>,
    pub _type: CardType,
    power: usize,
    toughness: usize,
    id: u32
}

impl Card {
    pub fn new(
        name: String,
        mana_cost: String,
        colors: Vec<String>,
        color_identity: Vec<String>,
        _type: CardType,
        power: usize,
        toughness: usize,
        id: u32
    ) -> Self {
        Card {
            name: name,
            mana_cost: mana_cost,
            colors: colors,
            color_identity: color_identity,
            _type: _type,
            power: power,
            toughness: toughness,
            id: id
        }
    }
}
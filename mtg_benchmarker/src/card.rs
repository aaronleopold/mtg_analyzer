
pub struct Card {
    name: String,
    mana_cost: String,
    colors: Vec<String>,
    color_identity: Vec<String>,
    _type: String,
    power: usize,
    toughness: usize,
    id: u32
}
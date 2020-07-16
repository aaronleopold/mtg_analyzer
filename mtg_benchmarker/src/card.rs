// TODO: add support for super/subtypes
pub enum CardType {
    Land,
    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery
}
pub struct Card {
    name: String,
    mana_cost: String,
    colors: Vec<String>,
    color_identity: Vec<String>,
    _type: CardType,
    power: usize,
    toughness: usize,
    id: u32
}
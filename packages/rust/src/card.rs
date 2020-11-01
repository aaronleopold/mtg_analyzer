// TODO: add support for super/subtypes
#[derive(Copy, Clone, Debug)]
pub enum CardType {
    Land,
    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery
}

#[derive(Copy, Clone, Debug)]
pub enum ManaType {
    RED,
    GREEN,
    BLUE,
    BLACK,
    WHITE,
    MULTICOLORED,
    COLORLESS
}

#[derive(Clone)]
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

    // FIXME: this is disgusting lol
    // consider parsing in python prior to passing onto rust code
    pub fn mana_cost_parsed(&self) -> Vec<(ManaType, usize)>{
        let mut parsed_cost: Vec<(ManaType, usize)> = Vec::new();
        let mut new_string = String::from(&self.mana_cost);
        new_string = new_string.replace("{", "");
        new_string = new_string.replace("}", " ");
        new_string = String::from(new_string.trim());

        let mut str_iter = new_string.split_ascii_whitespace();
        let mut costs: Vec<(ManaType, usize)> = Vec::new();

        loop {
            let cost = match str_iter.next() {
                Some(_cost) => {
                    let parsed: Option<usize> = match _cost.parse() {
                        Ok(val) => Some(val),
                        _ => None
                    };

                    parsed
                },
                _ => None
            };

            // TODO: add all types
            let mana_type: Option<ManaType> = match str_iter.next() {
                Some(_type) => {
                    match _type {
                        "R" => Some(ManaType::RED),
                        "G" => Some(ManaType::RED),
                        "B" => Some(ManaType::RED),
                        "BL" => Some(ManaType::RED),
                        "W" => Some(ManaType::RED),
                        _ => None
                    }
                },
                _ => None
            };

            match cost {
                None => break,
                Some(_cost) => {
                    match mana_type {
                        None => break,
                        Some(_mana_type) => costs.push((_mana_type, _cost))
                    }
                }
            }
        }

        costs
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Card Name: {:?} Mana Cost:{:?}\n", self.name, self.mana_cost)
    }
}
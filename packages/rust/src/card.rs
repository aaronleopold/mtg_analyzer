use core::panic;

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

impl From<&str> for ManaType {
    fn from(mana_code: &str) -> ManaType {
        match mana_code {
            "R" => ManaType::RED,
            "G" => ManaType::GREEN,
            "B" => ManaType::BLUE,
            "BL" => ManaType::BLACK,
            "W" => ManaType::WHITE,
            _ => todo!("handle me")
            
        }
    }
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
            name,
            mana_cost,
            colors,
            color_identity,
            _type,
            power,
            toughness,
            id
        }
    }

    // FIXME/TODO: rework -> this is disgusting lol
    pub fn get_mana(&self) -> Vec<(ManaType, usize)>{
        // let mut parsed_cost: Vec<(ManaType, usize)> = Vec::new();
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
                Some(_type) => Some(ManaType::from(_type)),
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

    pub fn get_type(&self) -> CardType {
        self._type.clone()
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Card Name: {:?} Mana Cost:{:?}\n", self.name, self.mana_cost)
    }
}
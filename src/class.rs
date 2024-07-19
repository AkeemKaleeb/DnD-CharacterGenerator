#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Class {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard
}

impl Class {
    pub fn as_str(&self) -> &'static str {
        match self {
            Class::Artificer => "Artificer",
            Class::Barbarian => "Barbarian",
            Class::Bard => "Bard",
            Class::Cleric => "Cleric",
            Class::Druid => "Druid",
            Class::Fighter => "Fighter",
            Class::Monk => "Monk",
            Class::Paladin => "Paladin",
            Class::Ranger => "Ranger",
            Class::Rogue => "Rogue",
            Class::Sorcerer => "Sorcerer",
            Class::Warlock => "Warlock",
            Class::Wizard => "Wizard",
        }
    }

    pub fn all() -> Vec<Class> {
        vec![
            Class::Artificer,
            Class::Barbarian,
            Class::Bard,
            Class::Cleric,
            Class::Druid,
            Class::Fighter,
            Class::Monk,
            Class::Paladin,
            Class::Ranger,
            Class::Rogue,
            Class::Sorcerer,
            Class::Warlock,
            Class::Wizard
        ]
    }
}
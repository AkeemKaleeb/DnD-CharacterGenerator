use crate::race::Race;
use crate::class::Class;
use crate::equipment::{Armor, Weapon};
use crate::spells::Spell;
use crate::stats::{Stats, Skills};
use crate::descriptions::Description;
use std::fmt;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub level: u8,
    pub stats: Stats,
    pub skills: Vec<Skills>,
    pub armor: Vec<Armor>,
    pub weapons: Vec<Weapon>,
    pub spells: Vec<Spell>,
    pub description: Description,
}

impl Character {
    pub fn new(
        name: String,
        race: Race,
        class: Class,
        level: u8,
        stats: Stats,
        armor: Vec<Armor>,
        weapons: Vec<Weapon>,
        spells: Vec<Spell>,
        skills: Vec<Skills>,
        description: Description,
    ) -> Self {
        Self {
            name,
            race,
            class,
            level,
            stats,
            armor,
            weapons,
            spells,
            skills,
            description,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn set_race(&mut self, race: Race) {
        self.race = race;
    }

    pub fn set_class(&mut self, class: Class) {
        self.class = class;
    }

    pub fn set_background(&mut self, background: &str) {
        self.description.background = background.to_string();
    }

    pub fn set_appearance(&mut self, appearance: &str) {
        self.description.appearance = appearance.to_string();
    }

    pub fn set_personality(&mut self, personality: &str) {
        self.description.personality = personality.to_string();
    }

    pub fn add_spell(&mut self, spell: Spell) {
        self.spells.push(spell);
    }

    pub fn remove_spell(&mut self, spell_name: &str) {
        self.spells.retain(|spell| spell.name != spell_name);
    }

    pub fn add_armor(&mut self, armor: Armor) {
        self.armor.push(armor);
    }

    pub fn remove_armor(&mut self, armor_name: &str) {
        self.armor.retain(|armor| armor.name != armor_name);
    }

    pub fn add_weapon(&mut self, weapon: Weapon) {
        self.weapons.push(weapon);
    }

    pub fn remove_weapon(&mut self, weapon_name: &str) {
        self.weapons.retain(|weapon| weapon.name != weapon_name);
    }

    pub fn set_strength(&mut self, stat: u8)  {
        self.stats.strength = stat;
    }

    pub fn set_dexterity(&mut self, stat: u8)  {
        self.stats.dexterity = stat;
    }

    pub fn set_constitution(&mut self, stat: u8)  {
        self.stats.constitution = stat;
    }

    pub fn set_intelligence(&mut self, stat: u8)  {
        self.stats.intelligence = stat;
    }

    pub fn set_wisdom(&mut self, stat: u8)  {
        self.stats.wisdom = stat;
    }

    pub fn set_charisma(&mut self, stat: u8)  {
        self.stats.charisma = stat;
    }

    pub fn add_proficiency(&mut self, skills: Skills) {
        self.skills.push(skills);
    }

    pub fn remove_proficiency(&mut self, skills_name: &str) {
        self.skills.retain(|skills| skills.name != skills_name);
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("Name: {}", self.name);
        println!("Race: {:?}", self.race);
        println!("Class: {:?}", self.class);
        println!("Level: {}", self.level);
        println!("\nStats:\n{:?}", self.stats);
        println!("\nArmor:");
        for armor in &self.armor {
            println!("{:?}", armor);
        }
        println!("\nWeapons:");
        for weapon in &self.weapons {
            println!("{:?}", weapon);
        }
        println!("\nSpells:");
        for spell in &self.spells {
            println!("{:?}", spell);
        }
        println!("\nProficiencies:");
        for skill in &self.skills {
            println!("{:?}", skill);
        }
        println!("\nDescription:\n{:?}", self.description);

        Ok(())
    }
}
#[derive(Debug)]
pub struct Stats {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl Stats {
    pub fn calculate_mod(stat: u8) -> i8 {
        let num = (stat as f32 - 10 as f32) / 2 as f32;
        num.floor() as i8
    }
}

#[derive(Debug)]
pub struct Skills {
    pub name: String,
    pub bonus: u8,
}
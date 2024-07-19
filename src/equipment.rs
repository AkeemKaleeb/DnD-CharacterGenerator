#[derive(Debug)]
pub struct Armor {
    pub name: String,
    pub defense: u8,
}

#[derive(Debug)]
pub struct Weapon {
    pub name: String,
    pub damage: String,  // e.g., "1d8"
}
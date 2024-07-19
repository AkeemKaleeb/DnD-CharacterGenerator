#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Race {
    Aarakocra,
    Aasimar,
    Aasimar_Fallen,
    Aasimar_Protector,
    Aasimar_Scourge,
    Bugbear,
    Centaur,
    Changeling,
    Dragonborn,
    Dragonborn_Draconblood,
    Dragonborn_Ravenite,
    Dwarf_Duergar,
    Dwarf_Hill,
    Dwarf_MarkofWarding,
    Dwarf_Mountain,
    Elf_Aereni,
    Elf_Drow,
    Elf_Eladrin,
    Elf_High,
    Elf_MarkofShadow,
    Elf_Pallid,
    Elf_Sea,
    Elf_Shadarkai,
    Elf_Valenar,
    Elf_Wood,
    Firbolg,
    Genasi_Air,
    Genasi_Earth,
    Genasi_Fire,
    Genasi_Water,
    Gith_Githyanki,
    Gith_Githzerai,
    Gnome_Deep,
    Gnome_Forest,
    Gnome_MarkofScribing,
    Gnome_Rock,
    Goblin,
    Goblin_Dankwood,
    Goliath,
    Grung,
    HalfElf,
    HalfElf_Aquatic,
    HalfElf_Drow,
    HalfElf_High,
    HalfElf_MarkofDetection,
    HalfElf_MarkofStorm,
    HalfElf_Wood,
    HalfOrc,
    HalfOrc_MarkofFinding,
    Halfling_Ghostwise,
    Halfling_Lightfoot,
    Halfling_Lotusden,
    Halfling_MarkofHealing,
    Halfling_MarkofHospitality,
    Halfling_Stout,
    Hobgoblin,
    Human,
    Human_MarkofFinding,
    Human_MarkofHandling,
    Human_MarkofMaking,
    Human_MarkofPassage,
    Human_MarkofSentinel,
    Human_Variant,
    Kalashtar,
    Kenku,
    Kobold,
    Leonin,
    Lizardfolk,
    Locathah,
    Loxodon,
    Minotaur,
    Orc,
    Satyr,
    Shifter_Beasthide,
    Shifter_Longtooth,
    Shifter_Swiftstride,
    Shifter_Wildhunt,
    SimicHybrid,
    Tabaxi,
    Tiefling_Asmodeous,
    Tiefling_Baalzebul,
    Tiefling_Dispater,
    Tiefling_Fierna,
    Tiefling_Glasya,
    Tiefling_Levistus,
    Tiefling_Mammon,
    Tiefling_Mephistopheles,
    Tiefling_Variant,
    Tiefling_Zariel,
    Tortle,
    Triton,
    Vedalken,
    Verdan,
    Warforged,
    YuanTiPureblood,
}

impl Race {
    pub fn as_str(&self) -> &'static str {
        match self {
            Race::Aarakocra => "Aarakocra",
            Race::Aasimar => "Aasimar",
            Race::Aasimar_Fallen => "Aasimar Fallen",
            Race::Aasimar_Protector => "Aasimar Protector",
            Race::Aasimar_Scourge => "Aasimar Scourge",
            Race::Bugbear => "Bugbear",
            Race::Centaur => "Centaur",
            Race::Changeling => "Changeling",
            Race::Dragonborn => "Dragonborn",
            Race::Dragonborn_Draconblood => "Dragonborn Draconblood",
            Race::Dragonborn_Ravenite => "Dragonborn Ravenite",
            Race::Dwarf_Duergar => "Dwarf Duergar",
            Race::Dwarf_Hill => "Dwarf Hill",
            Race::Dwarf_MarkofWarding => "Dwarf Mark of Warding",
            Race::Dwarf_Mountain => "Dwarf Mountain",
            Race::Elf_Aereni => "Elf Aereni",
            Race::Elf_Drow => "Elf Drow",
            Race::Elf_Eladrin => "Elf Eladrin",
            Race::Elf_High => "Elf High",
            Race::Elf_MarkofShadow => "Elf Mark of Shadow",
            Race::Elf_Pallid => "Elf Pallid",
            Race::Elf_Sea => "Elf Sea",
            Race::Elf_Shadarkai => "Elf Shadarkai",
            Race::Elf_Valenar => "Elf Valenar",
            Race::Elf_Wood => "Elf Wood",
            Race::Firbolg => "Firbolg",
            Race::Genasi_Air => "Genasi Air",
            Race::Genasi_Earth => "Genasi Earth",
            Race::Genasi_Fire => "Genasi Fire",
            Race::Genasi_Water => "Genasi Water",
            Race::Gith_Githyanki => "Gith Githyanki",
            Race::Gith_Githzerai => "Gith Githzerai",
            Race::Gnome_Deep => "Gnome Deep",
            Race::Gnome_Forest => "Gnome Forest",
            Race::Gnome_MarkofScribing => "Gnome Mark of Scribing",
            Race::Gnome_Rock => "Gnome Rock",
            Race::Goblin => "Goblin",
            Race::Goblin_Dankwood => "Goblin Dankwood",
            Race::Goliath => "Goliath",
            Race::Grung => "Grung",
            Race::HalfElf => "Half-Elf",
            Race::HalfElf_Aquatic => "Half-Elf Aquatic",
            Race::HalfElf_Drow => "Half-Elf Drow",
            Race::HalfElf_High => "Half-Elf High",
            Race::HalfElf_MarkofDetection => "Half-Elf Mark of Detection",
            Race::HalfElf_MarkofStorm => "Half-Elf Mark of Storm",
            Race::HalfElf_Wood => "Half-Elf Wood",
            Race::HalfOrc => "Half-Orc",
            Race::HalfOrc_MarkofFinding => "Half-Orc Mark of Finding",
            Race::Halfling_Ghostwise => "Halfling Ghostwise",
            Race::Halfling_Lightfoot => "Halfling Lightfoot",
            Race::Halfling_Lotusden => "Halfling Lotusden",
            Race::Halfling_MarkofHealing => "Halfling Mark of Healing",
            Race::Halfling_MarkofHospitality => "Halfling Mark of Hospitality",
            Race::Halfling_Stout => "Halfling Stout",
            Race::Hobgoblin => "Hobgoblin",
            Race::Human => "Human",
            Race::Human_MarkofFinding => "Human Mark of Finding",
            Race::Human_MarkofHandling => "Human Mark of Handling",
            Race::Human_MarkofMaking => "Human Mark of Making",
            Race::Human_MarkofPassage => "Human Mark of Passage",
            Race::Human_MarkofSentinel => "Human Mark of Sentinel",
            Race::Human_Variant => "Human Variant",
            Race::Kalashtar => "Kalashtar",
            Race::Kenku => "Kenku",
            Race::Kobold => "Kobold",
            Race::Leonin => "Leonin",
            Race::Lizardfolk => "Lizardfolk",
            Race::Locathah => "Locathah",
            Race::Loxodon => "Loxodon",
            Race::Minotaur => "Minotaur",
            Race::Orc => "Orc",
            Race::Satyr => "Satyr",
            Race::Shifter_Beasthide => "Shifter Beasthide",
            Race::Shifter_Longtooth => "Shifter Longtooth",
            Race::Shifter_Swiftstride => "Shifter Swiftstride",
            Race::Shifter_Wildhunt => "Shifter Wildhunt",
            Race::SimicHybrid => "Simic Hybrid",
            Race::Tabaxi => "Tabaxi",
            Race::Tiefling_Asmodeous => "Tiefling Asmodeous",
            Race::Tiefling_Baalzebul => "Tiefling Baalzebul",
            Race::Tiefling_Dispater => "Tiefling Dispater",
            Race::Tiefling_Fierna => "Tiefling Fierna",
            Race::Tiefling_Glasya => "Tiefling Glasya",
            Race::Tiefling_Levistus => "Tiefling Levistus",
            Race::Tiefling_Mammon => "Tiefling Mammon",
            Race::Tiefling_Mephistopheles => "Tiefling Mephistopheles",
            Race::Tiefling_Variant => "Tiefling Variant",
            Race::Tiefling_Zariel => "Tiefling Zariel",
            Race::Tortle => "Tortle",
            Race::Triton => "Triton",
            Race::Vedalken => "Vedalken",
            Race::Verdan => "Verdan",
            Race::Warforged => "Warforged",
            Race::YuanTiPureblood => "Yuan-Ti Pureblood",
        }
    }

    pub fn all() -> Vec<Race> {
        vec![
            Race::Aarakocra,
            Race::Aasimar,
            Race::Aasimar_Fallen,
            Race::Aasimar_Protector,
            Race::Aasimar_Scourge,
            Race::Bugbear,
            Race::Centaur,
            Race::Changeling,
            Race::Dragonborn,
            Race::Dragonborn_Draconblood,
            Race::Dragonborn_Ravenite,
            Race::Dwarf_Duergar,
            Race::Dwarf_Hill,
            Race::Dwarf_MarkofWarding,
            Race::Dwarf_Mountain,
            Race::Elf_Aereni,
            Race::Elf_Drow,
            Race::Elf_Eladrin,
            Race::Elf_High,
            Race::Elf_MarkofShadow,
            Race::Elf_Pallid,
            Race::Elf_Sea,
            Race::Elf_Shadarkai,
            Race::Elf_Valenar,
            Race::Elf_Wood,
            Race::Firbolg,
            Race::Genasi_Air,
            Race::Genasi_Earth,
            Race::Genasi_Fire,
            Race::Genasi_Water,
            Race::Gith_Githyanki,
            Race::Gith_Githzerai,
            Race::Gnome_Deep,
            Race::Gnome_Forest,
            Race::Gnome_MarkofScribing,
            Race::Gnome_Rock,
            Race::Goblin,
            Race::Goblin_Dankwood,
            Race::Goliath,
            Race::Grung,
            Race::HalfElf,
            Race::HalfElf_Aquatic,
            Race::HalfElf_Drow,
            Race::HalfElf_High,
            Race::HalfElf_MarkofDetection,
            Race::HalfElf_MarkofStorm,
            Race::HalfElf_Wood,
            Race::HalfOrc,
            Race::HalfOrc_MarkofFinding,
            Race::Halfling_Ghostwise,
            Race::Halfling_Lightfoot,
            Race::Halfling_Lotusden,
            Race::Halfling_MarkofHealing,
            Race::Halfling_MarkofHospitality,
            Race::Halfling_Stout,
            Race::Hobgoblin,
            Race::Human,
            Race::Human_MarkofFinding,
            Race::Human_MarkofHandling,
            Race::Human_MarkofMaking,
            Race::Human_MarkofPassage,
            Race::Human_MarkofSentinel,
            Race::Human_Variant,
            Race::Kalashtar,
            Race::Kenku,
            Race::Kobold,
            Race::Leonin,
            Race::Lizardfolk,
            Race::Locathah,
            Race::Loxodon,
            Race::Minotaur,
            Race::Orc,
            Race::Satyr,
            Race::Shifter_Beasthide,
            Race::Shifter_Longtooth,
            Race::Shifter_Swiftstride,
            Race::Shifter_Wildhunt,
            Race::SimicHybrid,
            Race::Tabaxi,
            Race::Tiefling_Asmodeous,
            Race::Tiefling_Baalzebul,
            Race::Tiefling_Dispater,
            Race::Tiefling_Fierna,
            Race::Tiefling_Glasya,
            Race::Tiefling_Levistus,
            Race::Tiefling_Mammon,
            Race::Tiefling_Mephistopheles,
            Race::Tiefling_Variant,
            Race::Tiefling_Zariel,
            Race::Tortle,
            Race::Triton,
            Race::Vedalken,
            Race::Verdan,
            Race::Warforged,
            Race::YuanTiPureblood,
        ]
    }

}

impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
use eframe::{egui, egui::ComboBox, epi};

mod character;
mod race;
mod class;
mod equipment;
mod spells;
mod stats;
mod descriptions;

use character::Character;
use race::Race;
use class::Class;
use equipment::{Armor, Weapon};
use spells::Spell;
use stats::{Skills, Stats};
use descriptions::Description;


fn main() {
    let app = MyApp::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(app),
        options,
    );
}

struct MyApp {
    character: Character,
}

impl Default for MyApp {
    fn default() -> Self {
        let race = Race::Human;
        let class = Class::Fighter;
        let stats = Stats {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        };
        let description = Description {
            background: String::from(""),
            appearance: String::from(""),
            personality: String::from(""),
        };
        Self {
            character: Character::new(
                String::from(""),
                race,
                class,
                1,
                stats,
                vec![],
                vec![],
                vec![],
                vec![],
                description,
            ),
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("D&D Character Creator");

            ui.label("Name:");
            ui.text_edit_singleline(&mut self.character.name);

            ui.label("Race:");
            egui::ComboBox::from_label("Select Race")
                .selected_text(self.character.race.as_str())
                .show_ui(ui, |ui| {
                    for race in Race::all() {
                        ui.selectable_value(&mut self.character.race, race, format!("{}", race.as_str()));
                    }
                });

            ui.label("Class:");
            egui::ComboBox::from_label("Select Class")
                .selected_text(self.character.class.as_str())
                .show_ui(ui, |ui| {
                    for class in Class::all() {
                        ui.selectable_value(&mut self.character.class, class, format!("{}", class.as_str()));
                    }
                });

            ui.label("Level:");
            ui.add(egui::Slider::new(&mut self.character.level, 1..=20));

            ui.label("Stats:");
            egui::Grid::new("stats_grid")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Strength");
                    ui.add(egui::DragValue::new(&mut self.character.stats.strength));
                    ui.add(egui::Label::new(Stats::calculate_mod(self.character.stats.strength)));
                    ui.end_row();

                    ui.label("Dexterity");
                    ui.add(egui::DragValue::new(&mut self.character.stats.dexterity));
                    ui.add(egui::Label::new(Stats::calculate_mod(self.character.stats.dexterity)));
                    ui.end_row();

                    ui.label("Constitution");
                    ui.add(egui::DragValue::new(&mut self.character.stats.constitution));
                    ui.add(egui::Label::new(Stats::calculate_mod(self.character.stats.constitution)));
                    ui.end_row();

                    ui.label("Intelligence");
                    ui.add(egui::DragValue::new(&mut self.character.stats.intelligence));
                    ui.add(egui::Label::new(Stats::calculate_mod(self.character.stats.intelligence)));
                    ui.end_row();

                    ui.label("Wisdom");
                    ui.add(egui::DragValue::new(&mut self.character.stats.wisdom));
                    ui.add(egui::Label::new(Stats::calculate_mod(self.character.stats.wisdom)));
                    ui.end_row();

                    ui.label("Charisma");
                    ui.add(egui::DragValue::new(&mut self.character.stats.charisma));
                    ui.add(egui::Label::new(Stats::calculate_mod(self.character.stats.charisma)));
                    ui.end_row();
                });

            ui.label("Description:");
            ui.label("Background:");
            ui.text_edit_multiline(&mut self.character.description.background);

            ui.label("Appearance:");
            ui.text_edit_multiline(&mut self.character.description.appearance);

            ui.label("Personality:");
            ui.text_edit_multiline(&mut self.character.description.personality);

            // Implement armor, weapons, spells, and proficiencies UI

            if ui.button("Print Character").clicked() {
                println!("{}", self.character);
            }
        });
    }

    fn name(&self) -> &str {
        "D&D Character Creator"
    }
}
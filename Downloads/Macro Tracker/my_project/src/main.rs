use eframe::egui;
use std::fs::File;
use std::io::{Write, Read};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct FoodEntry {
    name: String,
    calories: i32,
    protein: i32,
    carbs: i32,
    fat: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct MacroApp {
    // Current Goals
    calorie_goal: i32,
    protein_goal: i32,
    carbs_goal: i32,
    fat_goal: i32,
    
    history: Vec<FoodEntry>,
    
    #[serde(skip)]
    food_name: String,
    #[serde(skip)]
    food_calories: i32,
    #[serde(skip)]
    food_protein: i32,
    #[serde(skip)]
    food_carbs: i32,
    #[serde(skip)]
    food_fat: i32,
}

impl Default for MacroApp {
    fn default() -> Self {
        Self {
            calorie_goal: 2000,
            protein_goal: 150,
            carbs_goal: 200,
            fat_goal: 70,
            history: Vec::new(),
            food_name: "".to_owned(),
            food_calories: 0,
            food_protein: 0,
            food_carbs: 0,
            food_fat: 0,
        }
    }
}

impl MacroApp {
    fn save_data(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            if let Ok(mut file) = File::create("data.json") {
                let _ = file.write_all(json.as_bytes());
            }
        }
    }

    fn load_data() -> Self {
        let mut file_content = String::new();
        if let Ok(mut file) = File::open("data.json") {
            if file.read_to_string(&mut file_content).is_ok() {
                if let Ok(data) = serde_json::from_str(&file_content) {
                    return data;
                }
            }
        }
        Self::default()
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Macro Tracker Pro",
        options,
        Box::new(|_cc| Ok(Box::new(MacroApp::load_data()))),
    )
}

impl eframe::App for MacroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate current totals from history
        let total_cals: i32 = self.history.iter().map(|f| f.calories).sum();
        let total_p: i32 = self.history.iter().map(|f| f.protein).sum();
        let total_c: i32 = self.history.iter().map(|f| f.carbs).sum();
        let total_f: i32 = self.history.iter().map(|f| f.fat).sum();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎯 Daily Goals");
            
            // --- Goal Setting Section ---
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Calories");
                        if ui.add(egui::DragValue::new(&mut self.calorie_goal)).changed() { self.save_data(); }
                    });
                    ui.vertical(|ui| {
                        ui.label("Protein (g)");
                        if ui.add(egui::DragValue::new(&mut self.protein_goal)).changed() { self.save_data(); }
                    });
                    ui.vertical(|ui| {
                        ui.label("Carbs (g)");
                        if ui.add(egui::DragValue::new(&mut self.carbs_goal)).changed() { self.save_data(); }
                    });
                    ui.vertical(|ui| {
                        ui.label("Fat (g)");
                        if ui.add(egui::DragValue::new(&mut self.fat_goal)).changed() { self.save_data(); }
                    });
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.heading("📊 Progress");

            // --- Status Table ---
            egui::Grid::new("stat_grid").spacing([40.0, 10.0]).show(ui, |ui| {
                ui.label("Macro");
                ui.label("Current");
                ui.label("Goal");
                ui.label("Remaining");
                ui.end_row();

                let macros = [
                    ("Calories", total_cals, self.calorie_goal, "kcal"),
                    ("Protein", total_p, self.protein_goal, "g"),
                    ("Carbs", total_c, self.carbs_goal, "g"),
                    ("Fat", total_f, self.fat_goal, "g"),
                ];

                for (name, current, goal, unit) in macros {
                    ui.label(name);
                    ui.label(format!("{}{}", current, unit));
                    ui.label(format!("{}{}", goal, unit));
                    
                    let remaining = goal - current;
                    let color = if remaining < 0 { egui::Color32::LIGHT_RED } else { egui::Color32::LIGHT_GREEN };
                    ui.label(egui::RichText::new(format!("{}{}", remaining, unit)).color(color));
                    ui.end_row();
                }
            });

            ui.separator();
            ui.add_space(10.0);

            // --- Input Section ---
            ui.label("Add New Entry:");
            ui.add(egui::TextEdit::singleline(&mut self.food_name).hint_text("Food name..."));

            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.food_calories).prefix("Cals: "));
                ui.add(egui::DragValue::new(&mut self.food_protein).prefix("P: "));
                ui.add(egui::DragValue::new(&mut self.food_carbs).prefix("C: "));
                ui.add(egui::DragValue::new(&mut self.food_fat).prefix("F: "));
            });

            if ui.button("➕ Log Food").clicked() && !self.food_name.is_empty() {
                self.history.push(FoodEntry {
                    name: self.food_name.clone(),
                    calories: self.food_calories,
                    protein: self.food_protein,
                    carbs: self.food_carbs,
                    fat: self.food_fat,
                });
                self.food_name.clear();
                self.food_calories = 0; self.food_protein = 0; self.food_carbs = 0; self.food_fat = 0;
                self.save_data();
            }

            ui.separator();

            // --- History Section ---
            egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                let mut to_remove = None;
                for (i, entry) in self.history.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button("❌").clicked() { to_remove = Some(i); }
                        ui.label(format!("{}: {}kcal (P:{} C:{} F:{})", 
                            entry.name, entry.calories, entry.protein, entry.carbs, entry.fat));
                    });
                }
                if let Some(index) = to_remove {
                    self.history.remove(index);
                    self.save_data();
                }
            });

            if ui.button("Reset Day").clicked() {
                self.history.clear();
                self.save_data();
            }
        });
    }
}
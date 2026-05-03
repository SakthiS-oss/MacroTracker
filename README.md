🍎 Rust Macro Tracker
A lightweight, high-performance desktop application for tracking daily calories and macronutrients (Protein, Carbs, and Fats). Built with Rust and the egui framework.

✨ Features
Real-time Tracking: See your remaining calories and macros update instantly as you add food.

Custom Goals: Set specific daily targets for Calories, Protein, Carbohydrates, and Fats.

Persistent Storage: All data is automatically saved to a local data.json file. Your progress and goals are right where you left them when you restart the app.

Clean UI: Simple, distraction-free interface with a scrollable history log and easy "drag-to-edit" values.

Safety First: Built with Rust’s memory safety guarantees to ensure a crash-free experience.

🚀 Getting Started
Prerequisites
Rust Toolchain: Install from rustup.rs.

C++ Build Tools (Windows only): Ensure you have the "Desktop development with C++" workload installed via the Visual Studio Installer.

Installation & Running
Clone or download this repository.

Open a terminal in the project folder.

Run the application:

Bash
cargo run
🛠️ Tech Stack
Language: Rust

GUI Framework: eframe/egui (Immediate mode GUI)

Serialization: Serde & Serde JSON

📂 Data Storage
The app saves your data in a human-readable data.json file in the root directory. It looks like this:

JSON
{
  "calorie_goal": 2000,
  "protein_goal": 150,
  "carbs_goal": 200,
  "fat_goal": 70,
  "history": [
    {
      "name": "Chicken Breast",
      "calories": 165,
      "protein": 31,
      "carbs": 0,
      "fat": 3
    }
  ]
}

📜 License
This project is open-source and available under the MIT License.

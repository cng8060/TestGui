#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, Frame};
use egui::{Context};
use rand::prelude::*;

struct GuessingGame {
    ai_guess: i32,
    usr_guess: i32,
    usr_input: String
}

impl Default for GuessingGame {
    fn default() -> Self {
        let mut rng = thread_rng();

        Self {
            ai_guess: rng.gen_range(1..101),
            usr_guess: -1,
            usr_input: String::from("Enter Guess Here")
        }
    }
}

impl eframe::App for GuessingGame {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let mut on_box = true;

                // Guessing/Input area
                ui.horizontal(|ui| {
                    ui.heading("Guessing Game!");
                    ui.label("Guess my number between 1-100: ");
                    let user_num = ui.add_sized(ui.min_size(), egui::TextEdit::singleline(&mut self.usr_input));
                    on_box = user_num.has_focus();
                });

                // Response to input
                ui.horizontal(|ui| {
                    if self.usr_input.parse::<i32>().is_ok() {
                        self.usr_guess = self.usr_input.parse::<i32>().unwrap();
                    }

                    if !on_box {
                        if self.usr_guess == self.ai_guess {
                            ui.heading("Congrats!");
                            ui.label(format!("The number was {}!", self.ai_guess));
                        } else {
                            ui.heading("Nope!");

                            if self.usr_guess > self.ai_guess {
                                ui.label(format!("The number is less than {}!", self.usr_guess));
                            } else {
                                ui.label(format!("The number is greater than {}!", self.usr_guess));
                            }
                        }
                    }
                });
            });
        });
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Try to Win!",
        native_options,
        Box::new(|_cc| Box::new(GuessingGame::default())),
    );
}

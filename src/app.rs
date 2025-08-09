use eframe::App;
use egui::{self, RichText};
use std::path::PathBuf;

use crate::{
    audio::play_chime,
    core::{config::Config, timer::Timer},
    persistence::{load_config, save_config},
};

pub struct PomodoroApp {
    cfg_path: PathBuf,
    pub cfg: Config,
    pub timer: Timer,
}

impl PomodoroApp {
    pub fn new(cfg_path: PathBuf) -> Self {
        let cfg = load_config(&cfg_path).unwrap_or_default();
        let timer = Timer::new(&cfg);
        Self { cfg_path, cfg, timer}
    }

    fn handle_tick_and_transitions(&mut self) {
        if self.timer.tick() {
            // phase finished
            let _ = play_chime();
            self.timer.advance_phase(&self.cfg);
        }
    }
}

impl App for PomodoroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_tick_and_transitions();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.heading("Chronodoro");
                ui.add_space(12.0);

                ui.label(RichText::new(self.timer.phase.label()).size(18.0));
                ui.add_space(6.0);

                ui.label(RichText::new(Timer::format_mmss(self.timer.seconds_left)).size(48.0).monospace());
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button(if self.timer.running { "Pause" } else { "Start" }).clicked() { self.timer.running = !self.timer.running; self.timer.last_tick = std::time::Instant::now(); }
                    if ui.button("Reset").clicked() { self.timer.reset_phase(&self.cfg, self.timer.phase); }
                    if ui.button("Skip").clicked() { self.timer.advance_phase(&self.cfg); }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                ui.heading("Settings");
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label("Focus (min)");
                    ui.add(egui::DragValue::new(&mut self.cfg.focus_minutes).range(1..=120));
                    ui.label("Short break");
                    ui.add(egui::DragValue::new(&mut self.cfg.short_break_minutes).range(1..=60));
                    ui.label("Long break");
                    ui.add(egui::DragValue::new(&mut self.cfg.long_break_minutes).range(1..=120));
                });
                ui.horizontal(|ui| {
                    ui.label("Sessions until long break");
                    ui.add(egui::DragValue::new(&mut self.cfg.sessions_until_long_break).range(1..=12));
                });
                ui.checkbox(&mut self.cfg.auto_start_next, "Auto-start next phase");

                ui.add_space(6.0);
                if ui.button("Save settings").clicked() {
                    let _ = save_config(&self.cfg_path, &self.cfg);
                    if !self.timer.running {
                        self.timer.reset_phase(&self.cfg, self.timer.phase);
                    }
                }

                ui.add_space(8.0);
                ui.label("Tip: Put this EXE on a USB drive. A \"pomodoro_config.json\" file will be created next to it.");
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}
use eframe::egui;
use std::{ops::Not};

fn main() {
    let native_options = eframe::NativeOptions { // For if I want to change stuff here later
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "To-Do List",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(3.0);
            Ok(Box::new(TODOEguiApp::new(cc)))
        })
    ).expect("An error occured");
}

#[derive(Debug, Copy, Clone)]
enum TaskStatus {
    Complete,
    Incomplete
}

impl Not for TaskStatus {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self{
            TaskStatus::Complete => TaskStatus::Incomplete,
            TaskStatus::Incomplete => TaskStatus::Complete
        }
    }
}

struct Task {
    name: String,
    status: TaskStatus
}

#[derive(Default)]
struct TODOEguiApp {
    task_buf: String,
    tasks: Vec<Task>
}

impl TODOEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TODOEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("To-Do App");
            let task_input = ui.text_edit_singleline(&mut self.task_buf);
            let enter = task_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button("Add").clicked() || enter {
                self.tasks.push(Task{
                    name: self.task_buf.clone(),
                    status: TaskStatus::Incomplete
                });
                self.task_buf.clear();
            }
            let mut task_index: usize = 0;
            let mut to_remove: Vec<usize> = vec![];
            let mut to_toggle: Vec<usize> = vec![];
            for task in &mut self.tasks {
                ui.horizontal(|ui| {
                    ui.label(format!("{} - {:?}", task.name, task.status));
                    if ui.button("Delete").clicked() {
                        to_remove.push(task_index);
                    }
                    if ui.button(format!("Toggle Status")).clicked() {
                        to_toggle.push(task_index);
                    }
                });
                task_index += 1;
            }
            for index_to_remove in to_remove {
                self.tasks.remove(index_to_remove);
            }
            for index_to_remove in to_toggle {
                self.tasks[index_to_remove].status = !self.tasks[index_to_remove].status;
            }
        });
    }
}
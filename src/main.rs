use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "To-Do List",
        native_options,
        Box::new(|cc| Ok(Box::new(TODOEguiApp::new(cc))))
    ).expect("An error occured");
}

#[derive(Default)]
struct TODOEguiApp {
    task_buf: String,
    tasks: Vec<(String, bool)>
}

impl TODOEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TODOEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("To-Do App");
            ui.text_edit_singleline(&mut self.task_buf);
            if ui.button("Add").clicked() {
                self.tasks.push((self.task_buf.clone(), false));
            }
            let mut task_index: usize = 0;
            let mut to_remove: Vec<usize> = vec![];
            for task in &mut self.tasks {
                ui.horizontal(|ui| {
                    ui.label(format!("{} - {}", task.0, task.1));
                    if ui.button("Delete").clicked() {
                        to_remove.push(task_index);
                    }
                });
                task_index += 1;
            }
            for index_to_remove in to_remove {
                self.tasks.remove(index_to_remove);
            }
        });
    }
}
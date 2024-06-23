use eframe::egui;
use rfd::FileDialog;
use shannon_entropy::shannon_entropy;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "File Entropy Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    file_path: Option<String>,
    entropy: Option<f64>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_path: None,
            entropy: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Entropy Calculator");

            if ui.button("Upload File").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file_path = Some(path.display().to_string());
                    self.entropy = calculate_entropy(&path.display().to_string());
                }
            }

            if let Some(file_path) = &self.file_path {
                ui.label(format!("Selected file: {}", file_path));
            }

            if let Some(entropy) = self.entropy {
                ui.label(format!("File Entropy: {:.6} bits/byte", entropy));
            }
        });
    }
}

fn calculate_entropy(file_path: &str) -> Option<f64> {
    let mut file = File::open(file_path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;
    let hex_string: String = buffer.iter().map(|b| format!("{:02x}", b)).collect();
    Some(f64::from(shannon_entropy(&hex_string)))
}

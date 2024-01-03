use egui::Widget;
use egui_plot::{Line, PlotPoints};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    cylinder_count: u32,
    arm_position_int: u32,
    sequence: Vec<u32>,
    sequence_count: u32,
    open_panel: Panel,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Panel {
    SSTF,
    SCAN,
    CSCAN,
    CLOOK,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            cylinder_count: 0,
            arm_position_int: 0,
            sequence: vec![0],
            sequence_count: 0,
            open_panel: Panel::SSTF,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
pub fn ordered_by_closeness(input: &Vec<u32>, base: u32) -> Vec<u32> {
    let mut cloned_input = input.clone();
    cloned_input.sort_by_key(|&x| (x as i32 - base as i32).abs());
    cloned_input
}
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Disk Configuration");

            egui::Grid::new("disk_setting_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Total Cylinder");
                    ui.add(egui::Slider::new(&mut self.cylinder_count, 0..=1000));

                    ui.end_row();

                    ui.label("Arm Position");
                    ui.add(egui::Slider::new(
                        &mut self.arm_position_int,
                        0..=self.cylinder_count,
                    ));
                });

            ui.separator();

            ui.heading("Sequence Configuration");
            egui::Grid::new("sequence_setting_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for item in &mut self.sequence {
                        ui.add(egui::Slider::new(item, 0..=self.cylinder_count).text("Sequence"));
                        ui.end_row();
                    }

                    if &self.sequence.len() == &0 {
                        ui.label("Empty Sequence");
                        ui.end_row();
                    }

                    if ui.button("Add Sequence").clicked() {
                        self.sequence.push(0);
                    };
                    if ui.button("Remove Sequence").clicked() {
                        self.sequence.pop();
                    };
                    ui.end_row();
                });

            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.open_panel,
                    Panel::SSTF,
                    "Shortest Seek Time First",
                );
                ui.selectable_value(&mut self.open_panel, Panel::SCAN, "Scan");
                ui.selectable_value(&mut self.open_panel, Panel::CSCAN, "Circular Scan");
                ui.selectable_value(&mut self.open_panel, Panel::CLOOK, "Circular Look");
            });

            match self.open_panel {
                Panel::SSTF => {
                    egui_plot::Plot::new("SSTF")
                        .y_axis_width(2)
                        .data_aspect(1.0)
                        .show(ui, |plot_ui| {
                            let new_seq = TemplateApp::ordered_by_closeness(&mut self.sequence, self.arm_position_int);
                            for (i, el) in  new_seq.iter().enumerate(){
                                if i == 0 {
                                    plot_ui.line(Line::new(PlotPoints::new(vec![
                                        [self.arm_position_int as f64, 0.0],
                                        [new_seq[i].to_owned() as f64, -5.0],
                                    ])));
                                } else {
                                    let prev_y = -5.0 * i as f64;
                                    plot_ui.line(Line::new(PlotPoints::new(vec![
                                        [new_seq[i - 1].to_owned() as f64, -5.0 * i as f64],
                                        [el.to_owned() as f64, prev_y - 5.0],
                                    ])));
                                }
                            }
                        });
                }
                Panel::SCAN => {
                    ui.label("Scan");
                }
                Panel::CSCAN => {
                    ui.label("C-Scan");
                }
                Panel::CLOOK => {
                    ui.label("C-Look");
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

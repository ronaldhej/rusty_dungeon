use bevy::{prelude::*, ui::UI_MATERIAL_SHADER_HANDLE};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MyApp {
    selected_file_input: SelectedPathInput,
    binary_path: Option<String>,
    script_path: Option<String>,
    output_path: Option<String>,
}

#[derive(PartialEq)]
enum SelectedPathInput {
    Binary,
    Script,
    Output,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, file_picker_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(MyApp {
        selected_file_input: SelectedPathInput::Binary,
        binary_path: None,
        script_path: None,
        output_path: None,
    });
}

fn file_picker_system(mut contexts: EguiContexts, mut query: Query<&mut MyApp>) {
    let mut my_app = query.single_mut();

    egui::Window::new("Choose file").show(contexts.ctx_mut(), |ui| {

        ui.label("Which file would you like to select?");
        //Radio menu
        ui.horizontal(|ui| {
        ui.radio_value(
            &mut my_app.selected_file_input,
            SelectedPathInput::Binary,
            "Binary",
        );
        ui.radio_value(
            &mut my_app.selected_file_input,
            SelectedPathInput::Script,
            "Script",
        );
        ui.radio_value(
            &mut my_app.selected_file_input,
            SelectedPathInput::Output,
            "Output",
        );
        });
        ui.label("Select a file");

        //File picker
        if ui.button("Open file...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                match &my_app.selected_file_input {
                    SelectedPathInput::Binary => {
                        my_app.binary_path = Some(path.display().to_string())
                    }
                    SelectedPathInput::Script => {
                        my_app.script_path = Some(path.display().to_string())
                    }
                    SelectedPathInput::Output => {
                        my_app.output_path = Some(path.display().to_string())
                    }
                    _ => println!("Invalid radio option"),
                }
            }
        }

        if let Some(binary_path) = &my_app.binary_path {
            ui.horizontal(|ui| {
                ui.label("Binary file:");
                ui.monospace(binary_path);
            });
        };
        if let Some(script_path) = &my_app.script_path {
            ui.horizontal(|ui| {
                ui.label("Script file:");
                ui.monospace(script_path);
            });
        };
        if let Some(output_path) = &my_app.output_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(output_path);
            });
        };
    });
}

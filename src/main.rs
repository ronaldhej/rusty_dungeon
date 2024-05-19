use bevy::{
    log::tracing_subscriber::fmt::FormattedFields, prelude::*, ui::UI_MATERIAL_SHADER_HANDLE,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use serde::{ser::Error, Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::Borrow, collections::HashMap, process::Command};

#[derive(Component)]
struct MyApp {
    selected_file_input: SelectedPathInput,
    binary_path: Option<String>,
    script_path: Option<String>,
    python_path: Option<String>,
    map: Option<HashMap<String, Room>>,
}

#[derive(PartialEq)]
enum SelectedPathInput {
    Python,
    Binary,
    Script,
}

#[derive(Serialize, Deserialize, Debug)]
struct Room {
    layers: Layer,
}

#[derive(Serialize, Deserialize, Debug)]
struct Layer {
    terrain: Vec<Vec<String>>,
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
        selected_file_input: SelectedPathInput::Python,
        binary_path: None,
        script_path: None,
        python_path: None,
        map: Some(HashMap::new()),
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
                SelectedPathInput::Python,
                "Python",
            );
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
                    SelectedPathInput::Python => {
                        my_app.python_path = Some(path.display().to_string())
                    }
                    _ => println!("Invalid radio option"),
                }
            }
        }

        if ui.button("Run binary").clicked() {
            match (
                &my_app.python_path,
                &my_app.binary_path,
                &my_app.script_path,
            ) {
                (Some(python_path), Some(binary_path), Some(script_path)) => {
                    let result: Result<String, std::string::FromUtf8Error> =
                        run_binary(python_path, binary_path, script_path);
                    match result {
                        Ok(result) => {
                            let (name, room) = parse_output_to_room(&result);
                            my_app.map.as_mut().unwrap().insert(name, room);
                            println!("Hashmap values: {:?}", my_app.map);
                        }
                        Err(e) => println!("Error: {e}"),
                    }
                }
                _ => {
                    println!("Some paths are missing, script cannot be run")
                }
            }
        }

        if let Some(python_path) = &my_app.python_path {
            ui.horizontal(|ui| {
                ui.label("Python file:");
                ui.monospace(python_path);
            });
        };

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

        if let (Some(binary_path), Some(script_path)) = (&my_app.binary_path, &my_app.script_path) {
            ui.horizontal(|ui| {
                ui.label("Command to be run: ");
                ui.monospace(format!("python3 {binary_path} -p {script_path}"))
            });
        }
    });
}

fn run_binary(
    python_path: &str,
    binary_path: &str,
    script_path: &str,
) -> Result<String, std::string::FromUtf8Error> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(python_path)
            .arg(binary_path)
            .arg("-p")
            .arg(script_path)
            .output()
            .expect("failed to execute process")
    };

    if output.status.success() {
        return String::from_utf8(output.stdout);
    } else {
        return String::from_utf8(output.stderr);
    }
}

fn parse_output_to_room(output: &str) -> (String, Room) {
    let value: Value = serde_json::from_str(output).unwrap();
    let (room_name, room_content) = value
        .as_object()
        .and_then(|obj| obj.into_iter().next())
        .expect("Expected a single map key and value");

    let room: Room = serde_json::from_value(room_content.clone()).expect("Deserialization failed");

    return (room_name.to_string(), room);
}

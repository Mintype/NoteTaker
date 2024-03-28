slint::include_modules!();

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use nfd::Response;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    // Function for when user clicks 'open' button.
    ui.on_open_note({ 
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("User has clicked: open");

            // Open file dialog
            if let Some((file_path, file_name, file_contents)) = open_file_dialog() {

                // Print out information about the file the user has selected.
                println!("User selected file: {:?}", file_path);
                println!("File name: {}", file_name);
                println!("File contents:\n{}", file_contents);

                // Set UI to the file title & contents.
                ui.set_file_title(file_name.into());
                ui.set_file_contents(file_contents.into());
            }

        }
    });

    // Function for when user clicks 'save' button.
    ui.on_save_note({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("User has clicked: save");
        }
    });

    // Function for when user clicks 'new' button.
    ui.on_new_note({ 
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("User has clicked: new");
            // Clear file title and file contents.
            ui.set_file_title("".into());
            ui.set_file_contents("".into());
        }
    });

    ui.run()
}

// Function to open file dialog and read file contents.
fn open_file_dialog() -> Option<(PathBuf, String, String)> {

    if let Ok(Response::Okay(file_path)) = nfd::open_file_dialog(None, None) {
        // User selects a file.
        let path_buf = PathBuf::from(&file_path);
        // Get the file name from the file path.
        let file_name = path_buf.file_name()?.to_string_lossy().into_owned();

        // Read the file contents.
        let mut file_contents = String::new();
        if let Ok(mut file) = File::open(&path_buf) {
            if let Ok(_) = file.read_to_string(&mut file_contents) {
                // Normalize line endings
                file_contents = normalize_line_endings(&file_contents);
                return Some((path_buf, file_name, file_contents));
            }
        }
    }
    // If User canceled or closed the dialog, or an error occurred, return None.
    None
}

// Function to normalize line endings.
fn normalize_line_endings(text: &str) -> String {
    // Replace '\r\n' with '\n' to normalize line endings.
    text.replace("\r\n", "\n")
}
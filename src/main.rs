slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    ui.on_open_note({ 
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("User has clicked: open");
        }
    });

    ui.on_save_note({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("User has clicked: save");
        }
    });

    ui.on_new_note({ 
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("User has clicked: new");
        }
    });

    ui.run()
}

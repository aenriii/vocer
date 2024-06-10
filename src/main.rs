use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let palletes = vec!["mocha", "macchiato", "frappe", "latte"];
    let mut i = 0;
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
            i+=1;
            ui.set_pallete(SharedString::from(palletes[i % 4]))
        }
    });

    ui.run()
}

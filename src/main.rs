slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // ui.on_request_increase_value({
    // let ui_handle: Weak<AppWindow> =  ui.as_weak();

    ui.run()

}

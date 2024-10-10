// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// 引進自定義的widgets, .slint路徑在build.rs中自定
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 此處的MainWindow widget是由slint中export的元件
    let main_window = MainWindow::new()?;
    main_window.run()
}
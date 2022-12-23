use eform::EformApp;
use eframe::{run_native, NativeOptions};

fn main() {
    run_native(
        "eform",
        NativeOptions::default(),
        Box::new(|_| Box::new(EformApp::new())),
    );
}

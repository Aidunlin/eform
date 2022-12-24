fn main() {
    eframe::run_native(
        "eform",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(eform::EformApp::new())),
    );
}

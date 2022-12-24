fn main() {
    eframe::run_native(
        "eform",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(eform::EformApp::new(cc))),
    );
}

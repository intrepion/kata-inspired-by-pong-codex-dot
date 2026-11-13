mod backend;
use backend::Backend;
use qtbridge::QApp;
fn main() {
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("Main.qml"))
        .run();
}

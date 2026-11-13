//! Separate automated test renderer. Uses the production QML and actual Backend slots.
mod backend;
use backend::Backend;
use qtbridge::QApp;
fn main() {
    let main = include_str!("Main.qml");
    let end = main.rfind('}').expect("root Window");
    let mut qml = format!(
        "{}\n{}\n}}",
        &main[..end],
        include_str!("Integration.qml.inc")
    );
    if std::env::args().any(|a| a == "--negative-control") {
        qml = qml.replace("s.length === 7", "s.length === 8");
    }
    QApp::new()
        .register::<Backend>()
        .load_qml(qml.as_bytes())
        .run();
}

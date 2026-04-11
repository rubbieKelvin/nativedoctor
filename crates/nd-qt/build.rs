use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("com.nativedoctor.app").qml_file("qml/main.qml"),
    )
    // Qt Core is always linked; QML pulls Gui; on macOS, QML needs Network.
    .qt_module("Network")
    .files(["src/cxxqt_object.rs"])
    .build();
}

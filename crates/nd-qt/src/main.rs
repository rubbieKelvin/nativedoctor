//! NativeDoctor Qt/QML shell — entry point for the cxx-qt desktop app.

pub mod cxxqt_object;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQmlEngine, QUrl};
use std::pin::Pin;

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from(
            "qrc:/qt/qml/com/nativedoctor/app/qml/main.qml",
        ));
    }

    if let Some(engine) = engine.as_mut() {
        let engine: Pin<&mut QQmlEngine> = engine.upcast_pin();
        engine
            .on_quit(|_| {
                println!("QML quit");
            })
            .release();
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}

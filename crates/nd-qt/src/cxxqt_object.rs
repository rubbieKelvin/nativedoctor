//! Example `QObject` exposed to QML — placeholder for future app state.

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        #[namespace = "nd_app"]
        type MyObject = super::MyObjectRust;

        #[qinvokable]
        #[cxx_name = "incrementNumber"]
        fn increment_number(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "sayHi"]
        fn say_hi(&self, string: &QString, number: i32);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct MyObjectRust {
    number: i32,
    string: QString,
}

impl qobject::MyObject {
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.number();
        self.set_number(previous + 1);
    }

    pub fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust: string={string:?}, number={number}");
    }
}

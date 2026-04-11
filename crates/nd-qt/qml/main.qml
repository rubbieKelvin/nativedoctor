import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

import com.nativedoctor.app 1.0

ApplicationWindow {
    id: root
    width: 640
    height: 480
    visible: true
    title: qsTr("NativeDoctor")
    color: palette.window

    readonly property MyObject myObject: MyObject {
        number: 1
        string: qsTr("Rust + Qt (%1)").arg(number)
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: qsTr("Desktop shell (cxx-qt). Number: %1").arg(root.myObject.number)
            color: palette.text
        }

        Label {
            text: qsTr("String: %1").arg(root.myObject.string)
            color: palette.text
        }

        Button {
            text: qsTr("Increment")
            onClicked: root.myObject.incrementNumber()
        }

        Button {
            text: qsTr("Say hi")
            onClicked: root.myObject.sayHi(root.myObject.string, root.myObject.number)
        }

        Button {
            text: qsTr("Quit")
            onClicked: Qt.quit()
        }
    }
}

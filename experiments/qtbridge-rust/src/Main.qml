import QtQuick
import QtQuick.Window
import pong_qtbridge
Window {
    id: root
    visible: true
    width: 1000; height: 660
    color: "#091322"
    title: "Pong / Qt Bridge for Rust"
    property var state: [250, 250, 450, 250, 0, 0, 0]
    property bool up: false
    property bool down: false
    onActiveChanged: if (!active) { up=false; down=false; Backend.pause() }
    Text { anchors.top: parent.top; anchors.topMargin: 18; anchors.horizontalCenter: parent.horizontalCenter
        text: "QT BRIDGE FOR RUST    " + root.state[4] + " : " + root.state[5] + "    " + ["READY","PLAYING","PAUSED","GAME OVER"][root.state[6]]
        color: "#e8f1ff"; font.pixelSize: 22 }
    Text { anchors.bottom: parent.bottom; anchors.bottomMargin: 16; anchors.horizontalCenter: parent.horizontalCenter
        text: "W/S or ↑/↓ move · Enter/R restart · Space play/pause · Esc pause"
        color: "#afc5df"; font.pixelSize: 15 }
    Item {
        id: court
        width: 900; height: 500
        anchors.centerIn: parent
        scale: Math.max(0.01,Math.min(root.width/920,(root.height-110)/500))
        focus: true
        Keys.onPressed: event => {
            if(event.key===Qt.Key_W || event.key===Qt.Key_Up) root.up=true
            else if(event.key===Qt.Key_S || event.key===Qt.Key_Down) root.down=true
            else if(!event.isAutoRepeat && (event.key===Qt.Key_Return || event.key===Qt.Key_Enter || event.key===Qt.Key_R)) Backend.start()
            else if(!event.isAutoRepeat && event.key===Qt.Key_Space) Backend.toggle()
            else if(event.key===Qt.Key_Escape) Backend.pause()
            else return
            event.accepted=true
        }
        Keys.onReleased: event => {
            if(event.isAutoRepeat) return
            if(event.key===Qt.Key_W || event.key===Qt.Key_Up) root.up=false
            if(event.key===Qt.Key_S || event.key===Qt.Key_Down) root.down=false
        }
        Rectangle { anchors.fill: parent; color: "#102339"; border.color: "#476079" }
        Repeater { model: 20; Rectangle { required property int index; x:449; y:index*25; width:2;height:12;color:"#476079" } }
        Rectangle { x:32; y:root.state[0]-45;width:12;height:90;color:"#74edc8" }
        Rectangle { x:856;y:root.state[1]-45;width:12;height:90;color:"#74edc8" }
        Rectangle { x:root.state[2]-8;y:root.state[3]-8;width:16;height:16;radius:8;color:"#ffffff" }
    }
    Timer { interval: 8; running: true; repeat: true; onTriggered: root.state=Backend.frame((root.down?1:0)-(root.up?1:0)) }
}

    // Injected only into the separate integration binary, never the normal app.
    property int testStage: 0
    property real pausedY: 0
    property real pausedBallX: 0
    function expect(ok, message) {
        if (!ok) { console.error("QTBRIDGE_TEST_FAIL: " + message); Qt.exit(1); throw new Error(message) }
    }
    Timer {
        interval: 150; running: true; repeat: true
        onTriggered: {
            let s = Backend.frame(0)
            if (root.testStage === 0) {
                expect(s.length === 7 && s[6] === 0, "initial snapshot contract")
                Backend.start(); root.down = true
            } else if (root.testStage === 1) {
                expect(s[6] === 1 && s[0] > 250, "start and held-axis movement")
                root.down = false; Backend.pause()
                let p = Backend.frame(0); root.pausedY=p[0]; root.pausedBallX=p[2]
            } else if (root.testStage === 2) {
                expect(s[6] === 2 && s[0] === root.pausedY && s[2] === root.pausedBallX, "paused snapshot stable")
                Backend.toggle(); root.up=true
            } else if (root.testStage === 3) {
                expect(s[6] === 1 && s[0] < root.pausedY, "resume and opposite movement")
                root.up=false; Backend.start(); let r=Backend.frame(0)
                expect(r[0] === 250 && r[4] === 0 && r[5] === 0 && r[6] === 1, "restart resets")
                Backend.pause(); root.width=720;root.height=500
            } else if (root.testStage === 4) {
                expect(court.scale > 0 && court.scale < 1, "small-window letterbox scaling")
                root.width=1000;root.height=660
            } else if (root.testStage === 5) {
                expect(court.width === 900 && court.height === 500, "logical court size")
                stop()
                expect(court.grabToImage(function(result) {
                    if (!result.saveToFile("qtbridge-offscreen-court.png")) {
                        console.error("QTBRIDGE_TEST_FAIL: image save"); Qt.exit(1); return
                    }
                    console.log("QTBRIDGE_TEST_PASS: slots, movement, pause, resume, restart, scaling, render")
                    Qt.quit()
                }), "render request accepted")
            }
            root.testStage++
        }
    }
    Timer { interval: 10000; running: true; onTriggered: { console.error("QTBRIDGE_TEST_FAIL: timeout"); Qt.exit(1) } }

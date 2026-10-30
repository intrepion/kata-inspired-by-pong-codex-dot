# Rallylight

An original real-time paddle game inspired by Pong. Return the ball, steer shots with your paddle, and beat the computer to seven points. The computer has a short reaction delay and a limited paddle speed, so well-placed shots can get past it. This independent kata is not affiliated with or endorsed by Atari.

## Play

[Play Rallylight online](https://intrepion.github.io/kata-inspired-by-pong-codex-dot/).

Open `index.html` in a modern browser, or run `python3 -m http.server` and visit `http://localhost:8000`. No dependencies or build step are needed.

Move with the up/down arrows or W/S. Press Space or use the Pause button to pause. On a touch screen, hold the arrow buttons or drag on the court. The game pauses when the tab is hidden or the window loses focus. Hit the ball near the edge of your paddle to change its angle.

## Test

Run `node --test tests/game.test.js`.

## License

MIT; see [LICENSE](LICENSE). The root `.gitignore` combines GitHub's global Linux, macOS, and Windows templates.

'use strict';
// Source of truth: edit game.ts, then run npm run build. game.js is generated.
declare const module: {
    exports: unknown;
} | undefined;
type GameState = 'ready' | 'playing' | 'paused' | 'over';
interface Ball {
    x: number;
    y: number;
    vx: number;
    vy: number;
}
interface Game {
    player: number;
    cpu: number;
    ball: Ball;
    scores: [
        number,
        number
    ];
    state: GameState;
    serve: number;
    random: () => number;
    cpuTarget: number;
    cpuReaction: number;
    resetBall(direction: number): void;
    start(): void;
    step(dt: number, input?: number): void;
}
interface FrameLoop {
    reset(): void;
    advance(timestamp: number, input?: number): void;
}
// The simulation uses seconds and court coordinates so its pace is frame-rate independent.
const W = 900, H = 500, PADDLE_H = 90, PADDLE_W = 12, BALL = 8, WIN = 7;
const clamp = (n: number, min: number, max: number) => Math.max(min, Math.min(max, n));
function createGame(random: () => number = Math.random): Game {
    const s: Game = { player: H / 2, cpu: H / 2, ball: { x: W / 2, y: H / 2, vx: 0, vy: 0 }, scores: [0, 0], state: 'ready', serve: 0, random, cpuTarget: H / 2, cpuReaction: 0, resetBall: () => { }, start: () => { }, step: () => { } };
    s.resetBall = (direction) => { s.ball = { x: W / 2, y: H / 2, vx: direction * 350, vy: (random() - .5) * 180 }; s.serve = .65; s.cpuTarget = H / 2; s.cpuReaction = .3; };
    s.start = () => { s.scores = [0, 0]; s.player = s.cpu = H / 2; s.state = 'playing'; s.resetBall(random() < .5 ? -1 : 1); };
    s.step = (dt, input = 0) => {
        if (s.state !== 'playing' || !Number.isFinite(dt) || !Number.isFinite(input))
            return;
        dt = clamp(dt, 0, .033);
        s.player = clamp(s.player + clamp(input, -1, 1) * 470 * dt, PADDLE_H / 2, H - PADDLE_H / 2);
        const b = s.ball; // The opponent reacts periodically to the ball's current position, with a small aiming error.
        s.cpuReaction -= dt;
        if (s.cpuReaction <= 0) {
            s.cpuTarget = b.vx > 0 ? clamp(b.y + (random() - .5) * 50, PADDLE_H / 2, H - PADDLE_H / 2) : H / 2;
            s.cpuReaction = .3;
        }
        const cpuSpeed = b.vx > 0 ? 190 : 130;
        s.cpu = clamp(s.cpu + clamp(s.cpuTarget - s.cpu, -cpuSpeed * dt, cpuSpeed * dt), PADDLE_H / 2, H - PADDLE_H / 2);
        if (s.serve > 0) {
            s.serve -= dt;
            return;
        }
        const oldX = b.x;
        b.x += b.vx * dt;
        b.y += b.vy * dt;
        if (b.y < BALL) {
            b.y = 2 * BALL - b.y;
            b.vy = Math.abs(b.vy);
        }
        if (b.y > H - BALL) {
            b.y = 2 * (H - BALL) - b.y;
            b.vy = -Math.abs(b.vy);
        }
        for (const [x, y, dir] of [[38, s.player, 1], [W - 38, s.cpu, -1]]) {
            const face = x + dir * (PADDLE_W / 2 + BALL);
            if ((dir === 1 ? oldX >= face && b.x <= face : oldX <= face && b.x >= face) && Math.abs(b.y - y) <= PADDLE_H / 2 + BALL) {
                const angle = clamp((b.y - y) / (PADDLE_H / 2), -1, 1) * 1.05;
                const speed = Math.min(720, Math.hypot(b.vx, b.vy) * 1.055);
                b.x = face;
                b.vx = dir * speed * Math.cos(angle);
                b.vy = speed * Math.sin(angle);
            }
        }
        if (b.x < -BALL || b.x > W + BALL) {
            const scorer = b.x < 0 ? 1 : 0;
            s.scores[scorer]++;
            if (s.scores[scorer] >= WIN) {
                s.state = 'over';
            }
            else
                s.resetBall(scorer === 0 ? -1 : 1);
        }
    };
    return s;
}
// RAF timestamps are milliseconds; simulation steps are seconds. Catch up at most
// 100 ms per frame, matching the Rust web adapters without a long-stall spiral.
function createFrameLoop(game: Game): FrameLoop {
    const step = 1 / 120;
    let last: number | null = null, accumulator = 0;
    return {
        reset() { last = null; accumulator = 0; },
        advance(timestamp, input = 0) {
            if (!Number.isFinite(timestamp) || !Number.isFinite(input))
                return;
            if (last === null) {
                last = timestamp;
                return;
            }
            if (timestamp < last)
                return;
            accumulator += Math.min((timestamp - last) / 1000, .1);
            last = timestamp;
            while (accumulator + 1e-12 >= step) {
                game.step(step, input);
                accumulator = Math.max(0, accumulator - step);
            }
        }
    };
}
if (typeof module !== 'undefined')
    module.exports = { createGame, createFrameLoop, W, H, PADDLE_H, BALL };
if (typeof document !== 'undefined') {
    function element<T extends HTMLElement>(selector: string, kind: {
        new (): T;
    }): T {
        const found = document.querySelector(selector);
        if (!(found instanceof kind))
            throw new Error('Missing or invalid game element: ' + selector);
        return found;
    }
    const canvas = element('#court', HTMLCanvasElement), context = canvas.getContext('2d');
    if (!context)
        throw new Error('Canvas 2D rendering is unavailable');
    const ctx: CanvasRenderingContext2D = context, game = createGame(), keys = new Set<string>(), loop = createFrameLoop(game);
    let touch = 0;
    const overlay = element('#overlay', HTMLElement), message = element('#message', HTMLElement), hint = element('#hint', HTMLElement), start = element('#start', HTMLButtonElement), pause = element('#pause', HTMLButtonElement), status = element('#status', HTMLElement);
    function sync() { element('#you', HTMLElement).textContent = String(game.scores[0]); element('#cpu', HTMLElement).textContent = String(game.scores[1]); pause.disabled = game.state === 'ready' || game.state === 'over'; pause.textContent = game.state === 'paused' ? 'Resume' : 'Pause'; status.textContent = game.state === 'playing' ? (game.serve > 0 ? 'Next serve' : 'Rally in progress') : game.state === 'paused' ? 'Paused' : game.state === 'over' ? (game.scores[0] === WIN ? 'You won the match' : 'Computer wins the match') : 'Ready to play'; overlay.hidden = game.state === 'playing'; if (game.state === 'paused') {
        message.textContent = 'Take a breather.';
        hint.textContent = 'Your match is waiting for you.';
        start.firstChild!.textContent = 'Resume match ';
    } if (game.state === 'over') {
        message.textContent = game.scores[0] === WIN ? 'You take the court.' : 'A worthy opponent.';
        hint.textContent = game.scores[0] === WIN ? 'Nice rally. Ready for another?' : 'The computer won this round. Run it back?';
        start.firstChild!.textContent = 'Play again ';
    } }
    function toggle() { if (game.state === 'playing')
        game.state = 'paused';
    else if (game.state === 'paused')
        game.state = 'playing'; loop.reset(); sync(); }
    start.addEventListener('click', () => { if (game.state === 'paused')
        game.state = 'playing';
    else
        game.start(); loop.reset(); sync(); });
    pause.addEventListener('click', toggle);
    document.addEventListener('keydown', e => { if (['ArrowUp', 'ArrowDown', ' ', 'w', 's', 'W', 'S'].includes(e.key))
        e.preventDefault(); if (e.key === ' ' && !e.repeat && (game.state === 'playing' || game.state === 'paused'))
        toggle(); keys.add(e.key.toLowerCase()); });
    document.addEventListener('keyup', e => keys.delete(e.key.toLowerCase()));
    window.addEventListener('blur', () => { keys.clear(); touch = 0; if (game.state === 'playing')
        toggle(); });
    document.addEventListener('visibilitychange', () => { if (document.hidden && game.state === 'playing')
        toggle(); });
    for (const [id, dir] of [['up', -1], ['down', 1]] as const) {
        const el = element('#' + id, HTMLButtonElement);
        el.addEventListener('pointerdown', e => { e.preventDefault(); touch = dir; el.setPointerCapture(e.pointerId); });
        for (const event of ['pointerup', 'pointercancel', 'lostpointercapture'])
            el.addEventListener(event, () => { touch = 0; });
    }
    canvas.addEventListener('pointerdown', e => { if (e.pointerType !== 'mouse')
        canvas.setPointerCapture(e.pointerId); });
    canvas.addEventListener('pointermove', e => { if (e.buttons && e.pointerType !== 'mouse' && game.state === 'playing') {
        const rect = canvas.getBoundingClientRect();
        game.player = clamp((e.clientY - rect.top) / rect.height * H, PADDLE_H / 2, H - PADDLE_H / 2);
    } });
    function draw() { ctx.fillStyle = '#153f37'; ctx.fillRect(0, 0, W, H); ctx.strokeStyle = '#a8e6bb30'; ctx.lineWidth = 2; ctx.setLineDash([12, 16]); ctx.beginPath(); ctx.moveTo(W / 2, 0); ctx.lineTo(W / 2, H); ctx.stroke(); ctx.setLineDash([]); ctx.strokeStyle = '#a8e6bb25'; ctx.beginPath(); ctx.arc(W / 2, H / 2, 70, 0, Math.PI * 2); ctx.stroke(); ctx.fillStyle = '#d9ffae'; ctx.fillRect(32, game.player - PADDLE_H / 2, PADDLE_W, PADDLE_H); ctx.fillStyle = '#e3f3e7'; ctx.fillRect(W - 44, game.cpu - PADDLE_H / 2, PADDLE_W, PADDLE_H); ctx.beginPath(); ctx.arc(game.ball.x, game.ball.y, BALL, 0, Math.PI * 2); ctx.fill(); }
    function frame(t: number) { loop.advance(t, touch + (keys.has('arrowdown') || keys.has('s') ? 1 : 0) - (keys.has('arrowup') || keys.has('w') ? 1 : 0)); draw(); sync(); requestAnimationFrame(frame); }
    sync();
    requestAnimationFrame(frame);
}

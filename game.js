'use strict';
// The simulation uses seconds and court coordinates so its pace is frame-rate independent.
const W = 900, H = 500, PADDLE_H = 90, PADDLE_W = 12, BALL = 8, WIN = 7;
const clamp = (n, min, max) => Math.max(min, Math.min(max, n));
// Captured pointers retain independent button ownership until released/cancelled.
function createPointerButtons() {
    const held = new Map();
    return {
        press(id, dir) { held.set(id, dir); },
        release(id) { held.delete(id); },
        clear() { held.clear(); },
        axis() {
            const directions = [...held.values()];
            return Number(directions.includes(1)) - Number(directions.includes(-1));
        }
    };
}
// A pause ends all active movement sources; repeats and continuing drags cannot re-arm them.
function createControls() {
    const keys = new Set(), buttons = createPointerButtons(), drags = new Set();
    return {
        keyDown(key, repeat) { if (!repeat)
            keys.add(key.toLowerCase()); },
        keyUp(key) { keys.delete(key.toLowerCase()); },
        buttonDown(id, dir) { buttons.press(id, dir); },
        buttonUp(id) { buttons.release(id); },
        dragDown(id) { drags.add(id); },
        dragUp(id) { drags.delete(id); },
        dragAllowed(id) { return drags.has(id); },
        suspend() { keys.clear(); buttons.clear(); drags.clear(); },
        axis() { return buttons.axis() + Number(keys.has('arrowdown') || keys.has('s')) - Number(keys.has('arrowup') || keys.has('w')); }
    };
}
function createGame(random = Math.random) {
    const s = { player: H / 2, cpu: H / 2, ball: { x: W / 2, y: H / 2, vx: 0, vy: 0 }, scores: [0, 0], state: 'ready', serve: 0, random, cpuTarget: H / 2, cpuReaction: 0, resetBall: () => { }, start: () => { }, step: () => { } };
    s.resetBall = (direction) => { s.ball = { x: W / 2, y: H / 2, vx: direction * 350, vy: (random() - .5) * 180 }; s.serve = .65; s.cpuTarget = H / 2; s.cpuReaction = .3; };
    s.start = () => { s.scores = [0, 0]; s.player = s.cpu = H / 2; s.state = 'playing'; s.resetBall(random() < .5 ? -1 : 1); };
    s.step = (dt, input = 0) => {
        if (s.state !== 'playing' || !Number.isFinite(dt) || !Number.isFinite(input))
            return;
        dt = clamp(dt, 0, .033);
        const oldPlayer = s.player, oldCpu = s.cpu, playerSpeed = clamp(input, -1, 1) * 470;
        s.player = clamp(oldPlayer + playerSpeed * dt, PADDLE_H / 2, H - PADDLE_H / 2);
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
        const oldX = b.x, oldY = b.y, oldVy = b.vy;
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
        for (const [x, dir] of [[38, 1], [W - 38, -1]]) {
            const face = x + dir * (PADDLE_W / 2 + BALL);
            const crosses = dir === 1 ? oldX >= face && b.x <= face : oldX <= face && b.x >= face;
            if (!crosses || oldX === b.x)
                continue;
            const fraction = (face - oldX) / (b.x - oldX), contactTime = dt * fraction;
            let hitY = oldY + oldVy * contactTime;
            if (hitY < BALL)
                hitY = 2 * BALL - hitY;
            else if (hitY > H - BALL)
                hitY = 2 * (H - BALL) - hitY;
            const paddleY = dir === 1
                ? clamp(oldPlayer + playerSpeed * contactTime, PADDLE_H / 2, H - PADDLE_H / 2)
                : clamp(oldCpu + clamp(s.cpuTarget - oldCpu, -cpuSpeed * contactTime, cpuSpeed * contactTime), PADDLE_H / 2, H - PADDLE_H / 2);
            if (Math.abs(hitY - paddleY) <= PADDLE_H / 2 + BALL) {
                const angle = clamp((hitY - paddleY) / (PADDLE_H / 2), -1, 1) * 1.05;
                const speed = Math.min(720, Math.hypot(b.vx, b.vy) * 1.055);
                b.x = face;
                b.y = hitY;
                b.vx = dir * speed * Math.cos(angle);
                b.vy = speed * Math.sin(angle);
                const remaining = dt - contactTime;
                b.x += b.vx * remaining;
                b.y += b.vy * remaining;
                if (b.y < BALL) {
                    b.y = 2 * BALL - b.y;
                    b.vy = Math.abs(b.vy);
                }
                else if (b.y > H - BALL) {
                    b.y = 2 * (H - BALL) - b.y;
                    b.vy = -Math.abs(b.vy);
                }
                break;
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
function createFrameLoop(game) {
    const step = 1 / 120;
    let last = null, accumulator = 0;
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
function parseScenario(text) {
    const fields = new Map(), events = [], inputs = [], captures = [];
    const number = (s) => { if (!/^\d+$/.test(s) || (!Number.isSafeInteger(Number(s)) || Number(s) > 0xffffffff))
        throw Error('Invalid unsigned scenario integer'); return Number(s); };
    for (const line of text.split(/\r?\n/)) {
        const p = line.split('#')[0].trim().split(/\s+/);
        if (!p[0])
            continue;
        if (p[0] === 'event' && p.length === 3 && ['start', 'pause', 'toggle_pause'].includes(p[2]))
            events.push({ tick: number(p[1]), action: p[2] });
        else if (p[0] === 'input' && p.length === 4 && ['-1', '0', '1'].includes(p[3]))
            inputs.push({ from: number(p[1]), to: number(p[2]), value: Number(p[3]) });
        else if (p[0] === 'capture' && p.length === 2)
            captures.push(number(p[1]));
        else if (p.length === 2 && ['schema', 'id', 'seed', 'step_hz', 'end_tick', 'report_every'].includes(p[0]) && !fields.has(p[0]))
            fields.set(p[0], p[1]);
        else
            throw Error('Invalid or duplicate scenario line: ' + line);
    }
    if (fields.get('schema') !== '1' || !/^[a-zA-Z0-9-]+$/.test(fields.get('id') || ''))
        throw Error('Unsupported scenario schema/id');
    const m = { id: fields.get('id'), seed: number(fields.get('seed') || ''), hz: number(fields.get('step_hz') || ''), end: number(fields.get('end_tick') || ''), reportEvery: number(fields.get('report_every') || ''), events, inputs, captures };
    if (!m.hz || !m.end || !m.reportEvery || events.some((e, i) => e.tick >= m.end || (i > 0 && events[i - 1].tick >= e.tick)) || inputs.some((r, i) => r.from >= r.to || r.to > m.end || (i > 0 && inputs[i - 1].to > r.from)) || captures.some((t, i) => !t || t > m.end || (i > 0 && captures[i - 1] >= t)))
        throw Error('Invalid scenario ranges/order');
    return m;
}
// Stable TS-demo-only RNG. This intentionally does not pretend to reproduce Rust's u64/f32 stream.
function scenarioRandom(seed) {
    let state = seed >>> 0;
    return () => { state = (Math.imul(state, 1664525) + 1013904223) >>> 0; return state / 4294967296; };
}
function createScriptedReplay(manifest) {
    const game = createGame(scenarioRandom(manifest.seed));
    let tick = 0, last = null, accumulator = 0;
    const report = () => ({ scenario: manifest.id, implementation: 'original-typescript', seed: manifest.seed, tick, state: game.state, player: game.player, cpu: game.cpu, ball: { ...game.ball }, scores: [...game.scores], finished: tick >= manifest.end });
    const reports = [report()], actions = [];
    function step() {
        if (tick >= manifest.end)
            return;
        for (const event of manifest.events.filter(e => e.tick === tick)) {
            actions.push({ ...event });
            if (event.action === 'start')
                game.start();
            else if (event.action === 'pause' && game.state === 'playing')
                game.state = 'paused';
            else if (event.action === 'toggle_pause') {
                if (game.state === 'paused')
                    game.state = 'playing';
                else if (game.state === 'playing')
                    game.state = 'paused';
            }
        }
        const range = manifest.inputs.find(r => tick >= r.from && tick < r.to);
        game.step(1 / manifest.hz, range?.value || 0);
        tick++;
        if (tick % manifest.reportEvery === 0)
            reports.push(report());
    }
    return { game, reports, actions, manifest, get tick() { return tick; }, get finished() { return tick >= manifest.end; }, step,
        frame(t) {
            if (!Number.isFinite(t))
                return;
            if (last === null) {
                last = t;
                return;
            }
            if (t < last)
                return;
            accumulator += Math.min((t - last) / 1000, .1);
            last = t;
            while (accumulator + 1e-12 >= 1 / manifest.hz && tick < manifest.end) {
                step();
                accumulator = Math.max(0, accumulator - 1 / manifest.hz);
                if (manifest.captures.includes(tick))
                    break;
            }
        }
    };
}
if (typeof module !== 'undefined')
    module.exports = { createControls, createPointerButtons, createGame, createFrameLoop, parseScenario, createScriptedReplay, W, H, PADDLE_H, BALL };
if (typeof document !== 'undefined') {
    function element(selector, kind) {
        const found = document.querySelector(selector);
        if (!(found instanceof kind))
            throw new Error('Missing or invalid game element: ' + selector);
        return found;
    }
    const canvas = element('#court', HTMLCanvasElement), context = canvas.getContext('2d');
    if (!context)
        throw new Error('Canvas 2D rendering is unavailable');
    const ctx = context;
    let game = createGame();
    const loop = createFrameLoop(game);
    const demoRequested = new URLSearchParams(location.search).has('scripted-test');
    let replay;
    const captures = [];
    let exported = false;
    let demoError;
    if (demoRequested)
        fetch('scenarios/smoke-v1.txt').then(r => { if (!r.ok)
            throw Error('Scenario manifest unavailable'); return r.text(); }).then(text => { replay = createScriptedReplay(parseScenario(text)); game = replay.game; }).catch(e => { console.error(e); demoError = String(e); });
    const controls = createControls();
    const overlay = element('#overlay', HTMLElement), message = element('#message', HTMLElement), hint = element('#hint', HTMLElement), start = element('#start', HTMLButtonElement), pause = element('#pause', HTMLButtonElement), status = element('#status', HTMLElement);
    function sync() {
        element('#you', HTMLElement).textContent = String(game.scores[0]);
        element('#cpu', HTMLElement).textContent = String(game.scores[1]);
        pause.disabled = demoRequested || game.state === 'ready' || game.state === 'over';
        start.disabled = demoRequested;
        pause.textContent = game.state === 'paused' ? 'Resume' : 'Pause';
        status.textContent = game.state === 'playing' ? (game.serve > 0 ? 'Next serve' : 'Rally in progress') : game.state === 'paused' ? 'Paused' : game.state === 'over' ? (game.scores[0] === WIN ? 'You won the match' : 'Computer wins the match') : 'Ready to play';
        overlay.hidden = game.state === 'playing';
        if (game.state === 'paused') {
            message.textContent = 'Take a breather.';
            hint.textContent = 'Your match is waiting for you.';
            start.firstChild.textContent = 'Resume match ';
        }
        if (game.state === 'over') {
            message.textContent = game.scores[0] === WIN ? 'You take the court.' : 'A worthy opponent.';
            hint.textContent = game.scores[0] === WIN ? 'Nice rally. Ready for another?' : 'The computer won this round. Run it back?';
            start.firstChild.textContent = 'Play again ';
        }
    }
    function toggle() {
        if (demoRequested)
            return;
        if (game.state === 'playing') {
            controls.suspend();
            game.state = 'paused';
        }
        else if (game.state === 'paused')
            game.state = 'playing';
        loop.reset();
        sync();
    }
    start.addEventListener('click', () => {
        if (demoRequested)
            return;
        if (game.state === 'paused')
            game.state = 'playing';
        else {
            controls.suspend();
            game.start();
        }
        loop.reset();
        sync();
    });
    pause.addEventListener('click', toggle);
    document.addEventListener('keydown', e => {
        if (demoRequested)
            return;
        if (e.key === ' ' && (e.target === start || e.target === pause))
            return;
        if (['ArrowUp', 'ArrowDown', ' ', 'w', 's', 'W', 'S'].includes(e.key))
            e.preventDefault();
        if (e.key === ' ' && !e.repeat && (game.state === 'playing' || game.state === 'paused'))
            toggle();
        if (game.state === 'playing')
            controls.keyDown(e.key, e.repeat);
    });
    document.addEventListener('keyup', e => controls.keyUp(e.key));
    window.addEventListener('blur', () => {
        controls.suspend();
        if (game.state === 'playing')
            toggle();
    });
    document.addEventListener('visibilitychange', () => {
        if (document.hidden && game.state === 'playing')
            toggle();
    });
    for (const [id, dir] of [['up', -1], ['down', 1]]) {
        const el = element('#' + id, HTMLButtonElement);
        el.addEventListener('pointerdown', e => { if (demoRequested || game.state !== 'playing')
            return; e.preventDefault(); controls.buttonDown(e.pointerId, dir); el.setPointerCapture(e.pointerId); });
        // Space/Enter hold a focused movement button without triggering global pause.
        el.addEventListener('keydown', e => {
            if (e.key !== ' ' && e.key !== 'Enter')
                return;
            e.preventDefault();
            e.stopPropagation();
            if (!demoRequested && game.state === 'playing' && !e.repeat)
                controls.buttonDown(`key:${id}:${e.key}`, dir);
        });
        el.addEventListener('keyup', e => {
            if (e.key !== ' ' && e.key !== 'Enter')
                return;
            e.preventDefault();
            e.stopPropagation();
            controls.buttonUp(`key:${id}:${e.key}`);
        });
        el.addEventListener('blur', () => { controls.buttonUp(`key:${id}: `); controls.buttonUp(`key:${id}:Enter`); });
        // Non-pointer activation (including an assistive-technology click) moves one bounded increment.
        el.addEventListener('click', e => {
            if (!demoRequested && game.state === 'playing' && e.detail === 0)
                game.player = clamp(game.player + dir * 32, PADDLE_H / 2, H - PADDLE_H / 2);
        });
        for (const event of ['pointerup', 'pointercancel', 'lostpointercapture'])
            el.addEventListener(event, e => { controls.buttonUp(e.pointerId); });
    }
    canvas.addEventListener('pointerdown', e => {
        if (!demoRequested && game.state === 'playing' && e.pointerType !== 'mouse') {
            controls.dragDown(e.pointerId);
            canvas.setPointerCapture(e.pointerId);
        }
    });
    for (const event of ['pointerup', 'pointercancel', 'lostpointercapture'])
        canvas.addEventListener(event, e => controls.dragUp(e.pointerId));
    canvas.addEventListener('pointermove', e => {
        if (demoRequested)
            return;
        if (e.buttons && e.pointerType !== 'mouse' && game.state === 'playing' && controls.dragAllowed(e.pointerId)) {
            const rect = canvas.getBoundingClientRect();
            game.player = clamp((e.clientY - rect.top) / rect.height * H, PADDLE_H / 2, H - PADDLE_H / 2);
        }
    });
    function draw() { ctx.fillStyle = '#153f37'; ctx.fillRect(0, 0, W, H); ctx.strokeStyle = '#a8e6bb30'; ctx.lineWidth = 2; ctx.setLineDash([12, 16]); ctx.beginPath(); ctx.moveTo(W / 2, 0); ctx.lineTo(W / 2, H); ctx.stroke(); ctx.setLineDash([]); ctx.strokeStyle = '#a8e6bb25'; ctx.beginPath(); ctx.arc(W / 2, H / 2, 70, 0, Math.PI * 2); ctx.stroke(); ctx.fillStyle = '#d9ffae'; ctx.fillRect(32, game.player - PADDLE_H / 2, PADDLE_W, PADDLE_H); ctx.fillStyle = '#e3f3e7'; ctx.fillRect(W - 44, game.cpu - PADDLE_H / 2, PADDLE_W, PADDLE_H); ctx.beginPath(); ctx.arc(game.ball.x, game.ball.y, BALL, 0, Math.PI * 2); ctx.fill(); }
    function frame(t) {
        if (demoRequested)
            replay?.frame(t);
        else
            loop.advance(t, controls.axis());
        draw();
        sync();
        if (demoError)
            status.textContent = demoError;
        if (replay && replay.manifest.captures.includes(replay.tick) && !captures.some(c => c.tick === replay.tick))
            captures.push({ tick: replay.tick, png: canvas.toDataURL('image/png') });
        if (replay?.finished) {
            if (!exported) {
                exported = true;
                const evidence = { kind: 'original TS state and own Canvas2D; not Rust-identical or desktop QA', reports: replay.reports, actions: replay.actions, captures };
                const a = document.createElement('a');
                a.textContent = 'Download original TypeScript demo evidence';
                a.download = 'pong-original-ts-demo.json';
                a.href = URL.createObjectURL(new Blob([JSON.stringify(evidence)], { type: 'application/json' }));
                document.body.append(a);
            }
        }
        else
            requestAnimationFrame(frame);
    }
    sync();
    requestAnimationFrame(frame);
}

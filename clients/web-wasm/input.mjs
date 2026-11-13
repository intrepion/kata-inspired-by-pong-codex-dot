// Track keyboard and pointer bindings independently; only fresh presses in play arm movement.
export class MovementInput {
  constructor() {
    this.keys = new Set();
    this.pointers = new Map();
  }
  key(key, pressed, repeat, playing) {
    key = key.toLowerCase();
    if (!['w', 'arrowup', 's', 'arrowdown'].includes(key)) return;
    if (!pressed) this.keys.delete(key);
    else if (playing && !repeat) this.keys.add(key);
  }
  pointer(id, direction, pressed, playing) {
    if (!pressed) this.pointers.delete(id);
    else if (playing && (direction === -1 || direction === 1)) this.pointers.set(id, direction);
  }
  clear() {
    this.keys.clear();
    this.pointers.clear();
  }
  axis() {
    const directions = [...this.pointers.values()];
    const up = this.keys.has('w') || this.keys.has('arrowup') || directions.includes(-1);
    const down = this.keys.has('s') || this.keys.has('arrowdown') || directions.includes(1);
    return Number(down) - Number(up);
  }
}

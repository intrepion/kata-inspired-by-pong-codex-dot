// Track keyboard and pointer bindings independently; only fresh presses in play arm movement.
export class MovementInput {
  constructor() {
    this.keys = new Set();
    this.pointers = new Map();
    // One unsampled activation per direction; null means normal release.
    this.pending = new Map();
  }
  key(key, pressed, repeat, playing) {
    key = key.toLowerCase();
    if (!['w', 'arrowup', 's', 'arrowdown'].includes(key)) return;
    if (!pressed) this.keys.delete(key);
    else if (playing && !repeat) {
      const dir = key === 'w' || key === 'arrowup' ? -1 : 1;
      if (!this.held(dir) && !this.pending.has(dir)) this.pending.set(dir, key);
      this.keys.add(key);
    }
  }
  pointer(id, direction, pressed, playing) {
    if (!pressed) {
      this.pointers.delete(id);
      for (const [dir, owner] of this.pending) if (owner === id) this.pending.set(dir, null);
    }
    else if (playing && (direction === -1 || direction === 1)) {
      if (!this.held(direction) && !this.pending.has(direction)) this.pending.set(direction, id);
      this.pointers.set(id, direction);
    }
  }
  pointerHeld(id) { return this.pointers.has(id); }
  cancelPointer(id) {
    this.pointers.delete(id);
    for (const [direction, owner] of this.pending) {
      if (owner === id) this.pending.delete(direction);
    }
  }
  clear() {
    this.keys.clear();
    this.pointers.clear();
    this.pending.clear();
  }
  held(direction) {
    return [...this.pointers.values()].includes(direction) || (direction === -1
      ? this.keys.has('w') || this.keys.has('arrowup')
      : this.keys.has('s') || this.keys.has('arrowdown'));
  }
  axis() {
    return Number(this.held(1)) - Number(this.held(-1));
  }
  stepAxis() {
    const axis = Number(this.held(1) || this.pending.has(1)) - Number(this.held(-1) || this.pending.has(-1));
    this.pending.clear();
    return axis;
  }
}

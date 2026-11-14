const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {EventEmitter} = require('node:events');

function fixture({throwOnCreate = null, streamFactory = null} = {}) {
  const code = fs.readFileSync(path.join(__dirname, '..', 'scripts', 'serve.cjs'), 'utf8');
  let handler, stream, listens = 0, creates = 0;
  const fakeHttp = {createServer(fn) { handler = fn; return {listen() { listens++; }}; }};
  const fakeFs = {createReadStream() {
    creates++;
    if (throwOnCreate) throw throwOnCreate;
    if (streamFactory) { stream = streamFactory(); return stream; }
    stream = new EventEmitter();
    stream.destroyed = false;
    stream.destroy = () => { stream.destroyed = true; };
    stream.pipe = res => { stream.pipedTo = res; };
    return stream;
  }};
  const context = {require: name => name === 'node:http' ? fakeHttp : name === 'node:fs' ? fakeFs : require(name),
    __dirname: path.join(__dirname, '..', 'scripts'), process: {env: {}}, URL, console: {log() {}}};
  vm.runInNewContext(code, context);
  function request(url) {
    const res = new EventEmitter();
    Object.assign(res, {headersSent: false, destroyed: false, status: null, body: null,
      writeHead(status, headers) { this.headWrites = (this.headWrites || 0) + 1; this.status = status; this.headers = headers; this.headersSent = true; },
      end(body) { this.endCalls = (this.endCalls || 0) + 1; this.body = body; }, destroy() { this.destroyCalls = (this.destroyCalls || 0) + 1; this.destroyed = true; }});
    handler({url}, res);
    return res;
  }
  return {request, get stream() { return stream; }, get creates() { return creates; }, listens};
}

test('invalid URLs and unknown paths return controlled errors without opening files', () => {
  const f = fixture(); assert.equal(f.listens, 1);
  const bad = f.request('http://['); assert.equal(bad.status, 400); assert.equal(bad.body, 'Bad request');
  const missing = f.request('/not-allowlisted'); assert.equal(missing.status, 404); assert.equal(missing.body, 'Not found');
  assert.equal(f.creates, 0);
});
test('successful open sends correct headers before piping, including query URLs', () => {
  const f = fixture(), res = f.request('/game.js?v=1');
  assert.equal(res.status, null); f.stream.emit('open');
  assert.equal(res.status, 200); assert.equal(res.headers['Content-Type'], 'text/javascript');
  assert.equal(res.headers['Cache-Control'], 'no-store'); assert.equal(f.stream.pipedTo, res);
});
test('open failures return 404 or 500 without leaking error details', () => {
  for (const [code, status, body] of [['ENOENT', 404, 'Not found'], ['EACCES', 500, 'Server error']]) {
    const f = fixture(), res = f.request('/game.js');
    f.stream.emit('error', Object.assign(new Error('private path'), {code}));
    assert.equal(res.status, status); assert.equal(res.body, body);
    assert.equal(res.body.includes('private path'), false);
  }
});
test('a real missing file stream fails asynchronously with 404, without opening a socket', async () => {
  const missing = path.join(__dirname, 'fixture-file-that-does-not-exist');
  assert.equal(fs.existsSync(missing), false);
  const f = fixture({streamFactory: () => fs.createReadStream(missing)}), res = f.request('/game.js');
  await new Promise(resolve => f.stream.once('close', resolve));
  assert.equal(res.status, 404); assert.equal(res.body, 'Not found');
});
test('synchronous stream creation errors return controlled 500', () => {
  const f = fixture({throwOnCreate: new Error('private path')}), res = f.request('/game.js');
  assert.equal(res.status, 500); assert.equal(res.body, 'Server error');
});
test('late stream errors close an already-started response', () => {
  const f = fixture(), res = f.request('/game.js'); f.stream.emit('open');
  f.stream.emit('error', new Error('late')); assert.equal(res.status, 200); assert.equal(res.destroyed, true);
});
test('closed responses destroy the stream and do not start a response', () => {
  const f = fixture(), res = f.request('/game.js'); res.destroyed = true; res.emit('close');
  assert.equal(f.stream.destroyed, true); f.stream.emit('open'); assert.equal(res.status, null);
});
test('stream error after client close neither writes nor ends the response', () => {
  const f = fixture(), res = f.request('/game.js');
  res.destroyed = true; res.emit('close'); f.stream.emit('error', new Error('late'));
  assert.equal(res.headWrites || 0, 0); assert.equal(res.endCalls || 0, 0);
  assert.equal(res.destroyCalls || 0, 0);
});
test('stream error after headers destroys once without a second response', () => {
  const f = fixture(), res = f.request('/game.js'); f.stream.emit('open');
  f.stream.emit('error', new Error('late')); f.stream.emit('error', new Error('later'));
  assert.equal(res.headWrites, 1); assert.equal(res.endCalls || 0, 0);
  assert.equal(res.destroyCalls, 1);
});
test('stream error before open returns one error response', () => {
  const f = fixture(), res = f.request('/game.js');
  f.stream.emit('error', Object.assign(new Error('missing'), {code:'ENOENT'}));
  assert.equal(res.status, 404); assert.equal(res.headWrites, 1); assert.equal(res.endCalls, 1);
});

'use strict';
// Local development only. Serve the checked-in browser files, never file:// URLs.
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const port = Number(process.env.PORT || 4189);
const root = path.resolve(__dirname, '..');
const files = new Map([['/', ['index.html', 'text/html']], ['/index.html', ['index.html', 'text/html']], ['/game.js', ['game.js', 'text/javascript']], ['/scenarios/smoke-v1.txt', ['scenarios/smoke-v1.txt', 'text/plain']], ['/style.css', ['style.css', 'text/css']]]);
http.createServer((req,res) => {
  let pathname;
  try { pathname = new URL(req.url, 'http://127.0.0.1').pathname; }
  catch { res.writeHead(400); res.end('Bad request'); return; }
  const file = files.get(pathname);
  if (!file) { res.writeHead(404); res.end('Not found'); return; }
  const fail = error => {
    if (res.destroyed) return;
    if (res.headersSent) { res.destroy(); return; }
    const missing = error?.code === 'ENOENT';
    res.writeHead(missing ? 404 : 500);
    res.end(missing ? 'Not found' : 'Server error');
  };
  let stream;
  try { stream = fs.createReadStream(path.join(root, file[0])); }
  catch (error) { fail(error); return; }
  stream.on('error', fail);
  res.on('close', () => stream.destroy());
  stream.once('open', () => {
    if (res.destroyed) { stream.destroy(); return; }
    res.writeHead(200, {'Content-Type': file[1], 'Cache-Control': 'no-store'});
    stream.pipe(res);
  });
}).listen(port, '127.0.0.1', () => console.log(`Rallylight: http://127.0.0.1:${port}`));

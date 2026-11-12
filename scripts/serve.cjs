'use strict';
// Local development only. Serve the checked-in browser files, never file:// URLs.
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '..');
const files = new Map([['/', ['index.html', 'text/html']], ['/index.html', ['index.html', 'text/html']], ['/game.js', ['game.js', 'text/javascript']], ['/style.css', ['style.css', 'text/css']]]);
http.createServer((req,res) => {
  const file = files.get(new URL(req.url, 'http://127.0.0.1').pathname);
  if (!file) { res.writeHead(404); res.end('Not found'); return; }
  res.writeHead(200, {'Content-Type': file[1], 'Cache-Control': 'no-store'});
  fs.createReadStream(path.join(root, file[0])).pipe(res);
}).listen(4189, '127.0.0.1', () => console.log('Rallylight: http://127.0.0.1:4189'));

'use strict';
// CI captures only: play the real rendered game through its public controls.
const { chromium } = require('playwright');
const { mkdir, readFile, writeFile } = require('node:fs/promises');
const { createHash } = require('node:crypto');
const path = require('node:path');

(async () => {
  const output = process.env.CAPTURE_OUTPUT || '/tmp/rallylight-captures';
  const site = process.env.CAPTURE_SITE || 'capture-site';
  await mkdir(output, { recursive: true });
  const inputs = {};
  for (const file of ['index.html', 'style.css', 'game.js']) {
    inputs[file] = createHash('sha256').update(await readFile(path.join(site, file))).digest('hex');
  }
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1, locale: 'en-US', reducedMotion: 'reduce' });
    // Fix the game's random sequence without changing its game logic or state.
    await page.addInitScript(() => { let seed = 12345; Math.random = () => ((seed = (seed * 16807) % 2147483647) - 1) / 2147483646; });
    await page.goto('http://127.0.0.1:4173/', { waitUntil: 'networkidle' });
    await page.getByRole('button', { name: /start match/i }).click();
    await page.getByText('Rally in progress').waitFor({ state: 'visible', timeout: 10000 });
    const court = page.locator('section.game');
    await court.screenshot({ path: path.join(output, 'rally-01.png'), animations: 'disabled' });
    await page.keyboard.down('ArrowDown');
    await page.waitForTimeout(400);
    await page.keyboard.up('ArrowDown');
    await page.waitForTimeout(300);
    await court.screenshot({ path: path.join(output, 'rally-02.png'), animations: 'disabled' });
    const metadata = { schemaVersion: 1, sourceCommit: process.env.GITHUB_SHA || null, runId: process.env.GITHUB_RUN_ID || null, source: 'local CI build served over HTTP', url: 'http://127.0.0.1:4173/', inputs, browser: browser.version(), viewport: { width: 1280, height: 900 }, seed: 12345, screenshots: ['rally-01.png', 'rally-02.png'] };
    await writeFile(path.join(output, 'capture.json'), JSON.stringify(metadata, null, 2) + '\n');
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });

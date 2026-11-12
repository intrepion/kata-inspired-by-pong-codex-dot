// Derived resources only. Keep the declarative timeline in scenarios/smoke-v1.txt.
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '..');
const source = fs.readFileSync(path.join(root, 'scenarios/smoke-v1.txt'));
const destinations = [
  'experiments/native-swift/App/Resources/smoke-v1.txt',
  'experiments/native-swift/Tests/PongCoreTests/Fixtures/smoke-v1.txt',
  'experiments/native-kotlin/app/src/test/resources/smoke-v1.txt',
  'experiments/native-kotlin/app/src/main/assets/scenarios/smoke-v1.txt',
];
for (const name of destinations) {
  const file = path.join(root, name);
  if (process.argv.includes('--check')) {
    if (!fs.existsSync(file) || !fs.readFileSync(file).equals(source)) throw new Error(`Stale scenario resource: ${name}`);
  } else {
    fs.mkdirSync(path.dirname(file), {recursive:true}); fs.writeFileSync(file, source);
  }
}
console.log(`Scenario resources ${process.argv.includes('--check') ? 'verified' : 'generated'}: ${destinations.length}`);

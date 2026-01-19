const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const ROOT = path.resolve(__dirname, '..');
const CACHE_DIR = path.join(ROOT, '.cache', 'mermaid_validate');

function walk(dir) {
  let results = [];
  const list = fs.readdirSync(dir, { withFileTypes: true });
  for (const dirent of list) {
    const full = path.join(dir, dirent.name);
    if (dirent.isDirectory()) {
      if (['.git', 'node_modules', '.cache'].includes(dirent.name)) continue;
      results = results.concat(walk(full));
    } else if (dirent.isFile() && full.endsWith('.md')) {
      results.push(full);
    }
  }
  return results;
}

function extractMermaidBlocks(text) {
  const regex = /^```mermaid\s*\n([\s\S]*?)\n```/gm;
  const blocks = [];
  let match;
  while ((match = regex.exec(text)) !== null) {
    // Find the line number for reporting
    const before = text.slice(0, match.index);
    const line = before.split('\n').length;
    blocks.push({ code: match[1], index: match.index, line });
  }
  return blocks;
}

function ensureDir(d) {
  if (!fs.existsSync(d)) fs.mkdirSync(d, { recursive: true });
}

function run() {
  ensureDir(CACHE_DIR);
  const mdFiles = walk(ROOT);
  let total = 0;
  let failed = 0;
  const failures = [];

  console.log(`Found ${mdFiles.length} markdown files. Scanning for mermaid blocks...`);
  let counter = 0;
  for (const file of mdFiles) {
    const text = fs.readFileSync(file, 'utf8');
    const blocks = extractMermaidBlocks(text);
    if (!blocks.length) continue;
    for (const b of blocks) {
      counter += 1;
      total += 1;
      const mmdPath = path.join(CACHE_DIR, `diagram_${counter}.mmd`);
      const outPath = path.join(CACHE_DIR, `diagram_${counter}.svg`);
      fs.writeFileSync(mmdPath, b.code, 'utf8');

      console.log(`Rendering #${counter} from ${path.relative(ROOT, file)} (line ${b.line}) -> ${path.relative(ROOT, outPath)}`);

      // Use npx to run mermaid-cli (mmdc). Use shell true for Windows compatibility.
      const cmd = `npx --yes @mermaid-js/mermaid-cli -i "${mmdPath}" -o "${outPath}"`;
      console.log('CMD:', cmd);
      const res = spawnSync(cmd, { shell: true, encoding: 'utf8', timeout: 120000 });

      // Print outputs for diagnostics
      if (res.stdout && res.stdout.trim()) console.log('STDOUT:\n', res.stdout.trim());
      if (res.stderr && res.stderr.trim()) console.error('STDERR:\n', res.stderr.trim());
      if (res.error) console.error('ERROR:', res.error);

      if (res.status !== 0) {
        failed += 1;
        failures.push({ file, line: b.line, error: res.stderr || res.stdout || String(res.error) });
        console.error(`❌ Render failed (#${counter})`);
      } else {
        console.log(`✅ Rendered (#${counter}) OK`);
      }
    }
  }

  console.log('\n---\nValidation Summary');
  console.log(`Total diagrams: ${total}`);
  console.log(`Failures: ${failed}`);
  if (failed > 0) {
    console.log('\nFailure details:');
    failures.forEach((f, i) => {
      console.log(`\n${i+1}) File: ${path.relative(ROOT, f.file)} (line ${f.line})`);
      console.log('Error:\n', f.error);
    });
  }

  process.exit(failed > 0 ? 1 : 0);
}

run();

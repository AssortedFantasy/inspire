// Rasterize validated, self-contained SVGs with the same renderer and canvas.
const fs = require('node:fs');
const sharp = require('sharp');

async function main() {
  const jobs = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
  for (const job of jobs) {
    try {
      await sharp(Buffer.from(job.svg), { density: 144, limitInputPixels: 16000000 })
        .resize(400, 400, { fit: 'contain', background: '#ffffff' })
        .flatten({ background: '#ffffff' }).png().toFile(job.output);
      console.log(JSON.stringify({ id: job.id, status: 'rendered' }));
    } catch (error) {
      console.log(JSON.stringify({ id: job.id, status: 'failed', reason: error.message }));
    }
  }
}
main().catch(error => { console.error(error.message); process.exitCode = 1; });

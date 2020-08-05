const { Suite } = require('benchmark');
const { readFile: neonReadFile, readFileSync } = require('../');
const { readFile: nodeReadFile } = require('fs/promises');
const { join } = require('path');

const filePath = join(__dirname, 'file.txt');
const expected = Buffer.from('Hello, world!\nNode + rust is cool.\n'.repeat(64));

function validate(result) {
	return expected.equals(result) ? Promise.resolve() : Promise.reject(new Error(`${result} does not match ${expected}`));
}

const suite = new Suite()
	.add('Node read file', async () => {
		await validate(await nodeReadFile(filePath));
	})
	.add('Neon read file sync', async () => {
		await validate(readFileSync(filePath));
	})
	.add('Neon read file', async () => {
		await validate(await neonReadFile(filePath));
	})
	.on('cycle', (event) => {
		console.log(String(event.target));
	})
	.on('complete', () => {
		console.log(`Fastest was ${suite.filter('fastest').map('name')}`);
		process.exit();
	})
	.run({ 'async': true });

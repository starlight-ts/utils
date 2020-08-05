/* eslint-disable @typescript-eslint/no-var-requires */
const native = require('../native');
const util = require('util');

module.exports = {
	readFile: util.promisify(native.readFile),
	readFileSync: native.readFileSync,
	writeFile: util.promisify(native.writeFile),
	writeFileSync: native.writeFileSync,
	version: require('../package.json').version
};

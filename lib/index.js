/* eslint-disable @typescript-eslint/no-var-requires */
const native = require('../native');
const util = require('util');

module.exports = {
	readFile: util.promisify(native.readFile),
	writeFile: util.promisify(native.writeFile),
	version: require('../package.json').version
};

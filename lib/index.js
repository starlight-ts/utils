const native = require('../native');
const util = require('util');

module.exports = {
	readFile: util.promisify(native.readFile)
}

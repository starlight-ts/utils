declare module '@pyrotechniac/utils' {
	/**
	 * Reads a file and returns a buffer.
	 * @param path The path to the file.
	 */
	export function readFile(path: string): Promise<Buffer>;

	/**
	 * Writes to a file, creating it if it doesn't exist.
	 * @param path The path to the file.
	 * @param data The data to write.
	 */
	export function writeFile(path: string, data: Buffer): Promise<void>;
}

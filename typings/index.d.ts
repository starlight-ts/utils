declare module '@pyrotechniac/utils' {
	/**
	 * Reads a file and returns a buffer.
	 * @param path The path to the file.
	 */
	export function readFile(path: string): Promise<Buffer>;
}

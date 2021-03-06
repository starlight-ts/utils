/* eslint-disable @typescript-eslint/init-declarations */

declare module '@pyrotechniac/utils' {
	/**
	 * The version of the library
	 */
	export const version: string;

	/**
	 * Reads a file and returns a buffer.
	 * @param path The path to the file.
	 */
	export function readFile(path: string): Promise<Buffer>;

	/**
	 * Reads a file and returns a buffer.
	 * @param path The path to the file.
	 */
	export function readFileSync(path: string): Buffer;

	/**
	 * Writes to a file, creating it if it doesn't exist.
	 * @param path The path to the file.
	 * @param data The data to write.
	 */
	export function writeFile(path: string, data: Buffer): Promise<void>;

	/**
	 * Writes to a file, creating it if it doesn't exist.
	 * @param path The path to the file.
	 * @param data The data to write.
	 */
	export function writeFileSync(path: string, data: Buffer): void;
}

/**
 * Policy for trailing commas
 */
export type Policy = "ignore" | "insert" | "remove";

/**
 * Configuration options for WGSL formatting
 */
export interface Config {
	/**
	 * How to handle trailing commas
	 *
	 * @default "ignore"
	 */
	trailing_commas?: Policy;
	/**
	 * The string used for indentation (e.g., "\t" or "    ")
	 *
	 * @default "    " (4 spaces)
	 */
	indent_symbol?: string;
}

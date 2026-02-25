/* @ts-self-types="./wgslfmt.d.ts" */
import { readFileSync } from "node:fs";
import * as import_bg from "./wgslfmt_bg.js";
const { __wbg_set_wasm, format, ...wasmImport } = import_bg;

const wasmUrl = new URL("wgslfmt_bg.wasm", import.meta.url);
const wasmBytes = readFileSync(wasmUrl);
const wasmModule = new WebAssembly.Module(wasmBytes);

function getImports() {
	return {
		__proto__: null,
		"./wgslfmt_bg.js": wasmImport,
	};
}

/**
 * @import * as WASM from "./wgslfmt.wasm"
 */

const instance = new WebAssembly.Instance(wasmModule, getImports());

/**
 * @type {WASM}
 */
const wasm = instance.exports;
__wbg_set_wasm(wasm);

export { format };

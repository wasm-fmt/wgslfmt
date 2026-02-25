/* @ts-self-types="./wgslfmt.d.ts" */
// prettier-ignore
import source wasmModule from "./wgslfmt_bg.wasm";

import * as import_bg from "./wgslfmt_bg.js";
const { __wbg_set_wasm, format, ...wasmImport } = import_bg;

function getImports() {
	return {
		__proto__: null,
		"./wgslfmt_bg.js": wasmImport,
	};
}

const instance = new WebAssembly.Instance(wasmModule, getImports());

/**
 * @import * as WASM from "./wgslfmt_bg.wasm"
 */

/**
 * @type {WASM}
 */
const wasm = instance.exports;
__wbg_set_wasm(wasm);

export { format };

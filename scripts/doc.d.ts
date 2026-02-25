/**
 * WASM formatter for WGSL (WebGPU Shading Language).
 *
 * Powered by [wgsl-analyzer](https://github.com/wgsl-analyzer/wgsl-analyzer).
 *
 * @example
 * ```ts
 * import { format } from "@wasm-fmt/wgslfmt";
 *
 * const input = "@vertex fn vs() -> @builtin(position) vec4f { return vec4f(0.0); }";
 * const output = format(input);
 * ```
 *
 * @module
 */

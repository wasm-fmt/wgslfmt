[![Test](https://github.com/wasm-fmt/wgslfmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/wgslfmt/actions/workflows/test.yml)

# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/wgslfmt)](https://www.npmjs.com/package/@wasm-fmt/wgslfmt)

```bash
npm install @wasm-fmt/wgslfmt
```

[![jsr.io](https://jsr.io/badges/@fmt/wgslfmt)](https://jsr.io/@fmt/wgslfmt)

```bash
npx jsr add @fmt/wgslfmt
```

# Usage

## Node.js / Deno / Bun / Bundler

```javascript
import { format } from "@wasm-fmt/wgslfmt";

const input = `@vertex fn vs() -> @builtin(position) vec4f {
return vec4f(0.0, 0.0, 0.0, 1.0);
}`;

const formatted = format(input);
console.log(formatted);
```

## Web

For web environments, you need to initialize the WASM module manually:

```javascript
import init, { format } from "@wasm-fmt/wgslfmt/web";

await init();

const input = `@vertex fn vs() -> @builtin(position) vec4f {
return vec4f(0.0, 0.0, 0.0, 1.0);
}`;

const formatted = format(input);
console.log(formatted);
```

### Vite

```JavaScript
import init, { format } from "@wasm-fmt/wgslfmt/vite";

await init();
// ...
```

Or use the `./bundler` entry with [vite-plugin-wasm](https://www.npmjs.com/package/vite-plugin-wasm)

```javascript
import { format } from "@wasm-fmt/wgslfmt/bundler";
```

## Entry Points

- `.` - Auto-detects environment (Node.js uses node, Webpack uses bundler, default is ESM)
- `./node` - Node.js environment (no init required)
- `./esm` - ESM environments like Deno (no init required)
- `./bundler` - Bundlers like Webpack (no init required)
- `./web` - Web browsers (requires manual init)
- `./vite` - Vite bundler (requires manual init)

## Configuration

```javascript
import { format } from "@wasm-fmt/wgslfmt";

const formatted = format(input, {
	trailing_commas: "ignore", // "ignore" | "insert" | "remove"
	indent_symbol: "\t", // e.g., "\t" for tabs or "    " for 4 spaces
});
```

### Options

- `trailing_commas`: How to handle trailing commas
  - `"ignore"` (default): Keep existing trailing commas
  - `"insert"`: Add trailing commas where missing
  - `"remove"`: Remove all trailing commas

- `indent_symbol`: The string used for indentation (default: `"    "` - 4 spaces)
  - Use `"\t"` for tabs
  - Use `"  "` for 2 spaces
  - Or any other string

# Credits

Thanks to:

- The [wgsl-analyzer](https://github.com/wgsl-analyzer/wgsl-analyzer) project

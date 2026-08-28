# Marko for Zed

Zed language extension for [Marko](https://markojs.com) (`.marko` files):
syntax highlighting (Tree-sitter), TypeScript/CSS injections, and LSP support
via `@marko/language-server`.

## Dev install

1. Open Zed's extensions page (`zed: extensions`).
2. Click "Install Dev Extension" and select this directory.
3. Open `examples/hello.marko` to verify highlighting and language server
   startup. Expect:
   - Syntax highlighting across the file's mix of `<let>`/`<const>`,
     `<if>`/`<else-if>`/`<else>`, `<for>`, attribute tags, a dynamic tag,
     concise-mode text, and the `<style>` block.
   - A type diagnostic on the `static function shout(text: abobora)` line —
     `abobora` is a deliberately invalid type, left in as an LSP smoke
     test. Its presence in the editor (a red squiggle / Problems panel
     entry) confirms the language server is attached and type-checking,
     not just that the grammar highlights.

`examples/tsconfig.json` exists so `@marko/language-server` picks up a
project config for that directory — without one, it treats `.marko` files
as plain JS and skips type diagnostics entirely (no `abobora` squiggle).

## Language server

The extension installs `@marko/language-server` from npm on demand (via
`zed::npm_install_package`), then runs its `bin.js` entry point through the
Node binary Zed provides (`zed::node_binary_path()`), passing `--stdio`
explicitly — `vscode-languageserver`'s `createConnection()` throws without
it.

## Grammar

The Tree-sitter grammar and queries are sourced from
[marko-js/tree-sitter](https://github.com/marko-js/tree-sitter), pinned to a
commit in `extension.toml`.

# Marko for Zed

Zed language extension for [Marko](https://markojs.com) (`.marko` files):
syntax highlighting (Tree-sitter), TypeScript/CSS injections, and LSP support
via `@marko/language-server`.

## Dev install

1. Open Zed's extensions page (`zed: extensions`).
2. Click "Install Dev Extension" and select this directory.
3. Open a `.marko` file (see `examples/hello.marko`) to verify highlighting
   and language server startup.

## Language server

The extension installs `@marko/language-server` from npm on demand (via
`zed::npm_install_package`), then runs its `bin.js` entry point through the
Node binary Zed provides (`zed::node_binary_path()`). The server communicates
over stdio; no extra flags are required.

## Grammar

The Tree-sitter grammar and queries are sourced from
[marko-js/tree-sitter](https://github.com/marko-js/tree-sitter), pinned to a
commit in `extension.toml`.

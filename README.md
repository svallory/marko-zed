# Marko for Zed

[![CI](https://github.com/svallory/marko-zed/actions/workflows/ci.yml/badge.svg)](https://github.com/svallory/marko-zed/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Zed language extension for [Marko](https://markojs.com) (`.marko` files):
syntax highlighting (Tree-sitter), TypeScript/CSS injections, and LSP support
via `@marko/language-server`.

## Features

- Tree-sitter syntax highlighting for `.marko` files (tags, attribute tags,
  `<let>`/`<const>`, control-flow tags, concise-mode text)
- TypeScript and CSS injections inside script/style blocks
- Full LSP support via `@marko/language-server`, auto-installed from npm
- Automatic TypeScript default-lib patching so type diagnostics work even in
  projects without `node_modules`

## Installation

**From the Zed extension registry:** coming soon (pending submission to
[zed-industries/extensions](https://github.com/zed-industries/extensions)).

**Dev install (today):**

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

### TypeScript default libraries

After installing the server, the extension copies TypeScript's default
`lib.*.d.ts` files into the server's `dist/` directory. The server looks for
them by resolving `typescript` from the *project's* tsconfig and falls back to
its own bundle directory when that fails, and the npm-published server ships no
libs there — so in a project without `node_modules` every global type
(`Promise`, `Date`, `Symbol`) goes missing. The official VS Code extension
copies the same files into its bundle at build time; this reproduces that step
for the npm build. The libs come from whichever `typescript` npm installed for
the server, so they always match the compiler it runs.

## Grammar

The Tree-sitter grammar and queries are sourced from
[marko-js/tree-sitter](https://github.com/marko-js/tree-sitter), pinned to a
commit in `extension.toml`.

## Contributing

Bug reports, feature requests, and PRs are welcome. See
[CONTRIBUTING.md](CONTRIBUTING.md) for dev setup, the build/lint commands, and
the PR process.

## License

[MIT](LICENSE)

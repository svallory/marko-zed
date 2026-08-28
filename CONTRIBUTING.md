# Contributing to Marko for Zed

## Development setup

1. Install [Rust](https://rustup.rs) and the `wasm32-wasip2` target (the
   target Zed's dev-extension loader uses):

   ```bash
   rustup target add wasm32-wasip2
   ```

2. Build the extension:

   ```bash
   cargo build --target wasm32-wasip2
   ```

3. Lint and format before committing:

   ```bash
   cargo fmt --check
   cargo clippy --all-targets --target wasm32-wasip2 -- -D warnings
   ```

   These two commands are what CI runs; a PR won't pass until both are clean.

## Testing in Zed

There's no automated LSP integration test — Zed extensions run inside the
editor, so verification is manual:

1. Open Zed's extensions page (`zed: extensions`).
2. Click "Install Dev Extension" and select this repo's root.
3. Open `examples/hello.marko`. You should see:
   - Syntax highlighting across the file's mix of `<let>`/`<const>`,
     `<if>`/`<else-if>`/`<else>`, `<for>`, attribute tags, a dynamic tag,
     concise-mode text, and the `<style>` block.
   - A type diagnostic on the `static function shout(text: abobora)` line —
     `abobora` is a deliberately invalid type kept in the fixture as an LSP
     smoke test. Seeing the squiggle confirms `@marko/language-server`
     actually attached and type-checked, not just that the grammar
     highlighted.

`examples/tsconfig.json` exists so the language server picks up a project
config for that directory — without one it treats `.marko` files as plain JS
and skips type diagnostics entirely (no `abobora` squiggle). If you change
anything touching LSP startup or the TypeScript lib-copying step
(`src/lib.rs`), rerun this smoke test — it's the only thing that exercises
that path end-to-end.

## Commit conventions

This repo follows [Conventional Commits](https://www.conventionalcommits.org/):
`type(scope): summary`, e.g. `fix(lsp): pass --stdio to language server`.
Common types: `feat`, `fix`, `docs`, `chore`, `ci`, `refactor`.

## Pull request process

1. Fork the repo and branch off `main`.
2. Make your change, keeping `cargo fmt` and `cargo clippy` clean.
3. Update `CHANGELOG.md` under `Unreleased` if the change is user-facing.
4. Open a PR against `main` using the PR template. Link any related issue.
5. CI must pass before merge.

## Reporting bugs / requesting features

Use the issue templates in `.github/ISSUE_TEMPLATE/`. For bugs, include your
Zed version, the extension version, your OS, and the relevant excerpt from
`Zed.log` (Zed menu → Help → Open Log).

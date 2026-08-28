# marko-zed agent instructions

Zed language extension for Marko. Rust/WASM extension crate + Tree-sitter
grammar/query files + npm-provisioned LSP.

## Build

```
rustup target add wasm32-wasip2   # what Zed's dev-extension loader uses
cargo build --target wasm32-wasip2
```

`wasm32-wasip1` also compiles clean with `zed_extension_api = "0.7.0"` and is
fine as a faster compile check, but `wasip2` is the primary/required target —
build and verify against it before claiming the extension works.

## Toolchain notes

- `wasm32-wasip2` is required per the crate's own README
  (`zed_extension_api-0.7.0/README.md`: "Have the `wasm32-wasip2` target
  installed") and is confirmed installed and building clean here
  (`cargo build --target wasm32-wasip2` passes).
- No `zed` CLI dev-extension smoke test has been run in CI/agent contexts —
  only `cargo build` is verified automatically. A human with the Zed app
  should do "Install Dev Extension" + open `examples/src/routes/+page.marko`
  (after `bun install` inside `examples/`) to confirm
  highlighting/LSP end-to-end.

## Language server

- After installing `@marko/language-server`, the extension copies TypeScript's
  `lib.*.d.ts` files into the server's `dist/` (see `copy_default_ts_libs` in
  `src/lib.rs`) — without them, projects with no `node_modules` report bogus
  missing-global-type errors. Keep that step in sync with any install changes.

## Remote

`origin` is `git@github.com:svallory/marko-zed.git`, `main` pushed.

## Zed dev-extension symlink (project-specific)

The operator's Zed installs the dev extension from the stable symlink
`<space-root>/dev-ext` (local-only, never committed), which points at one
worktree — `worktrees/main` by default. To test in-progress work in Zed,
that symlink is repointed (`ln -sfn worktrees/<ref> dev-ext`) and Zed
reloads extensions; never install a dev extension from a worktree path
directly (worktree removal leaves Zed with a dangling pointer).

Only one worktree can own the symlink at a time. Dev agents must NOT
repoint it on their own: ask the team lead, who controls which session
holds it and repoints/restores it. (This is a rule of this repo's space,
not part of the team-lead skill.)

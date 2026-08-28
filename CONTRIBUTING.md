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
3. Open `examples/src/routes/+page.marko`. You should see:
   - Syntax highlighting across the file's mix of `<let>`/`<const>`,
     `<if>`/`<else-if>`/`<else>`, `<for>`, attribute tags, a dynamic tag,
     concise-mode text, and the `<style>` block.
   - A type diagnostic on the `static function shout(text: abobora)` line —
     `abobora` is a deliberately invalid type kept in the fixture as an LSP
     smoke test. Seeing the squiggle confirms `@marko/language-server`
     actually attached and type-checked, not just that the grammar
     highlighted.

`examples/` is a standalone `@marko/run` app (its own `package.json`,
`tsconfig.json`, `bun.lock`) so the language server picks up a real project
config — without one it treats `.marko` files as plain JS and skips type
diagnostics entirely (no `abobora` squiggle). Run `bun install` inside
`examples/` before opening it in Zed. If you change anything touching LSP
startup or the TypeScript lib-copying step (`src/lib.rs`), rerun this smoke
test — it's the only thing that exercises that path end-to-end.

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

## Releasing

Releases are automated by `.github/workflows/release.yml`, triggered by
pushing a `vX.Y.Z` tag (or via `workflow_dispatch` with a `tag` input, for
re-running a release). The procedure:

1. Bump the `version` field in `extension.toml` (e.g. `0.1.0` → `0.2.0`).
2. In `CHANGELOG.md`, move the contents of `## [Unreleased]` into a new
   dated section for the release, e.g.:

   ```markdown
   ## [Unreleased]

   ## [0.2.0] - 2026-09-01

   ### Added

   - ...(moved from Unreleased)
   ```

   Leave an empty `## [Unreleased]` section at the top for future changes.
3. Commit these changes (e.g. `chore(release): v0.2.0`).
4. Tag the commit and push the tag:

   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

Pushing the tag triggers three CI jobs:

- **verify** — confirms `extension.toml`'s version matches the tag, then
  runs `cargo fmt --check`, `cargo clippy`, and `cargo build` against
  `wasm32-wasip2`.
- **github-release** — creates a GitHub Release for the tag, using the
  matching `CHANGELOG.md` section as the release notes.
- **registry-pr** — opens (or reuses, if one is already open) a pull
  request against [`zed-industries/extensions`](https://github.com/zed-industries/extensions)
  updating the `marko` submodule and `extensions.toml` entry to the new
  version.

If the workflow fails partway, just re-run it via `workflow_dispatch` (with
the same tag as input) — completed steps (release already created, PR
already open) are detected and skipped, so re-running only retries what's
actually missing.

### One-time setup: `EXTENSIONS_PAT` secret

The `registry-pr` job pushes a branch to the
[`svallory/extensions`](https://github.com/svallory/extensions) fork and
opens a pull request against the public upstream
`zed-industries/extensions` repository. It uses a dedicated
`EXTENSIONS_PAT` secret instead of the default `GITHUB_TOKEN`, because the
default token is scoped to this repository only and cannot push to, or
open PRs against, a different repository.

**Token type:** a **classic** Personal Access Token. Fine-grained PATs
currently cannot open a pull request from a fork against an upstream
repository owned by someone else (a known GitHub platform gap), so a
fine-grained token will not work for this job.

**Scope required:** `public_repo` — this grants read/write access to code,
commit statuses, and pull requests on public repositories, which is
exactly what's needed to push a branch to the public `svallory/extensions`
fork and open a PR against the public `zed-industries/extensions` repo.
The broader `repo` scope (which also grants private-repo access) is not
needed since both repositories involved are public.

**To create the token:**

1. Go to GitHub → Settings → Developer settings → Personal access tokens →
   [Tokens (classic)](https://github.com/settings/tokens).
2. Generate a new token (classic), select the `public_repo` scope, and set
   an expiration.
3. Copy the token value (shown once).

**To add it as a repository secret:**

1. In the `marko-zed` repository on GitHub, go to **Settings** → **Secrets
   and variables** → **Actions**.
2. Under the **Secrets** tab, click **New repository secret**.
3. Name it `EXTENSIONS_PAT`, paste the token value, and save.

## Reporting bugs / requesting features

Use the issue templates in `.github/ISSUE_TEMPLATE/`. For bugs, include your
Zed version, the extension version, your OS, and the relevant excerpt from
`Zed.log` (Zed menu → Help → Open Log).

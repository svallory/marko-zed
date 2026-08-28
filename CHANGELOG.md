# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-08-28

### Added

- Tree-sitter grammar integration for `.marko` files, pinned to
  [marko-js/tree-sitter](https://github.com/marko-js/tree-sitter).
- Syntax highlighting queries covering tags, attribute tags,
  `<let>`/`<const>`, control-flow tags (`<if>`/`<else>`/`<else>`,
  `<for>`), and concise-mode text.
- TypeScript and CSS injections inside script/style blocks.
- LSP support via `@marko/language-server`, auto-installed from npm on
  first use.
- Automatic patch of TypeScript's default `lib.*.d.ts` files into the
  language server's bundle, fixing bogus missing-global-type errors in
  projects without `node_modules`.

[Unreleased]: https://github.com/svallory/marko-zed/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/svallory/marko-zed/releases/tag/v0.1.0

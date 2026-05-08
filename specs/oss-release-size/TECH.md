# OSS Release Size Audit

## Goal

Reduce the OSS release bundle size for this fork and keep the remaining work
focused on removing AI, auth, MCP, editor, and git UI code from the build.

## Current Evidence

Measured on macOS arm64 after building with:

```sh
./script/bundle --channel oss --nosign
```

The DMG step currently fails in this environment with `hdiutil: create failed -
Device not configured`, but the `.app` bundle is fully produced before that
step.

| Artifact | Before | Current |
| --- | ---: | ---: |
| `vфrp.app` | 368M | 167M |
| `Contents/MacOS/warp-oss` | 360M | 166M |
| `Contents/Resources` | 3.7M | 1.7M |

`xcrun size -m` on `Contents/MacOS/warp-oss` now reports:

| Section | Current |
| --- | ---: |
| `__TEXT` | 164,954,112 bytes |
| `__TEXT,__text` | 87,774,728 bytes |
| `__TEXT,__const` | 58,970,064 bytes |
| `__LINKEDIT` | 5,636,096 bytes |

The dSYM is preserved outside the app at `target/release-lto/bundle/osx/warp-oss.dSYM`
and is currently about 923M.

## Completed Cuts

- OSS bundle scripts pass `--no-default-features`.
- OSS app builds no longer enable `nld_improvements`.
- `gui` no longer enables `voice_input`.
- `oss_release` excludes `app/assets/async/**` from the embedded RustEmbed asset set.
- macOS OSS binaries are stripped with `strip -x` after the dSYM is copied.
- OSS resources omit bundled AI/MCP skills.
- macOS OSS resources omit the dock tile plugin.
- OSS startup skips AI, MCP, code review, codebase-indexing, managed-secrets,
  and code-editor singleton registrations.
- `generate_settings_schema --channel oss` uses release-level feature flags instead of falling back to dev.
- `input_classifier` is optional and defaults to the real classifier only for
  non-OSS builds. OSS builds use a small local shell-only shim for
  `InputType`, classifier context, and follow-up helpers.
- `natural_language_detection` is no longer a direct or transitive OSS app
  dependency.
- `onboarding` no longer depends on the `ai` crate just to share model IDs.
  It owns a small local `LLMId` string newtype and converts to the app AI
  model ID at the app boundary.
- AWS SDK credential loading is behind the non-OSS `aws_credentials` Cargo
  feature. OSS builds keep BYO-LLM AWS credential state disabled without
  pulling `aws-config`, STS, or AWS Smithy runtime crates.
- Tantivy full-text command-palette search is behind the existing
  `use_tantivy_search` feature. OSS builds use the fuzzy-search paths and no
  longer pull `tantivy`, `ownedbytes`, `bitpacking`, `levenshtein_automata`,
  or the data-sketch crates.
- Notebook code-block syntax highlighting is behind the
  `notebook_syntax_highlighting` feature. OSS builds keep shell command-token
  highlighting but no longer pull `syntect`.
- The embedded command-signature dataset is behind the
  `embedded_command_signatures` app feature and the
  `warp_completer/embedded-signatures` feature. OSS builds keep the shared
  signature types but no longer enable `warp-command-signatures/embed-signatures`.
- MCP runtime support is behind the non-OSS `mcp_runtime` app feature. OSS
  builds keep the MCP data model/UI types that still compile through the
  remaining AI module, but use the no-op manager path and no longer enable the
  app-side `rmcp` client, auth, HTTP/SSE, or child-process transport features.
- `git2`, `libgit2-sys`, and the now-unused `libz-sys` lockfile entries were
  removed by replacing the two direct `git2::Repository::discover` call sites
  with lightweight `.git` parent discovery.
- `warp_managed_secrets` is now behind the non-OSS
  `warp_managed_secrets` app feature. OSS builds use a local no-op shim for
  the manager/client/API surface that still compiles through auth/server/agent
  SDK modules, and no longer pull the managed-secrets crate or its HPKE/Tink
  support stack.
- `computer_use` is now behind the non-OSS `computer_use_runtime` app/AI
  feature. OSS builds use a small AI-crate shim for computer-use request/result
  types and no-op runtime hooks, so remaining AI modules can compile without
  pulling the platform automation crate.
- `warp_files` remote-file support is behind the non-OSS
  `remote_server_runtime` app feature and the `warp_files/remote_server`
  crate feature. OSS builds emit an explicit unsupported error for remote file
  save/delete requests when built without that feature.
- The app's direct `remote_server` dependency is behind the non-OSS
  `remote_server_runtime` feature. OSS builds keep a small local
  `crate::remote_server` compatibility shim for remote setup state,
  terminal-session bookkeeping, remote command/file request types, and the
  auth/SSH transport surfaces still referenced by terminal and AI modules.
  Remote-server CLI subcommands and runtime operations return unsupported/no-op
  behavior in OSS instead of pulling the real `remote_server` crate.

## Rejected Cuts

- A dedicated `release-oss` profile inheriting `release-lto` with `opt-level =
  "s"` and `codegen-units = 1` was measured and removed. It slowed packaging
  substantially and increased the app from 214M to 229M. The executable grew
  from 213M to 227M, largely because `__LINKEDIT` increased from 6,897,664
  bytes to 61,734,912 bytes.
- Disabling `rmcp` default features at the dependency declaration was tested
  and reverted. The upstream crate currently fails to compile without its
  default feature set because model/error modules still refer to default-gated
  service, base64, schema, and builder items.

## Verified Absent From OSS Feature Tree

This command produced no matches:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -e normal,build,features |
  rg "mcp_runtime|rmcp feature \"(auth|client|transport-streamable-http-client-reqwest|transport-sse-client-reqwest|transport-child-process|client-side-sse|__reqwest|reqwest)\"|embed-signatures|embedded-signatures|tantivy|ownedbytes|bitpacking|levenshtein_automata|sketches-ddsketch|datasketches|syntect|onig|aws-config|aws-credential-types|aws-sdk-sts|aws-types|aws-smithy|aws-runtime|warp_managed_secrets|managed_secrets|tink-|hpke|computer_use|computer-use|computer_use_runtime|remote_server v|input_classifier|natural_language_detection|voice_input|cpal|rubato|hound|nld_onnx|candle|tokenizers|command-signatures-v2|sentry|crash-handler|minidumper|git2|libgit2"
```

The inverse tree also reports that `remote_server` is not part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i remote_server
```

## Remaining Removal Targets

The OSS build still has these normal top-level dependencies. They remain the
main blockers to fully removing AI/auth/MCP/editor/git UI code from the binary:

```text
ai
languages
lsp
repo_metadata
rmcp
syntax_tree
warp_editor
warp_files
warp_graphql
```

Current anchors:

- `app/Cargo.toml` keeps these dependencies unconditional.
- `app/src/lib.rs` still keeps `mod ai`, `mod auth`, `mod code`, `mod code_review`, and `pub mod editor` unconditional.
- `ai` is now only pulled directly by `warp`; the previous `onboarding -> ai`
  edge is gone.
- The real `remote_server` crate is gone from the OSS tree; the remaining
  remote-server-shaped app code is a local shim that should disappear with the
  broader terminal/AI/auth cleanup.
- OSS startup registrations for MCP managers, code/editor state, codebase indexing, and managed secrets are now gated behind `not(oss_release)`.
- The gated constructors are the next dependency-removal anchors because they
  are no longer required by the OSS startup path. Managed secrets has already
  moved from a real dependency to a shim; the remaining work is to remove the
  auth/server/agent SDK modules that require the shim.

Verified inverse-tree blockers:

- `rmcp` is still reached through app AI modules. The app-side MCP runtime
  transport features are now gated off for OSS, but fully removing the crate
  still requires gating `app/src/ai/**` or splitting the app AI module into
  non-OSS features.
- `warp_editor` is reached directly from `warp` and indirectly via
  `languages` and `syntax_tree`; those are also pulled by `ai`.
- `mermaid_to_svg` remains in the OSS tree through both direct app notebook/AI
  rendering code and `warp_editor`; removing it cleanly should follow the
  editor/notebook/AI module gates rather than only making the app dependency
  optional.
- `warp_files` is reached directly from `warp`, with call sites in
  `app/src/code/**`, `app/src/notebooks/**`, `app/src/remote_server/**`, and
  `app/src/ai/**`. Its indirect `remote_server` edge is gone in OSS.
- `repo_metadata` is reached directly from `warp` and indirectly through
  `ai`, `lsp`, and `warp_files`.
- `warp_graphql` is reached directly from `warp` and indirectly through `ai`
  and `warp_server_client`.
- The local `crate::warp_managed_secrets` shim is still referenced by auth,
  server API adapters, and app AI agent SDK code. Those call sites are now
  compile-time compatible with OSS, but should disappear with the broader
  auth/server/AI module removal.

## Next Cut

Move each remaining dependency behind non-OSS Cargo features now that the OSS
startup roots are gated. Continue with MCP, codebase indexing, code review,
auth, editor/notebook, and AI agent models. After each group, rerun the same
`cargo tree` audit and remove any module declarations that become unused.

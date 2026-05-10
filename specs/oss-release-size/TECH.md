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
| `vфrp.app` | 368M | 93M |
| `Contents/MacOS/warp-oss` | 360M | 92M |
| `Contents/Resources` | 3.7M | 1.7M |

`xcrun size -m` on `Contents/MacOS/warp-oss` now reports:

| Section | Current |
| --- | ---: |
| `__TEXT` | 88,965,120 bytes |
| `__TEXT,__text` | 67,213,152 bytes |
| `__TEXT,__const` | 7,647,184 bytes |
| `__LINKEDIT` | 4,325,376 bytes |

The dSYM is preserved outside the app at `target/release-lto/bundle/osx/warp-oss.dSYM`
and is currently about 722M.

The current debug-speed iteration links `target/aarch64-apple-darwin/debug/warp-oss`
successfully with the same OSS feature set. Targeted string scans no longer find
the OSS-gated command/menu labels for new agent tabs, AI/Code/MCP/Environment
settings pages, MCP server collection opening, AI prompt creation, code review,
agent conversation list toggles, the environment setup selector, `/init`
AI/codebase/project setup copy, or the Oz cloud setup guide copy.

After replacing the legacy Warp AI panel, transcript, request model, rendering
utilities, command-search Warp AI data source, legacy terminal/menu entrypoints,
Resource Center AI command-search tip, Project Explorer file tree, main
CodeView, Code Review panel, Code Review comment rendering, and the AI context
menu with OSS shims/gates, Agent Mode code-diff rendering with OSS shims/gates,
the AI document/facts/execution-profile pane views with OSS shims/gates, and
the guided worktree/session-config onboarding surfaces with OSS shims/gates, the
app-local `repo_metadata` and `onboarding` compatibility shims, and the
AI-crate GraphQL conversion dependency gated out of OSS, the app
`server_api::ai` GraphQL implementation replaced with OSS unavailable/default
paths, and the app `server_api::auth` GraphQL implementation replaced with OSS
unavailable/default paths, and the app `server_api::team`/`server_api::workspace`
GraphQL implementations replaced with OSS unavailable/empty-metadata paths, the
app `server_api::referral`/`server_api::block`/`server_api::integrations`
GraphQL implementations replaced with OSS empty/unavailable paths, the debug
`warp-oss` executable is 440M with `__TEXT` at 178,438,144 bytes,
`__TEXT,__text` at 122,652,376 bytes, and `__LINKEDIT` at 278,315,008 bytes.
After replacing the app cloud-object server API implementation with OSS
empty/unavailable paths, the debug `warp-oss` executable is 427M with `__TEXT`
at 173,703,168 bytes, `__TEXT,__text` at 119,210,384 bytes, and `__LINKEDIT`
at 268,812,288 bytes.
After removing the remaining OSS GraphQL client request helper path and replacing
the OSS billing/API-key settings pages with no-op page shims, the debug
`warp-oss` executable is 422M with `__TEXT` at 172,113,920 bytes,
`__TEXT,__text` at 118,057,044 bytes, and `__LINKEDIT` at 265,584,640 bytes.
After replacing the terminal credit banner/auto-reload modal, workspace
billing/capacity modals, and Teams settings page with OSS no-op shims, the debug
`warp-oss` executable is 418M with `__TEXT` at 170,590,208 bytes,
`__TEXT,__text` at 116,939,348 bytes, and `__LINKEDIT` at 262,782,976 bytes.
After replacing the shared-object limit modal with an OSS no-op shim and gating
the workspace GraphQL conversion module out of OSS, the debug `warp-oss`
executable remains 418M with `__TEXT` at 170,491,904 bytes, `__TEXT,__text` at
116,874,324 bytes, and `__LINKEDIT` at 262,619,136 bytes. This delta is below
the threshold for a release bundle checkpoint.
After isolating managed-secrets and MCP gallery GraphQL value types behind local
app boundary structs, the debug `warp-oss` executable remains 418M with
`__TEXT` at 170,475,520 bytes, `__TEXT,__text` at 116,867,156 bytes, and
`__LINKEDIT` at 262,586,368 bytes. This is dependency-boundary cleanup, so no
release bundle was run.
After making the app's `warp_graphql` dependency optional behind the default
`ai_graphql_runtime` feature and moving the remaining OSS-facing GraphQL value
types behind app-local integration/artifact/AI-assist boundaries, the app no
longer has a direct `warp -> warp_graphql` edge in the OSS dependency graph.
`warp_graphql` is still reached indirectly through `warp_server_client`. The
debug `warp-oss` executable remains 418M with `__TEXT` at 170,475,520 bytes,
`__TEXT,__text` at 116,865,876 bytes, and `__LINKEDIT` at 262,553,600 bytes,
so no release bundle was run.
After making `warp_server_client`'s `warp_graphql` dependency optional behind
its default `graphql` feature and disabling that feature from the workspace/app
OSS edge, `warp_graphql` is no longer present in the OSS dependency graph. The
debug `warp-oss` executable remains 418M with `__TEXT` at 170,442,752 bytes,
`__TEXT,__text` at 116,852,588 bytes, and `__LINKEDIT` at 262,520,832 bytes,
so no release bundle was run.
After disabling the generated telemetry event-description catalog in OSS,
returning empty event descriptions, replacing OSS telemetry event names with a
generic disabled-telemetry name, and applying the same OSS behavior to the
feature-specific, app-local, and crate-level telemetry enums, the debug
`warp-oss` executable is 417M with `__TEXT` at 170,295,296 bytes,
`__TEXT,__text` at 116,777,068 bytes, `__TEXT,__const` at 12,315,872 bytes,
and `__LINKEDIT` at 262,373,376 bytes.
Targeted string checks no longer find the human-readable telemetry
names/descriptions that were previously retained for removed AI/MCP/billing/code
surfaces. This is still a debug-only checkpoint, so no release bundle was run.
After gating settings-schema inventory submissions behind an explicit
`settings_schema_registry` feature and leaving that feature out of the OSS app
binary, the debug `warp-oss` executable is 416M with `__TEXT` at 169,476,096
bytes, `__TEXT,__text` at 116,237,324 bytes, `__TEXT,__const` at 12,284,768
bytes, and `__LINKEDIT` at 261,423,104 bytes. The OSS schema generator still
builds and runs with `settings_schema_registry`, producing 184 settings for
bundle-time `settings_schema.json`, but the app binary no longer links the
inventory/static schema metadata. This remains below the threshold for a release
bundle checkpoint.
After gating the privacy settings cloud-conversation widget and AI analytics
copy out of OSS, and removing the matching dummy action from the OSS login-slide
stub, the debug `warp-oss` executable remains 416M with `__TEXT` at 169,476,096
bytes, `__TEXT,__text` at 116,231,692 bytes, `__TEXT,__const` at 12,284,768
bytes, and `__LINKEDIT` at 261,423,104 bytes. Targeted string checks no longer
find the OSS-inapplicable AI privacy copy or cloud-conversation action name.
This is below the threshold for a release bundle checkpoint.
After replacing the terminal inline AI conversation menu with an OSS no-op view,
the debug `warp-oss` executable is 415M with `__TEXT` at 169,148,416 bytes,
`__TEXT,__text` at 115,997,692 bytes, `__TEXT,__const` at 12,284,768 bytes,
and `__LINKEDIT` at 260,784,128 bytes. The real inline conversation data source,
search item, and navigation event are no longer linked into OSS. Remaining
conversation-list strings come from other AI conversation surfaces and should be
handled by broader agent/conversation-list gating. This is a debug-only
checkpoint, so no release bundle was run.
After replacing the command-palette conversation data source with an OSS empty
source, the debug `warp-oss` executable is 414M with `__TEXT` at 169,082,880
bytes, `__TEXT,__text` at 115,948,604 bytes, `__TEXT,__const` at 12,284,768
bytes, and `__LINKEDIT` at 260,620,288 bytes. The real command-palette
conversation searcher, search item, section labels, and recent-item lookup are
no longer linked into OSS. This remains a debug-only checkpoint.
After replacing the left-panel conversation list with an OSS no-op panel, the
debug `warp-oss` executable is 413M with `__TEXT` at 168,689,664 bytes,
`__TEXT,__text` at 115,660,512 bytes, `__TEXT,__const` at 12,276,576 bytes,
and `__LINKEDIT` at 259,915,776 bytes. The real conversation list view, item
rendering, view model, overflow/share actions, and empty-state UI are no longer
linked into OSS. Remaining `ConversationListView` strings are shared action or
snapshot enum metadata rather than the real panel implementation.
After limiting the OSS slash-command registry to terminal-safe commands and
gating non-OSS slash-command execution arms, the debug `warp-oss` executable
remains 413M with `__TEXT` at 168,591,360 bytes, `__TEXT,__text` at 115,595,204
bytes, `__TEXT,__const` at 12,272,480 bytes, and `__LINKEDIT` at 259,801,088
bytes. The OSS registry now omits AI/MCP/code-review/editor/git/billing
commands, and their direct execution handlers no longer instantiate command
metadata. Some AI slash-command strings remain through AI message-bar and
block-rendering helpers that still reference command constants; those should be
handled with a broader AI terminal-input/message-bar split.
After gating the terminal input message-bar AI hint producers, fork/init command
insertion references, agent-view message-bar producer chain, AI block
slash-prefix highlighting, and the AI slash-command controller parser/sender out
of OSS, the debug `warp-oss` executable remains 413M with `__TEXT` at
168,493,056 bytes, `__TEXT,__text` at 115,521,476 bytes, `__TEXT,__const` at
12,268,384 bytes, and `__LINKEDIT` at 259,670,016 bytes. The targeted scan no
longer finds the terminal message-bar `/agent`/`/plan` copy, the agent message
bar's conversation/fork/autodetect helper copy, or the slash-command
controller's request error strings. Remaining AI slash-command text is now
anchored by deeper agent SDK, zero-state/input-footer, orchestration, and asset
catalog paths that are still linked into OSS.
After replacing the agent-view zero-state block with an OSS no-op view, removing
AI/code-review/NLD rows from the terminal zero-state block in OSS, and making
the agent input footer render empty in OSS, the debug `warp-oss` executable is
412M with `__TEXT` at 168,263,680 bytes, `__TEXT,__text` at 115,346,616 bytes,
`__TEXT,__const` at 12,268,384 bytes, and `__LINKEDIT` at 259,276,800 bytes.
The targeted scan no longer finds the Oz zero-state body copy, the terminal
zero-state agent/cloud-agent shortcut copy, or the terminal zero-state
autodetection row. Remaining agent strings are now mostly debug metadata,
ambient-agent/SDK paths, orchestration widgets, broader AI module type names, and
use-agent toolbar button construction that still need structural removal.
After splitting the use-agent toolbar and Warpify footer constructors for OSS
and replacing the full agent input footer module with an OSS compatibility shim,
the debug `warp-oss` executable is 411M with `__TEXT` at 167,657,472 bytes,
`__TEXT,__text` at 114,913,660 bytes, `__TEXT,__const` at 12,264,288 bytes,
and `__LINKEDIT` at 258,113,536 bytes. The real agent footer constructor,
model/environment selectors, plugin chips, handoff controls, and CLI agent
footer controls are no longer linked into OSS. Targeted scans no longer find
the user-facing use-agent toolbar copy or the Full Terminal Agent model callout;
remaining matches are mostly action/type metadata and broader AI SDK/block
surfaces.
After replacing the ambient-agent host, harness, and model selector views with
OSS compatibility shims and switching the OSS `warp_cli` top-level help metadata
away from Oz/cloud-agent copy, the debug `warp-oss` executable is 410M with
`__TEXT` at 167,444,480 bytes, `__TEXT,__text` at 114,743,164 bytes,
`__TEXT,__const` at 12,260,192 bytes, and `__LINKEDIT` at 257,687,552 bytes.
The real selector dropdowns, model-search editor, harness availability menu,
and top-level Oz CLI help copy no longer link into OSS. The remaining Oz/cloud
CLI strings come from the command enum and agent SDK execution handlers, so a
larger follow-up should split the command graph itself rather than only hiding
or relabeling help text.
After removing the app's `LaunchMode::CommandLine` root and the flattened
`CommandLine` clap variant from OSS, the debug `warp-oss` executable is 384M
with `__TEXT` at 157,679,616 bytes, `__TEXT,__text` at 107,783,304 bytes,
`__TEXT,__const` at 11,968,368 bytes, and `__LINKEDIT` at 240,697,344 bytes.
This removes the full Oz CLI command graph and local agent SDK execution path
from the OSS app startup/completion roots. Targeted scans no longer find the
top-level Oz CLI help, harness-support subcommand help, or agent-driver runtime
strings; remaining harness-support matches are server API endpoint strings still
reachable through broader AI/server modules.
After replacing the remaining harness-support server API endpoints with OSS
unavailable paths while keeping the shared task-scoped request helpers for agent
messaging, the debug `warp-oss` executable remains 384M with `__TEXT` at
157,450,240 bytes, `__TEXT,__text` at 107,611,152 bytes, `__TEXT,__const` at
11,964,272 bytes, and `__LINKEDIT` at 240,418,816 bytes. Targeted scans no
longer find the harness-support endpoint paths or invalid conversation ID copy;
the only remaining harness-support match is the OSS unavailable error string.
This is below the threshold for a release bundle checkpoint.
After replacing task-scoped agent message server methods with OSS unavailable
paths and gating the shared task-scoped HTTP helpers back out of OSS, the debug
`warp-oss` executable remains 384M with `__TEXT` at 157,368,320 bytes,
`__TEXT,__text` at 107,546,248 bytes, `__TEXT,__const` at 11,968,368 bytes,
and `__LINKEDIT` at 240,320,512 bytes. Targeted scans no longer find
`agent/messages` request paths; the remaining match is the OSS unavailable
agent-messaging error string. This is below the threshold for a release bundle
checkpoint.
After adding an OSS mode to the feature-flag crate that keeps flag values but
collapses `Debug` output and preview descriptions, the debug `warp-oss`
executable remains 384M with `__TEXT` at 157,319,168 bytes, `__TEXT,__text` at
107,529,092 bytes, `__TEXT,__const` at 11,949,552 bytes, and `__LINKEDIT` at
240,304,128 bytes. Targeted scans no longer find the large feature-flag name
run or Preview-only AI/Oz/MCP/code-review flag descriptions in OSS. This is a
debug-only metadata cleanup, so no release bundle was run.
After reducing the OSS settings sidebar to terminal-safe pages and collapsing
removed settings-section display/parse/debug names, the debug `warp-oss`
executable remains 384M with `__TEXT` at 157,319,168 bytes, `__TEXT,__text` at
107,523,972 bytes, `__TEXT,__const` at 11,949,552 bytes, and `__LINKEDIT` at
240,304,128 bytes. Targeted scans no longer find the settings sidebar label
runs for Agents, MCP Servers, Code Review, Environments, or Oz Cloud API Keys;
direct navigation to removed sections falls back to Appearance in OSS.
After collapsing OSS server-experiment debug/display/parse metadata for removed
AI/Oz/code experiment arms, the debug `warp-oss` executable remains 384M with
`__TEXT` at 157,302,784 bytes, `__TEXT,__text` at 107,521,668 bytes,
`__TEXT,__const` at 11,945,456 bytes, and `__LINKEDIT` at 240,304,128 bytes.
Targeted scans no longer find the long server-experiment variant-name run or
the removed experiment IDs; unsupported OSS experiment arms serialize as a
single disabled-server-experiment label.
After making `LLMPreferences` server model refreshes no-op in OSS, the debug
`warp-oss` executable is 383M with `__TEXT` at 157,270,016 bytes,
`__TEXT,__text` at 107,495,044 bytes, `__TEXT,__const` at 11,949,552 bytes,
and `__LINKEDIT` at 240,238,592 bytes. Targeted scans no longer find the LLM
server-fetch error strings or direct feature-model/free-tier model request path
strings. Remaining LLM matches are serde/type/cache compatibility for model
metadata still referenced by shared terminal and agent surfaces.
After gating the cached-model serde path out of OSS while preserving
`LLMModelHost` serde for workspace settings, the debug `warp-oss` executable
remains 383M with `__TEXT` at 157,089,792 bytes, `__TEXT,__text` at
107,355,516 bytes, `__TEXT,__const` at 11,945,456 bytes, and `__LINKEDIT` at
239,960,064 bytes. Targeted scans no longer find the cached-model serde field
name run, `WireLLMInfo`, or cached LLM serialization/deserialization errors.
After disabling the dormant Claude wake path in OSS, the debug `warp-oss`
executable remains 383M with `__TEXT` at 156,942,336 bytes, `__TEXT,__text`
at 107,245,180 bytes, `__TEXT,__const` at 11,945,472 bytes, and
`__LINKEDIT` at 239,763,456 bytes. Targeted scans no longer find the dormant
Claude wake strings, wake prompt file name, or `wake_driver` path in OSS.
After treating third-party agent SDK harnesses as unsupported in OSS and
gating their concrete runner/transcript modules, the debug `warp-oss`
executable is 380M with `__TEXT` at 156,106,752 bytes, `__TEXT,__text` at
106,651,716 bytes, `__TEXT,__const` at 11,927,936 bytes, and `__LINKEDIT` at
238,272,512 bytes. Targeted scans no longer find the Claude transcript
session-index strings, Claude/Codex/Gemini harness runner names, or
`claude_code_cli` format string in OSS.
After gating CLI-agent plugin manager implementations out of OSS, the debug
`warp-oss` executable remains 380M with `__TEXT` at 156,057,600 bytes,
`__TEXT,__text` at 106,621,508 bytes, `__TEXT,__const` at 11,923,840 bytes,
and `__LINKEDIT` at 238,223,360 bytes. Targeted scans no longer find the
Claude, Codex, Gemini, or OpenCode plugin install/update instruction bodies in
OSS.
After gating the Codex promo modal and its root/URI actions out of OSS, the
debug `warp-oss` executable remains 380M with `__TEXT` at 156,008,448 bytes,
`__TEXT,__text` at 106,585,412 bytes, `__TEXT,__const` at 11,923,840 bytes,
and `__LINKEDIT` at 238,157,824 bytes. Targeted scans no longer find the
Codex modal body, root open actions, or initial Codex prompt string in OSS.
After removing the OpenCode plugin debug workspace actions from OSS, the debug
`warp-oss` executable remains 380M with `__TEXT` at 155,992,064 bytes,
`__TEXT,__text` at 106,573,088 bytes, `__TEXT,__const` at 11,923,840 bytes,
and `__LINKEDIT` at 238,141,440 bytes. Targeted scans no longer find the
OpenCode plugin debug action bindings or JSON config helper strings in OSS.
After replacing local third-party child harness launch support with an OSS
unavailable stub, the debug `warp-oss` executable remains 380M with `__TEXT`
at 155,975,680 bytes, `__TEXT,__text` at 106,554,400 bytes,
`__TEXT,__const` at 11,923,840 bytes, and `__LINKEDIT` at 238,108,672 bytes.
Targeted scans no longer find the concrete local Claude, Codex, or OpenCode
child launch commands, OpenCode install-doc URL, or local child harness shell
validation/setup strings in OSS.
After excluding third-party CLI-agent logo SVG payloads from the OSS embedded
asset set, the debug `warp-oss` executable remains 380M with `__TEXT` at
155,910,144 bytes, `__TEXT,__text` at 106,554,400 bytes, `__TEXT,__const` at
11,866,496 bytes, and `__LINKEDIT` at 238,108,672 bytes. Targeted scans still
find shared icon path names from the icon enum, but no longer find the excluded
SVG payload contents in OSS.
After excluding the remaining AI-specific embedded SVG payloads for agent mode,
cloud-agent UI, prompt/conversation icons, context-window meters, conversation
context meters, and loading-agent animations, the debug `warp-oss` executable
remains 380M with `__TEXT` at 155,582,464 bytes, `__TEXT,__text` at
106,554,400 bytes, `__TEXT,__const` at 11,534,720 bytes, and `__LINKEDIT` at
238,092,288 bytes. Targeted scans no longer find those SVG payload bodies in
OSS; shared icon path names may still remain through enum display metadata.
After replacing cloud-mode loading tips with an empty OSS list and moving the
slash-command metadata table to an OSS-specific shim that only registers
terminal-safe commands, the debug `warp-oss` executable remains 380M with
`__TEXT` at 155,566,080 bytes, `__TEXT,__text` at 106,548,768 bytes,
`__TEXT,__const` at 11,530,624 bytes, and `__LINKEDIT` at 238,092,288 bytes.
Targeted scans no longer find the Oz/cloud-agent loading tip copy, cloud-agent
docs URLs, AI/MCP/billing slash-command descriptions, or the slash-command
default binding labels in OSS. Some adjacent AI strings remain from deeper
agent block, orchestration, URI, and debug metadata paths that are still linked.
After gating the cloud-agent/create-environment URI action parser and root
actions out of OSS, and collapsing `TerminalAction`/`InputAction` debug
formatting plus AI-only input keybinding registrations in OSS, the debug
`warp-oss` executable remains 380M with `__TEXT` at 155,451,392 bytes,
`__TEXT,__text` at 106,465,224 bytes, `__TEXT,__const` at 11,522,432 bytes,
and `__LINKEDIT` at 237,944,832 bytes. Targeted scans no longer find the
cloud-agent/create-environment deeplink action strings, root create-environment
action registrations, terminal action debug names for AI/MCP/environment flows,
or AI input binding labels in OSS. Some binding names and type paths still
remain through the broader keybinding/settings registries and linked AI/editor
compatibility modules.
After collapsing `WorkspaceAction` debug formatting in OSS while preserving the
detailed formatter in non-OSS builds, the debug `warp-oss` executable is 379M
with `__TEXT` at 155,418,624 bytes, `__TEXT,__text` at 106,442,440 bytes,
`__TEXT,__const` at 11,514,240 bytes, and `__LINKEDIT` at 237,912,064 bytes.
Targeted scans no longer find the derived workspace action variant names, but
command binding/custom action strings and type paths still remain through the
shared action registry.
After also collapsing `CustomAction`, `DriveIndexAction`, `AIBlockAction`, and
`AIDocumentAction` debug formatting in OSS, the debug `warp-oss` executable
remains 379M with `__TEXT` at 155,402,240 bytes, `__TEXT,__text` at 106,431,432
bytes, `__TEXT,__const` at 11,510,144 bytes, and `__LINKEDIT` at 237,912,064
bytes. This removes more derived action variant-name payloads, though several
action labels still remain through typed action/menu metadata and explicit
diagnostic strings.
After neutralizing OSS-only shim view/debug names for the document, rules,
assistant panel, agent management, AI settings, MCP settings, and AI context
menu placeholders, plus replacing explicit DriveIndex diagnostic action names
with generic text, the debug `warp-oss` executable remains 379M with `__TEXT`
at 155,402,240 bytes, `__TEXT,__text` at 106,428,360 bytes, `__TEXT,__const`
at 11,510,144 bytes, and `__LINKEDIT` at 237,912,064 bytes. Targeted scans no
longer find the OSS shim UI names or `Use DriveIndexAction::...` diagnostics;
the remaining hits are mostly type paths, telemetry/settings schema fields, and
linked placeholder model names.
After applying the same neutral debug/UI-name treatment to the OSS code settings
page, code pane shim, file tree shim, and code review shim, replacing one
explicit CodeReviewView working-directory diagnostic, and serializing removed
telemetry metadata enums as a generic disabled value in OSS, the debug
`warp-oss` executable remains 379M with `__TEXT` at 155,385,856 bytes,
`__TEXT,__text` at 106,420,168 bytes, `__TEXT,__const` at 11,510,144 bytes,
and `__LINKEDIT` at 237,912,064 bytes. Targeted scans no longer find direct
`code_view:save`, `Reviewing code changes`, or CodeReviewView diagnostic
strings; the remaining AI/code-review hits are primarily type paths, shared
settings/action compatibility names, and still-linked agent/editor placeholder
models.
After replacing the web search/fetch, codebase search result, suggested unit
test, run-agents confirmation, and requested-command inline-action views with
OSS no-op compatibility shims, the debug `warp-oss` executable is 378M with
`__TEXT` at 154,714,112 bytes, `__TEXT,__text` at 105,916,872 bytes,
`__TEXT,__const` at 11,506,048 bytes, and `__LINKEDIT` at 236,699,648 bytes.
This removes the real AI inline-action rendering, buttons, editor integration,
selection/render helpers, and orchestration picker UI from the OSS build while
keeping the event/type surfaces needed by the still-linked AI block model. Some
view names still appear through Rust type paths, and one command-confirmation
copy string remains through the env-var collection block rather than the
requested-command view.
After replacing the ask-user-question inline-action view with an OSS no-op
compatibility shim, the debug `warp-oss` executable is 377M with `__TEXT` at
154,517,504 bytes, `__TEXT,__text` at 105,784,520 bytes, `__TEXT,__const` at
11,501,952 bytes, and `__LINKEDIT` at 236,371,968 bytes. This removes the real
questionnaire state machine, markdown answer rendering, shortcut registration,
input editor, focus handling, and completion-summary UI from OSS. Targeted
scans no longer find the `AskUserQuestionView` type name or its user-facing
questionnaire copy; only neutral OSS shim type metadata remains through the
still-linked AI block compatibility surface. This is a debug-only checkpoint,
so no release bundle was run.
After replacing the prompt alert and agent plan/todo context-chip views with
OSS no-op compatibility shims, the debug `warp-oss` executable remains 377M
rounded, with `__TEXT` at 154,435,584 bytes, `__TEXT,__text` at 105,720,776
bytes, `__TEXT,__const` at 11,501,952 bytes, and `__LINKEDIT` at 236,240,896
bytes. This removes the real AI request-limit/analytics/billing alert rendering,
model subscriptions, hyperlink formatting, plan chip, todo popup, document
dirty indicator, and todo button UI from OSS while keeping the event/action
surfaces expected by terminal input and context-chip compatibility code.
Targeted scans no longer find `PromptAlertView`, `PlanAndTodoListView`, or
their AI-specific alert/todo copy. Remaining `Out of credits` and plan-label
strings are anchored by other terminal/telemetry/shared-session surfaces. This
is below the threshold for a release bundle checkpoint.
After replacing the child-agent status card and orchestration pill bar with OSS
no-op compatibility shims, the debug `warp-oss` executable is 376M with
`__TEXT` at 154,222,592 bytes, `__TEXT,__text` at 105,554,592 bytes,
`__TEXT,__const` at 11,501,952 bytes, and `__LINKEDIT` at 235,896,832 bytes.
This removes the real child-agent status rows, dismiss state tracking,
orchestration pill menu, hover cards, breadcrumb rendering, avatar/status chip
rendering, pane/tab focus actions, and related model subscriptions from OSS.
Targeted scans no longer find `ChildAgentStatusCard`, pill-bar positioning
strings, menu labels, or child-card diagnostics. One `OrchestrationPillBar`
string remains through shared feature/telemetry metadata rather than the real
view implementation. This is still a debug-only checkpoint, so no release
bundle was run.
After replacing the agent message bar and inline agent-view rich-content header
with OSS no-op compatibility shims, the debug `warp-oss` executable remains
376M rounded, with `__TEXT` at 154,173,440 bytes, `__TEXT,__text` at
105,517,728 bytes, `__TEXT,__const` at 11,501,952 bytes, and `__LINKEDIT` at
235,782,144 bytes. This removes the real agent-message producers, Figma MCP
CTA chip, fork/resume/plan/code-review shortcut hints, autodetected shell-mode
copy, inline agent control header, and the header's action/history
subscriptions from OSS. Targeted scans no longer find `AgentMessageBar`,
`AgentMessageArgs`, or the removed agent header/message copy. Remaining
`InlineAgentViewHeader` and `Starting shell...` strings are anchored by
terminal rich-content metadata and generic terminal startup messaging rather
than the removed agent-view implementations. This is below the threshold for a
release bundle checkpoint.
After replacing the CLI subagent block view with an OSS no-op compatibility
shim, the debug `warp-oss` executable is 375M with `__TEXT` at 153,796,608
bytes, `__TEXT,__text` at 105,241,556 bytes, `__TEXT,__const` at 11,497,856
bytes, and `__LINKEDIT` at 235,225,088 bytes. This removes the real CLI
subagent renderer, blocked-action permission UI, allow/refine/take-over buttons,
auto-approve keybinding contexts, code-editor snippets, table sections, output
selection plumbing, speedbump checkboxes, and feedback-doc link handling from
OSS while keeping the terminal view's subagent event surface compiling.
Targeted scans no longer find `CLISubagentView`, its keybinding context names,
permission prompt copy, or saved-position IDs. Remaining `Copied to clipboard`
and feedback-doc strings are anchored by generic AI block/plugin instruction
paths rather than the removed CLI subagent view. This is a debug-only
checkpoint, so no release bundle was run.
After replacing the agent-view controller with an OSS permanently-inactive
compatibility shim, the debug `warp-oss` executable remains 375M rounded, with
`__TEXT` at 153,763,840 bytes, `__TEXT,__text` at 105,218,260 bytes,
`__TEXT,__const` at 11,497,856 bytes, and `__LINKEDIT` at 235,225,088 bytes.
This removes the real agent-view enter/exit state machine, pending
confirmation timers, ephemeral confirmation messages, conversation start/active
conversation mutations, long-running-command exit checks, and controller event
emission paths from OSS while preserving the inactive state/query API used by
terminal layout code. Targeted scans no longer find the removed
enter/exit/new-conversation confirmation copy or controller-specific
agent-mode error strings. This is a debug-only checkpoint, so no release bundle
was run.
After replacing the agent-view ephemeral message model with an OSS no-op
compatibility shim, the debug `warp-oss` executable remains 375M rounded, with
`__TEXT` at 153,763,840 bytes, `__TEXT,__text` at 105,210,068 bytes,
`__TEXT,__const` at 11,497,856 bytes, and `__LINKEDIT` at 235,175,936 bytes.
This removes the real agent ephemeral-message storage, timer cancellation,
message-provider output, and `MessageChanged` event path from OSS. Targeted
scans no longer find the blocked new-conversation ephemeral warning or
`MessageChanged` event name. The remaining `again to send to agent` string is
anchored by terminal agent-entry code that is still compiled, not by the removed
ephemeral message model. This is a debug-only checkpoint, so no release bundle
was run.
After replacing the terminal agent-entry helper module with an OSS no-op
compatibility shim, the debug `warp-oss` executable remains 375M rounded, with
`__TEXT` at 153,698,304 bytes, `__TEXT,__text` at 105,171,412 bytes,
`__TEXT,__const` at 11,497,856 bytes, and `__LINKEDIT` at 235,094,016 bytes.
This removes the terminal-side agent-entry conversation loading path, pending
context attachment persistence, initial prompt auto-submit/draft path,
agent-entry telemetry emission, and real AgentViewEntry rich-content insertion
from OSS. Targeted scans no longer find `again to send to agent` or the
cloud-conversation load failure copy from this module. Remaining
new-conversation and passive-code-diff entry error strings are anchored by other
compiled terminal/input/command-palette paths that still reference agent entry.
This is a debug-only checkpoint, so no release bundle was run.
After gating the remaining OSS command-palette and terminal-input
new-agent-conversation/image-entry branches, the debug `warp-oss` executable
remains 375M rounded, with `__TEXT` at 153,681,920 bytes, `__TEXT,__text` at
105,157,844 bytes, `__TEXT,__const` at 11,497,856 bytes, and `__LINKEDIT` at
235,061,248 bytes. This removes the command-palette StartNewConversation
dispatch path, terminal image-add auto-entry attempt, and terminal
zero-state/new-agent-conversation action path from OSS. Targeted scans no
longer find the agent-monitoring toast copy, image-add entry failure log, or
zero-state new-agent failure log. Remaining `Failed to enter agent view` and
`AgentHarness flag is disabled` strings are anchored by passive-code-diff,
conversation-loader, and pane-group paths still compiled in OSS. This is a
debug-only checkpoint, so no release bundle was run.
After gating the OSS passive prompt/diff agent-continuation branches, the debug
`warp-oss` executable remains 375M rounded, with `__TEXT` at 153,665,536 bytes,
`__TEXT,__text` at 105,144,260 bytes, `__TEXT,__const` at 11,497,856 bytes,
and `__LINKEDIT` at 235,044,864 bytes. This removes the passive prompt
agent-entry path and the passive code-diff continue-with-agent path from OSS,
including passive-result sends/queues that depend on an active agent
conversation. Targeted scans no longer find the passive-code-diff agent-entry
failure log. The remaining generic `Failed to enter agent view` string is
anchored by context-model paths that still expose compatibility entrypoints.
This is a debug-only checkpoint, so no release bundle was run.
After gating restored CLI-agent snapshot/conversation branches out of OSS, the
debug `warp-oss` executable remains 375M rounded, with `__TEXT` at 153,649,152
bytes, `__TEXT,__text` at 105,133,508 bytes, `__TEXT,__const` at 11,493,760
bytes, and `__LINKEDIT` at 235,028,480 bytes. This removes the OSS restore path
for third-party CLI-agent block snapshots, transcript pane replacement, and
non-Oz conversation-loader block-snapshot fetches. Targeted scans no longer
find the `AgentHarness flag is disabled` guard strings, CLI-agent conversation
ignore copy, or non-Oz conversation ignore copy. This is a debug-only
checkpoint, so no release bundle was run.
After gating OSS context-model agent-entry compatibility attempts, the debug
`warp-oss` executable remains 375M rounded, with `__TEXT` at 153,649,152 bytes,
`__TEXT,__text` at 105,129,924 bytes, `__TEXT,__const` at 11,497,856 bytes,
and `__LINKEDIT` at 235,012,096 bytes. This preserves pending-query state
updates while removing OSS calls into the inactive agent-view controller from
context-model helper methods. Targeted scans no longer find the generic
`Failed to enter agent view` log strings. This is a debug-only checkpoint, so
no release bundle was run.
After gating the historical CLI-agent restoration enum variant out of OSS, the
debug `warp-oss` executable remains 375M rounded, with `__TEXT` at 153,649,152
bytes, `__TEXT,__text` at 105,128,388 bytes, `__TEXT,__const` at 11,493,760
bytes, and `__LINKEDIT` at 235,012,096 bytes. This removes OSS metadata and
restore matching for `HistoricalCLIAgent`/`CLIAgentConversation`; targeted scans
no longer find those variant/type names in the OSS binary. This is a
debug-only checkpoint, so no release bundle was run.
After replacing `AuthManager` with an OSS no-op compatibility shim, the debug
`warp-oss` executable is 374M with `__TEXT` at 153,403,392 bytes,
`__TEXT,__text` at 104,949,892 bytes, `__TEXT,__const` at 11,493,760 bytes,
and `__LINKEDIT` at 234,651,648 bytes. This removes the real auth redirect,
Firebase refresh/exchange, anonymous-user creation/linking, device-code auth,
auth URL construction, login telemetry, user-persistence side effects, and
auth-manager event emission paths from OSS while preserving the singleton/event
API used by root/workspace/settings/terminal compatibility code. Targeted scans
no longer find the remote login/signup/login-options/link-SSO URL paths, device
authorization event name, or custom-token helper names in the OSS binary. This
is a debug-only checkpoint, so no release bundle was run.
After replacing `AuthState` with an OSS logged-out compatibility shim, the
debug `warp-oss` executable remains 374M rounded, with `__TEXT` at 153,337,856
bytes, `__TEXT,__text` at 104,901,764 bytes, `__TEXT,__const` at 11,489,664
bytes, and `__LINKEDIT` at 234,553,344 bytes. This removes persisted-user
loading, API-key authentication setup, Firebase credential storage, user-profile
accessors, anonymous-limit calculations, reauth mutation state, and secure
storage persistence decisions from OSS while preserving the logged-out
`AuthStateProvider` API and anonymous experiment ID used by shared telemetry and
server compatibility code. Targeted scans no longer find `PersistedUser`,
`WARP_USER_SECRET`, or anonymous-user creation copy in the OSS binary. Remaining
auth strings are now anchored by shared credential/user/server compatibility
types and generic login/logout UI/actions rather than the real auth-state
implementation. This is a debug-only checkpoint, so no release bundle was run.
After replacing the OSS credential and user type modules with compatibility
shims, the debug `warp-oss` section sizes remain effectively unchanged at
`__TEXT` 153,337,856 bytes, `__TEXT,__text` 104,901,764 bytes, `__TEXT,__const`
11,489,664 bytes, and `__LINKEDIT` 234,553,344 bytes. This removes the real
Firebase/API-key token helper methods, Firebase endpoint URL builders, user
metadata helpers, principal/anonymous-user conversion metadata, and personal
object-limit type metadata from OSS while keeping the shared auth type surface
for server and UI compatibility code. Targeted scans no longer find
`FirebaseAuthTokens`, `FirebaseToken`, `LoginToken`, `RefreshToken`,
`AuthOwnerType`, `PrincipalType`, `AnonymousUserType`, `PersonalObjectLimits`,
`UserMetadata`, or the Firebase token/proxy endpoint URL strings in the OSS
binary. This is metadata cleanup, so no release bundle was run.
After replacing the local Firebase response module with an OSS compatibility
shim, the debug `warp-oss` section sizes remain effectively unchanged at
`__TEXT` 153,337,856 bytes, `__TEXT,__text` 104,901,764 bytes, `__TEXT,__const`
11,489,664 bytes, and `__LINKEDIT` 234,553,344 bytes. This removes the real
Firebase account-info parser, provider-profile lookup, access-token response
payload names, and Firebase-specific display text from OSS while preserving the
minimal response/error types referenced by the auth server API trait surface.
Targeted scans no longer find `FetchAccessTokenResponse`,
`GetAccountInfoResponse`, `ProviderUserInfo`, `AccountInfo`, the missing-email
error, or the Firebase request failure display string in the OSS binary. This
is metadata cleanup, so no release bundle was run.
After collapsing OSS `UserAuthenticationError` display/actionability and gating
Firebase access-token classifier constants out of OSS, the debug `warp-oss`
section sizes remain effectively unchanged at `__TEXT` 153,337,856 bytes,
`__TEXT,__text` 104,900,740 bytes, `__TEXT,__const` 11,493,760 bytes, and
`__LINKEDIT` 234,553,344 bytes. The OSS enum keeps the same variants for
root/auth UI pattern matches but no longer embeds Firebase token error
descriptions, auth redirect state error descriptions, or refresh-token exchange
classification strings. Targeted scans no longer find the Firebase token/user
error strings, `TOKEN_EXPIRED`/`INVALID_REFRESH_TOKEN`/`MISSING_REFRESH_TOKEN`,
`USER_DISABLED`/`USER_NOT_FOUND`, or auth-redirect state error strings in the
OSS binary. This is metadata cleanup, so no release bundle was run.
Remaining Agent Management strings in debug scans come from telemetry/action
catalogs, not the real management view or notification UI bodies. Remaining
`CodeEditorView` strings are now anchored by other generic editor/notebook/AI
block surfaces rather than the main CodeView, Code Review panel, or Agent Mode
code-diff view paths.
After replacing the OSS code-editor, local-code-editor, global-buffer-model,
diff-viewer, and network-log pane implementations with no-op compatibility
shims, the debug `warp-oss` executable is 379,627,928 bytes with `__TEXT` at
149,061,632 bytes, `__TEXT,__text` at 101,843,708 bytes,
`__TEXT,__const` at 11,438,592 bytes, and `__LINKEDIT` at 226,131,968 bytes.
The OSS app dep-info no longer lists the real `code/editor`, `local_code_editor`,
`global_buffer_model`, `find_references_view`, `language_server_extension`,
`diff_viewer`, `inline_diff`, or `server/network_log_view` source files.
`warp_editor` is still compiled through generic editor/notebook surfaces and
the OSS compatibility type signatures, so fully removing the editor crate
requires a broader split of shared text-editor/notebook APIs.
After replacing the MCP tool/resource action executors with OSS no-op
compatibility shims, the debug `warp-oss` executable is 379,625,784 bytes with
`__TEXT` still at 149,061,632 bytes, `__TEXT,__text` at 101,842,600 bytes, and
`__LINKEDIT` at 226,131,968 bytes. The OSS dep-info now points at
`call_mcp_tool_oss.rs` and `read_mcp_resource_oss.rs`, while the default build
still compiles the real MCP executor modules.
After gating the Warp Drive MCP server and Rules row item implementations out
of OSS, the debug `warp-oss` executable is 379,594,312 bytes with `__TEXT` at
149,045,248 bytes, `__TEXT,__text` at 101,834,408 bytes, and `__LINKEDIT` at
226,115,584 bytes. The OSS dep-info no longer lists
`drive/items/mcp_server*.rs` or `drive/items/ai_fact*.rs`; the default-feature
build still compiles those real row item modules.
After gating the workspace AI Rules view registration/open path out of OSS and
replacing `AIFactManager` with an OSS no-op compatibility shim, the debug
`warp-oss` executable is 379,523,928 bytes with `__TEXT` at 149,012,480 bytes,
`__TEXT,__text` at 101,815,452 bytes, `__TEXT,__const` at 11,438,592 bytes, and
`__LINKEDIT` at 226,082,816 bytes. The OSS dep-info now points at
`ai/facts/manager_oss.rs` and `ai/facts/view_oss.rs`, while the default-feature
build still compiles the real `ai/facts/manager.rs` and AI Rules view modules.
After replacing the AI Fact pane adapter with an OSS placeholder pane, the
debug `warp-oss` executable is 378,723,240 bytes with `__TEXT` at 148,750,336
bytes, `__TEXT,__text` at 101,622,428 bytes, `__TEXT,__const` at 11,438,592
bytes, and `__LINKEDIT` at 225,558,528 bytes. The OSS dep-info now points at
`pane/ai_fact_pane_oss.rs`, while the default-feature build still compiles the
real `pane/ai_fact_pane.rs`.
After replacing the AI Document pane adapter with an OSS placeholder pane, the
debug `warp-oss` executable is 377,933,880 bytes with `__TEXT` at 148,488,192
bytes, `__TEXT,__text` at 101,431,964 bytes, `__TEXT,__const` at 11,438,592
bytes, and `__LINKEDIT` at 225,034,240 bytes. The OSS dep-info now points at
`pane/ai_document_pane_oss.rs`, while the default-feature build still compiles
the real `pane/ai_document_pane.rs`.
After replacing the code-diff pane adapter with an OSS placeholder pane and
gating its event-forwarding model out of OSS, the debug `warp-oss` executable
is 377,077,864 bytes with `__TEXT` at 148,209,664 bytes, `__TEXT,__text` at
101,237,660 bytes, `__TEXT,__const` at 11,434,496 bytes, and `__LINKEDIT` at
224,460,800 bytes. The OSS dep-info now points at `pane/code_diff_pane_oss.rs`
and no longer lists `pane/code_diff_pane.rs` or `pane/code_diff_pane_model.rs`;
the default-feature build still compiles the real pane adapter and model.
After replacing the code pane adapter with an OSS placeholder pane, the debug
`warp-oss` executable is 376,242,472 bytes with `__TEXT` at 147,914,752 bytes,
`__TEXT,__text` at 101,025,388 bytes, `__TEXT,__const` at 11,434,496 bytes, and
`__LINKEDIT` at 223,920,128 bytes. The OSS dep-info now points at
`pane/code_pane_oss.rs`, while the default-feature build still compiles the real
`pane/code_pane.rs`.
After replacing `code/editor_management.rs` with an OSS compatibility shim, the
debug `warp-oss` executable is 376,053,768 bytes with `__TEXT` at 147,865,600
bytes, `__TEXT,__text` at 100,995,948 bytes, `__TEXT,__const` at 11,434,496
bytes, and `__LINKEDIT` at 223,772,672 bytes. The OSS dep-info now points at
`code/editor_management_oss.rs`, while the default-feature build still compiles
the real `code/editor_management.rs`.
After replacing the workspace right-panel/code-review surface with an OSS
no-op right panel, the debug `warp-oss` executable is 375,234,104 bytes with
`__TEXT` at 147,587,072 bytes, `__TEXT,__text` at 100,792,656 bytes,
`__TEXT,__const` at 11,430,400 bytes, and `__LINKEDIT` at 223,232,000 bytes.
The OSS dep-info now points at `workspace/view/right_panel_oss.rs`, while the
default-feature build still compiles the real `workspace/view/right_panel.rs`.
After replacing the Code Review diff-state model/parser/watcher implementation
with an OSS compatibility shim, the debug `warp-oss` executable is 374,405,016
bytes with `__TEXT` at 147,259,392 bytes, `__TEXT,__text` at 100,556,864
bytes, `__TEXT,__const` at 11,426,304 bytes, and `__LINKEDIT` at 222,740,480
bytes. The OSS dep-info now points at `code_review/diff_state_oss.rs`, while
the default-feature build still compiles the real `code_review/diff_state.rs`.
After replacing the git-status update model with an OSS no-op compatibility
shim and making terminal git-status subscriptions no-op in OSS, the debug
`warp-oss` executable is 373,982,968 bytes with `__TEXT` at 147,111,936 bytes,
`__TEXT,__text` at 100,448,652 bytes, `__TEXT,__const` at 11,426,304 bytes,
and `__LINKEDIT` at 222,478,336 bytes. The OSS dep-info now points at
`code_review/git_status_update_oss.rs`, while the default-feature build still
compiles the real watcher-backed `code_review/git_status_update.rs`.
After replacing the Code Review diff-set AI attachment helper module with an
empty OSS shim and gating the terminal attach-diffset event path out of OSS, the
debug `warp-oss` executable is 373,895,512 bytes with `__TEXT` at 147,079,168
bytes, `__TEXT,__text` at 100,423,564 bytes, `__TEXT,__const` at 11,426,304
bytes, and `__LINKEDIT` at 222,412,800 bytes. The OSS dep-info now points at
`code_review/context_oss.rs`, while the default-feature build still compiles
the real `code_review/context.rs`.
After replacing the legacy and MAA passive-suggestions models with OSS no-op
compatibility models, the debug `warp-oss` executable is 372,449,592 bytes with
`__TEXT` at 146,604,032 bytes, `__TEXT,__text` at 100,075,108 bytes,
`__TEXT,__const` at 11,422,208 bytes, and `__LINKEDIT` at 221,462,528 bytes.
The OSS dep-info now points at `ai/blocklist/passive_suggestions_oss.rs` and no
longer lists `ai/blocklist/passive_suggestions/legacy.rs`,
`ai/blocklist/passive_suggestions/maa.rs`, or
`ai/blocklist/passive_suggestions/static_prompt_suggestions.rs`; the
default-feature build still compiles the real passive-suggestions modules.
After replacing the suggested agent-mode workflow modal with an OSS no-op
compatibility modal, the debug `warp-oss` executable is 372,347,992 bytes with
`__TEXT` at 146,554,880 bytes, `__TEXT,__text` at 100,046,948 bytes,
`__TEXT,__const` at 11,418,112 bytes, and `__LINKEDIT` at 221,413,376 bytes.
The OSS dep-info now points at
`ai/blocklist/suggested_agent_mode_workflow_modal_oss.rs`, while the
default-feature build still compiles the real
`ai/blocklist/suggested_agent_mode_workflow_modal.rs`.
After replacing the suggested rule/workflow chip view with an OSS compatibility
view and gating suggested-chip creation out of OSS AI blocks, the debug
`warp-oss` executable is 372,175,704 bytes with `__TEXT` at 146,505,728 bytes,
`__TEXT,__text` at 100,010,084 bytes, `__TEXT,__const` at 11,418,112 bytes,
and `__LINKEDIT` at 221,282,304 bytes. The OSS dep-info now points at
`ai/blocklist/suggestion_chip_view_oss.rs`, while the default-feature build
still compiles the real `ai/blocklist/suggestion_chip_view.rs`.
After replacing the summarization cancel dialog with an OSS empty dialog, the
debug `warp-oss` executable is 372,168,536 bytes with `__TEXT` at 146,505,728
bytes, `__TEXT,__text` at 100,002,148 bytes, `__TEXT,__const` at 11,418,112
bytes, and `__LINKEDIT` at 221,282,304 bytes. The OSS dep-info now points at
`ai/blocklist/summarization_cancel_dialog_oss.rs`, while the default-feature
build still compiles the real `ai/blocklist/summarization_cancel_dialog.rs`.
After replacing the AI prediction server endpoints, relevant-file controller,
and voice/transcription server paths with OSS unavailable/no-op shims, the
debug `warp-oss` executable reached 370,703,240 bytes with `__TEXT` at
146,046,976 bytes, `__TEXT,__text` at 99,671,028 bytes,
`__TEXT,__const` at 11,418,112 bytes, and `__LINKEDIT` at 220,266,496 bytes.
Targeted scans no longer find the prediction endpoint paths, `/ai/relevant_files`,
`/ai/transcribe`, `ServerVoiceTranscriber`, or voice quota strings in OSS.
The OSS dep-info points at the prediction, relevant-files, and voice shim
modules; the default-feature build still compiles the real implementations.
After replacing the multi-agent event stream, multi-agent output server request,
run-agents executor, start-agent executor, and response-stream controller with
OSS unavailable/no-op shims, the debug `warp-oss` executable is 369,821,784
bytes with `__TEXT` at 145,752,064 bytes, `__TEXT,__text` at 99,461,204 bytes,
`__TEXT,__const` at 11,414,016 bytes, and `__LINKEDIT` at 219,693,056 bytes.
Targeted scans no longer find `/api/v1/agent/events`, multi-agent request retry
strings, child-agent harness validation strings, or run-agents executor strings
in OSS. Remaining multi-agent protocol strings come from shared
`warp_multi_agent_api` request/response types still referenced by persistence,
conversation history, shared-session replay, and AI block compatibility models.

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
- OSS treats third-party agent SDK harnesses as unsupported and does not
  compile the Claude, Codex, or Gemini runner/transcript modules.
- OSS multi-agent event streams, multi-agent output requests, run-agents
  spawning, start-agent spawning, and response-stream retry handling use
  unavailable/no-op compatibility shims.
- OSS voice input uses a disabled `VoiceTranscriber` and no longer compiles the
  server transcriber or `/ai/transcribe` API path.
- OSS does not compile CLI-agent plugin manager implementations or their
  install/update instruction bodies.
- OSS does not compile the Codex promo modal or register its root/URI actions.
- OSS does not register the OpenCode plugin debug workspace actions or compile
  the helper that mutates `~/.config/opencode/opencode.json`.
- OSS local third-party child harness launches return an explicit unavailable
  error and do not compile the concrete Claude, Codex, or OpenCode command
  builders and setup paths.
- OSS excludes third-party CLI-agent logo SVG payloads from the embedded asset
  set.
- OSS excludes AI-specific agent/prompt/conversation/context SVG payloads from
  the embedded asset set.
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
- `warp_files` repository-level file watching is behind the non-OSS
  `repository_file_watching` app feature and the
  `warp_files/repository_watching` crate feature. OSS builds fall back to
  individual file watchers and no longer pull `repo_metadata` through
  `warp_files`.
- AI crate local-filesystem support is behind the non-OSS `ai_local_fs` app
  feature. OSS builds no longer propagate the app/platform `local_fs` feature
  into `ai`, disabling native AI codebase-indexing/project-context filesystem
  paths while leaving terminal-local filesystem support enabled.
- The AI crate no longer forces `cfg(feature = "local_fs")` from its build
  script on non-wasm targets. Its native code-indexing dependencies
  (`arborium`, `languages`, `syntax_tree`, `watcher`, `notify-debouncer-full`,
  and `async-fs`) are now optional under `ai/local_fs`; OSS builds use no-op
  app/AI compatibility paths for repo outlines and index snapshot maintenance.
- AI full-source-code embedding is now compiled only with `ai/local_fs`.
  OSS builds keep the public manager/status/store-client/hash/fragment API as
  an unavailable compatibility shim, and the AI crate's `repo_metadata`
  dependency is also behind `ai/local_fs`. This removes the previous
  `ai -> repo_metadata` normal dependency edge from OSS. This did not move the
  release app-size checkpoint because the linked OSS binary was already
  dominated by remaining app AI/settings/server strings and code.
- The app AI server API implementation now uses local `AgentTaskState`,
  `PlatformErrorCode`, and `ScheduledAgentHistory` boundary types, with the real
  GraphQL `AIClient` implementation compiled only for non-OSS builds. OSS keeps
  an unavailable/default `AIClient` implementation and no longer links the
  GraphQL AI task, conversation, model-choice query, artifact upload, and
  code-indexing request paths through `server_api::ai`. This moved the debug
  checkpoint from 468M to 458M, with `__TEXT` at 184,320,000 bytes,
  `__TEXT,__text` at 126,904,284 bytes, and `__LINKEDIT` at 290,914,304 bytes,
  and moved the release checkpoint from 102M app / 100M executable to
  100M app / 99M executable.
- The app auth server API implementation now compiles the real Firebase,
  custom-token, user-settings, conversation-usage, onboarding, device-code, API
  key, and ambient workload-token request paths only for non-OSS builds. OSS
  keeps unavailable/default `AuthClient` methods so auth callers compile without
  linking the real request machinery. This moved the debug checkpoint from 458M
  to 454M, with `__TEXT` at 183,009,280 bytes, `__TEXT,__text` at 125,951,500
  bytes, and `__LINKEDIT` at 288,227,328 bytes.
- The app team/workspace server API implementations now compile the real team
  management, workspace metadata polling, billing portal, usage-based pricing,
  AI overage, add-on credit, and workspace settings GraphQL request paths only
  for non-OSS builds. OSS keeps empty workspace/team metadata and unavailable
  billing responses so workspace metadata polling/settings callers compile
  without linking those server flows. This moved the debug checkpoint from 454M
  to 444M, with `__TEXT` at 179,683,328 bytes, `__TEXT,__text` at 123,566,296
  bytes, and `__LINKEDIT` at 280,821,760 bytes, and moved the release
  checkpoint from 100M app / 99M executable to 98M app / 97M executable.
- The app referral, shared-block, and integrations server API implementations
  now compile the real referral invite/info, shared-block upload/list/title,
  GitHub auth-status, simple integration, OAuth polling, environment integration,
  GitHub repo info, and cloud-environment image suggestion request paths only for
  non-OSS builds. OSS keeps empty/default results or explicit unavailable
  errors so settings and terminal share callers compile without linking those
  flows. This moved the debug checkpoint from 444M to 440M, with `__TEXT` at
  178,438,144 bytes, `__TEXT,__text` at 122,652,376 bytes, and `__LINKEDIT` at
  278,315,008 bytes.
- The app managed-secrets server API adapter now compiles the real managed
  secret config/list/create/update/delete, task-secret, and task identity-token
  GraphQL request paths only for non-OSS builds. OSS keeps empty/default results
  or explicit unavailable errors. This is a cleanup cut on top of the earlier
  managed-secrets crate shim and did not move the debug checkpoint.
- The app cloud-object server API implementation now compiles the real workflow,
  notebook, folder, generic-string-object, Warp Drive subscription, changed-object
  fetch, single-object fetch, owner transfer, trash/delete/move, action-history,
  sharing-permission, guest, and cloud-environment timestamp GraphQL paths only
  for non-OSS builds. OSS keeps empty changed-object responses, no-op streaming
  readiness, explicit unavailable errors for cloud writes/fetches, and simple
  success/failure defaults for delete/permission shapes that callers already
  handle. This moved the debug checkpoint from 440M to 427M, with `__TEXT` at
  173,703,168 bytes, `__TEXT,__text` at 119,210,384 bytes, and `__LINKEDIT` at
  268,812,288 bytes, and moved the release checkpoint from 98M app / 97M
  executable to 95M app / 93M executable.
- The app GraphQL request helper, request options, and real GraphQL client error
  reexports are now compiled only for non-OSS builds. OSS keeps a tiny local
  `GraphQLError` compatibility type for sync-queue retry classification and
  unavailable/default helper stubs for imports that remain in cfg-gated server
  modules. The only live OSS `send_graphql_request` callers were the
  cloud-environment image catalog CLI paths; those now use OSS-local unavailable
  behavior or a custom-image prompt. This moved the debug checkpoint from 427M
  to 425M, with `__TEXT` at 173,228,032 bytes, `__TEXT,__text` at 118,871,900
  bytes, and `__LINKEDIT` at 267,714,560 bytes.
- The Billing and Usage and Platform/API-key settings pages are now replaced
  with OSS-only no-op page shims. The settings shell still compiles, modal
  lookups return no content, and the pages do not render in OSS, so the real
  billing widgets, usage-history UI, add-on credit controls, API-key modal, and
  expire-button code are no longer linked into OSS. This moved the debug
  checkpoint from 425M to 422M, with `__TEXT` at 172,113,920 bytes,
  `__TEXT,__text` at 118,057,044 bytes, and `__LINKEDIT` at 265,584,640 bytes,
  and moved the release checkpoint from 95M app / 93M executable to 94M app /
  92M executable.
- The terminal buy-credits banner and enable-auto-reload modal now use OSS-only
  no-op view shims. The event/action surfaces remain for terminal and workspace
  callers, but the real add-on credit purchase, pricing dropdown, auto-reload
  update, and budget-exceeded UI code no longer links into OSS. This moved the
  debug checkpoint from 422M to 421M, with `__TEXT` at 171,671,552 bytes,
  `__TEXT,__text` at 117,733,204 bytes, and `__LINKEDIT` at 264,716,288 bytes.
- The workspace build-plan migration modal, free-tier limit modal, and cloud
  agent capacity modal now use OSS-only no-op view shims. The workspace shell
  still owns the same modal handles/events, but the real billing upgrade,
  plan-pricing, add-on credit, and cloud-agent capacity upsell views no longer
  link into OSS. This moved the debug checkpoint from 421M to 420M, with
  `__TEXT` at 171,311,104 bytes, `__TEXT,__text` at 117,472,596 bytes, and
  `__LINKEDIT` at 264,044,544 bytes.
- The Teams settings page now uses an OSS-only no-op page shim, while preserving
  exported invite option/URI argument types used by telemetry and deeplink
  parsing. Team management, invite, billing, transfer-ownership, admin action,
  editor, and shared-object confirmation UI for that page no longer links into
  OSS. Root onboarding and AI request-usage pricing reads now return
  `None`/`Hidden` in OSS, so the remaining active price-badge/buy-credit checks
  do not keep extra pricing UI behavior alive. This moved the debug checkpoint
  from 420M to 418M, with `__TEXT` at 170,590,208 bytes, `__TEXT,__text` at
  116,939,348 bytes, and `__LINKEDIT` at 262,782,976 bytes, and moved the
  release checkpoint from 94M app / 92M executable to 93M app / 92M executable.
- The shared-object limit modal now uses an OSS no-op shim, and
  `workspaces::gql_convert` is compiled only for non-OSS builds. Common
  workspace metadata keeps small OSS-local compatibility types for pricing,
  feature-model choices, add-on credit options, and service agreements so the
  OSS workspace/update-manager paths no longer compile the GraphQL workspace
  conversion body. This reduced the debug checkpoint to `__TEXT` 170,491,904
  bytes, `__TEXT,__text` 116,874,324 bytes, and `__LINKEDIT` 262,619,136 bytes;
  no release bundle was run for this small delta.
- Auth client trait results for anonymous user creation, custom-token minting,
  API-key owner type, API-key result aliases, and usage-history/API-key helper
  aliases now use app-local boundary types in OSS. GraphQL conversion is
  confined to the non-OSS server API implementation. This removes several
  active GraphQL auth type leaks from OSS call sites, but the measured debug
  app size stayed effectively flat at the current 418M checkpoint, so no
  release bundle was run.
- `ServerTimestamp` imports now go through `server::timestamp` as a central
  app boundary, backed by a `warp_server_client` re-export of the existing
  cloud-object timestamp and access-level types. The OSS-local timestamp wrapper
  attempt was deferred because `warp_server_client` cloud-object metadata still
  exposes the GraphQL timestamp type, so replacing it safely requires a broader
  client-boundary split.
- A probe making the app's direct `warp_graphql` dependency optional behind a
  non-OSS-only default feature showed the remaining true OSS blockers. The
  app still had active direct GraphQL imports in AI SDK integration/config/secret
  modules, AI artifacts/MCP gallery, request-usage grant types, cloud-object
  update inputs, block sharing display settings, managed-secrets server APIs,
  integration server APIs, object server APIs, and server experiment conversion.
  The dependency was restored to keep the build green; fixed blockers are being
  removed from that list before retrying the direct `warp -> warp_graphql`
  removal.
- Two small blocker surfaces are now isolated to non-OSS conversion code:
  `AIRequestUsageModel` owns a local `BonusGrantType` instead of re-exporting
  `warp_graphql::billing::BonusGrantType`, and server block sharing only maps
  local `DisplaySetting` into the GraphQL mutation enum in non-OSS builds.
  Server experiment string parsing stays local, while conversion from the
  GraphQL `Experiment` enum is now non-OSS only.
- Cloud-object initial-load version metadata now uses a local
  `UpdatedObjectVersions` struct in the common `CloudObject` trait and
  `ObjectsToUpdate` container. The GraphQL `UpdatedObjectInput` is constructed
  only in the non-OSS object server API query path.
- Managed-secrets app-facing types are now local in the OSS shim. The default
  build re-exports GraphQL aliases through the `warp_managed_secrets` wrapper,
  while the OSS shim owns local `ManagedSecret`, config, owner, type, and task
  secret value types. GraphQL conversion is confined to the non-OSS managed
  secrets server API implementation, so the OSS shim no longer imports
  `warp_graphql` directly.
- MCP gallery updates now cross the object API boundary as local app template
  structs. The GraphQL `MCPGalleryTemplate` query shape is converted in the
  non-OSS object server API response mapper, so `UpdateManager` and
  `MCPGalleryManager` no longer expose GraphQL gallery types.
- The app's direct `warp_graphql` dependency is now optional and enabled only
  by the default `ai_graphql_runtime` feature. OSS builds no longer pull
  `warp_graphql` through `warp` directly; remaining GraphQL access is through
  `warp_server_client`. The last direct blockers were moved behind local
  integration result types, non-OSS-only artifact/AI-assist conversions, and a
  local generic-string-object format string mapper for blocklist input context.
- `warp_server_client` now gates its `warp_graphql` dependency behind a default
  `graphql` feature. The workspace dependency disables that feature by default,
  while the app's default `ai_graphql_runtime` feature opts back into
  `warp_server_client/graphql` for non-OSS builds. OSS builds use local
  `ServerTimestamp` and `AccessLevel` boundary types with GraphQL conversions
  compiled only when the client feature is enabled. This removes the remaining
  indirect `warp_server_client -> warp_graphql` edge from the normal OSS graph.
- The generated telemetry event-description catalog, feature-specific telemetry
  event descriptions, remaining app-local telemetry descriptions, and the AI,
  onboarding, and repo-metadata crate telemetry descriptions are now disabled in
  OSS. The OSS channel has
  `telemetry_config: None`, so OSS telemetry names/descriptions now collapse to
  generic disabled-telemetry strings while payload construction stays intact for
  internal queues. This drops the internal analytics names/descriptions for
  removed AI/auth/MCP/billing/code surfaces from the OSS binary without changing
  non-OSS telemetry.
- Settings schema inventory submissions are now behind the explicit
  `settings_schema_registry` feature. The normal OSS app feature set omits that
  feature so the application binary does not link schema-generation inventory
  metadata, while bundled-resource preparation passes the feature when running
  `generate_settings_schema` so `settings_schema.json` is still generated.
- The OSS privacy settings page no longer links the cloud-conversation storage
  widget or the AI analytics/free-tier copy, while non-OSS builds keep the
  existing enterprise/free-tier behavior. The OSS login-slide stub no longer
  carries the unused cloud-conversation action variant.
- The terminal inline AI conversation menu now uses an OSS no-op view. The
  non-OSS menu continues to own the real conversation data source, search item,
  tab handling, and navigation event, while OSS keeps only the minimal view API
  required by shared terminal-input code.
- The command-palette conversation data source now uses an OSS empty source.
  Non-OSS builds keep the real fuzzy conversation searcher, command-palette
  result rendering, section labels, and recent-item lookup.
- The left-panel conversation list now uses an OSS no-op panel. Non-OSS builds
  keep the real list view, item rendering, model subscriptions, overflow/share
  actions, and keyboard bindings.
- The OSS slash-command registry now includes only terminal-safe commands, and
  non-OSS slash-command execution arms are gated out of OSS. This removes the
  registry and direct handler roots for AI/MCP/code-review/editor/git/billing
  slash commands while preserving the full command set for non-OSS builds.
- The OSS slash-command command table now lives in an OSS-specific shim that
  exposes compatibility placeholders for deeper AI code while retaining only
  feedback, tab rename/color, changelog, and settings-file metadata. The
  default slash-command binding table also returns no bindings in OSS, dropping
  the old AI/cloud-agent/conversation command binding labels.
- Terminal and agent message bars now compile their AI slash-command helper
  copy only for non-OSS. OSS keeps inline-history messaging but drops `/agent`,
  `/plan`, conversation-history, fork/continue, and autodetected-prompt status
  bar producers.
- OSS no longer links direct fork/init command insertion references, AI block
  slash-prefix highlighting, or the AI slash-command controller parser/sender.
  These paths still compile for non-OSS but no-op in OSS while the surrounding AI
  modules are still being removed.
- The agent-view zero-state block now uses an OSS no-op view. The terminal
  zero-state block keeps its terminal session UI but drops AI conversation,
  cloud-agent, code-review, and NLD rows in OSS. The agent input footer render
  path also returns empty in OSS while preserving the non-OSS toolbar.
- The use-agent toolbar constructor and Warpify footer now preserve Warpify
  rendering in OSS without constructing agent buttons. The agent input footer is
  now an OSS compatibility shim, so the real footer controls and
  model/plugin/handoff selector setup are not linked into OSS.
- Ambient-agent host, harness, and model selectors now use OSS compatibility
  shims, removing the real selector dropdowns and model-search editor from OSS.
- Cloud-mode loading tips return an empty list in OSS, so Oz/cloud-agent docs
  URLs and tip copy are no longer retained while the loading-screen API remains
  available to surrounding compatibility code.
- Cloud-agent/create-environment URI actions and root action registrations are
  gated out of OSS. The non-OSS deeplink behavior remains intact, while OSS no
  longer accepts or dispatches those agent/environment actions.
- `TerminalAction` and `InputAction` use generic `Debug` output in OSS, and
  AI-only input keybinding registrations are skipped in OSS. Non-OSS builds keep
  their detailed action names and keybindings.
- `warp_cli` has an `oss_release` feature wired from the app so OSS builds use
  neutral top-level Warp CLI help instead of embedding the Oz/cloud-agent
  top-level description and examples.
- OSS builds no longer include the app `LaunchMode::CommandLine` root or the
  flattened `CommandLine` clap variant, so the full Oz CLI command graph and
  local agent SDK execution path are not rooted by app startup/completion code.
- Harness-support server API endpoints now compile to OSS unavailable stubs,
  while shared task-scoped public API helpers remain available for agent
  messaging. This removes the real external-conversation, transcript upload,
  artifact reporting, notification, finish-task, and snapshot endpoint strings
  from OSS.
- Task-scoped agent message server methods now compile to OSS unavailable
  stubs, so OSS no longer links the real `agent/messages` request paths or the
  task-scoped authenticated HTTP helper implementation.
- `warp_features` now has an `oss_release` mode wired through `warp_core`, so
  OSS keeps feature-flag values and checks but collapses feature-flag debug
  formatting and Preview changelog descriptions. This drops retained
  AI/Oz/MCP/code-review feature names/descriptions from the OSS binary.
- The OSS settings shell now uses the reduced terminal-safe sidebar and
  collapses removed settings-section display/parse/debug names, so no-op
  settings page shims no longer retain Agents/MCP/Code Review/Oz Cloud sidebar
  labels in the OSS binary.
- Server experiment metadata now has OSS-specific debug/display/parse behavior:
  terminal/session/SSH experiment IDs remain supported, while removed
  AI/Oz/code experiment arms collapse to a generic disabled experiment label.
- `LLMPreferences` now keeps its compatibility model surface in OSS but skips
  authenticated and public server refreshes, removing the remaining direct model
  fetch roots and their error strings from the OSS binary.
- Cached LLM model serde is now non-OSS-only, with OSS always using default
  model metadata. This keeps the shared `LLMPreferences` API compiling while
  removing cached-model field metadata and cache error strings from OSS.
- The AI, Code, and MCP settings pages are now replaced with OSS-only no-op
  page shims. The settings shell still compiles and direct navigation requests
  safely no-op, but the real page modules and their large AI/MCP/code settings
  strings are no longer linked into OSS. This moved the release checkpoint from
  110M app / 109M executable to 109M app / 107M executable.
- Cloud-environment settings pages/forms and first-time cloud-agent setup are
  now replaced with OSS-only no-op shims. Direct navigation and call sites still
  compile, but the real environment setup views are no longer linked into OSS.
  This moved the release checkpoint from 109M app / 107M executable to
  108M app / 107M executable.
- Workspace command-palette bindings and macOS menu roots for AI, MCP, code
  review, code settings, cloud-environment settings, agent tabs, agent
  conversation lists, and AI prompt creation are now gated out of OSS. The
  remaining call sites still compile through existing enum/action surfaces, but
  their user-visible labels and action payloads are no longer linked into the
  debug `warp-oss` binary.
- Terminal cloud-environment init/setup UI now uses an OSS-only no-op module.
  The public `InitEnvironmentBlock` and selector types remain for compatibility,
  but the real prompt/modal rendering and AI blocklist UI imports are no longer
  compiled for OSS.
- Terminal `/init` project setup now uses an OSS-only no-op module. The public
  model, block, result, and event types remain for compatibility with terminal
  and code-review call sites, but the real AI project rules, codebase indexing,
  LSP setup, and cloud-environment setup blocks are no longer compiled for OSS.
- The Agent Management Oz cloud setup guide now uses an OSS-only no-op child
  view. The real guide text, workflow code-block rendering, Oz links, and setup
  commands are no longer linked into the debug `warp-oss` binary.
- The Agent Management view now uses an OSS-only no-op view while preserving
  the persistence/event/type surface expected by workspace state restoration and
  navigation actions. The real task list, filters, details panel, search editor,
  artifact buttons, and agent type selector are no longer compiled for OSS.
- The Agent Management notification model, notification item collection,
  mailbox view, and toast stack now use OSS-only no-op shims. Workspace unread
  indicators and navigation handlers still compile against the same surfaces,
  but the real notification subscriptions, rendering, artifact buttons, and
  toast timers are no longer compiled for OSS.
- The legacy agent toast component now uses an OSS-only no-op shim. The
  workspace's jump-to-latest-toast/navigation call sites still compile, but the
  old toast rendering, hover handling, timers, and keybinding subscription are
  no longer compiled for OSS.
- OSS startup no longer registers auth UI modals, Oz/cloud launch/capacity
  modals, Codex/free-tier modal bindings, header-toolbar editor bindings, agent
  conversation-list bindings, notebook actions, code view/file-tree actions, or
  LSP actions. The larger modules still compile where workspace type surfaces
  require them, but their keybinding/action registration roots are no longer on
  the OSS startup path.
- The new-worktree modal now uses an OSS-only no-op shim. Workspace modal state
  and action handlers still compile, but the real git repo picker, branch
  picker, editor field, validation, and modal rendering are no longer compiled
  for OSS.
- The auth override warning modal now uses an OSS-only no-op shim. Root and
  workspace auth-interruption handlers still compile, but the real destructive
  login override warning body, modal wrapper, bindings, and nested auth-manager
  subscription are no longer compiled for OSS.
- The auth login modal, login/onboarding slide, and paste-token modal now use
  OSS-only no-op shims. The redirect payload parser and auth view variants stay
  available for existing auth-manager and root/workspace call sites, but the
  real login modal body, onboarding login slide, token editor, privacy toggles,
  failure notification rendering, and auth UI keybindings are no longer compiled
  for OSS.
- The remaining auth UI helper modules now have OSS-only no-op/type shims. The
  login failure enum and SSO-link view surface remain for root/auth call sites,
  but the real login failure notification, login error modal, SSO-link modal
  body, and shared auth-view rendering helpers are no longer compiled for OSS.
- `AuthManager` now uses an OSS-only no-op compatibility shim. The real
  redirect handling, Firebase refresh/exchange, anonymous-user creation/linking,
  device-code auth, auth URL construction, login telemetry, user-persistence
  side effects, and auth-manager event emission paths remain available only in
  non-OSS builds.
- `AuthState` now uses an OSS-only logged-out compatibility shim. The real
  persisted-user loading, API-key authentication setup, Firebase credential
  storage, user-profile accessors, anonymous-limit calculations, reauth mutation
  state, and secure-storage persistence decisions remain available only in
  non-OSS builds.
- OSS credential/user modules now use compatibility shims. The real
  Firebase/API-key token helper methods, Firebase endpoint URL builders, user
  metadata helpers, principal/anonymous-user conversion metadata, and personal
  object-limit type metadata remain available only in non-OSS builds.
- The local Firebase response module now uses an OSS compatibility shim. The
  real account-info parser, provider-profile lookup, access-token response
  payload names, and Firebase-specific display text remain available only in
  non-OSS builds.
- OSS `UserAuthenticationError` keeps its compatibility variants but uses
  generic display/actionability behavior and does not compile Firebase
  access-token classifier constants or auth-redirect state error descriptions.
- The Oz launch slide deck now uses an OSS-only no-op slide implementation.
  Workspace's modal holder still compiles, but the real Oz launch copy, images,
  cloud-agent CTA, cloud-conversation checkbox, and telemetry path are no longer
  compiled for OSS.
- The OpenWarp launch modal now uses an OSS-only no-op view. Workspace's modal
  holder still compiles, but the real open-source launch copy, async hero image,
  Oz platform link, repo CTA, and custom modal rendering are no longer compiled
  for OSS.
- `OneTimeModalModel` now uses an OSS-only no-op singleton. Workspace visibility
  checks and debug actions still compile, but the real auth/settings/cloud
  preference subscriptions and automatic Oz/OpenWarp/HOA/build-plan modal
  trigger logic are no longer compiled for OSS.
- The legacy Warp AI panel, transcript view, request model, and rendering
  utilities now use OSS-only compatibility shims. Workspace open/close and
  `AskAIType` call sites still compile, but the real editor-backed panel,
  transcript rendering, prompt/request orchestration, and zero-state UI are no
  longer compiled for OSS.
- The command-search Warp AI data source is no longer compiled or registered in
  OSS. Natural-language command search still keeps the existing command-search
  action enum surface, but the Warp AI sync result, generated-workflow async
  source, icon rendering, and command-search result copy are no longer linked
  into OSS.
- Legacy Warp AI terminal context-menu items, input context-menu actions,
  keybindings, block toolbelt fallback button, workspace tab-bar entrypoint,
  warm-welcome card, and Resource Center AI command-search tip are now gated out
  of OSS. Agent Mode attach-context entrypoints remain available, but the
  non-agent-mode Warp AI labels and menu actions are no longer linked into the
  debug `warp-oss` binary.
- The Project Explorer file tree now uses an OSS-only no-op view while
  preserving the action, event, identifier, and compatibility methods needed by
  workspace and left-panel call sites. The real file tree rendering, filesystem
  tree state, drag/drop handling, and repository/file-watcher UI paths are no
  longer compiled for OSS. This moved the debug checkpoint from 490M to 489M,
  with `__TEXT` at 195,936,256 bytes and `__TEXT,__text` at 135,421,248 bytes.
- The main CodeView now uses an OSS-only compatibility view while preserving
  pane restoration, tab metadata, file-opening method surfaces, pane actions,
  and event variants expected by workspace/pane call sites. The real tabbed
  code editor view, preview handling, save/render-markdown UI, and active editor
  orchestration are no longer compiled through the main CodeView path. This
  moved the debug checkpoint from 489M to 488M, with `__TEXT` at 195,706,880
  bytes and `__TEXT,__text` at 135,256,232 bytes.
- The Code Review panel now uses OSS-only compatibility shims for its main
  view, header, and imported-comment rendering surface while preserving the
  action/event/debug-state methods expected by right-panel, workspace, pane
  group, terminal, and AI call sites. The real Code Review diff viewport,
  comment list, find model, diff selector/menu, git operation dialogs, file
  invalidation queue, and editor-backed comment rendering are no longer
  compiled for OSS. This moved the debug checkpoint from 488M to 481M, with
  `__TEXT` at 192,610,304 bytes and `__TEXT,__text` at 132,979,596 bytes.
- The Agent Mode code-diff view now uses an OSS-only compatibility view while
  preserving the action/event, diff state, file-diff data, display-mode, and
  accept/reject method surfaces expected by AI block, terminal passive
  suggestions, workspace panes, and request-file-edit execution. The real
  inline diff editors, tabbed diff UI, code-suggestion speedbump, skill/MCP
  buttons, and editor subscription plumbing are no longer compiled for OSS.
  This moved the debug checkpoint from 481M to 479M, with `__TEXT` at
  192,004,096 bytes and `__TEXT,__text` at 132,529,948 bytes.
- The AI document pane, AI facts pane, and execution-profile editor pane now
  use OSS-only compatibility views while preserving their pane restoration,
  focus, action/event, document/profile ID, and manager registration surfaces.
  The real rich-text document editor, rules/rule-editor UI, execution-profile
  dropdowns/editors, and associated editor subscriptions are no longer compiled
  through these pane paths in OSS. This moved the debug checkpoint from 479M to
  475M, with `__TEXT` at 190,562,304 bytes and `__TEXT,__text` at
  131,472,340 bytes.
- The suggested-rule modal now uses an OSS-only compatibility view while
  preserving the workspace modal holder and suggested-rule ID payload type. The
  real modal wrapper, name/content editors, update-manager subscriptions, and
  add/edit button UI are no longer compiled through this path in OSS. This kept
  the rounded debug checkpoint at 475M, with `__TEXT` reduced to 190,464,000
  bytes and `__TEXT,__text` reduced to 131,406,064 bytes.
- The codebase-index speedbump banner now uses an OSS-only compatibility state
  and empty render path while preserving the terminal action/state fields still
  mutated by terminal handlers. The real banner copy, checkbox/buttons, icon
  layout, and settings/action callbacks are no longer compiled through this path
  in OSS. This kept the rounded debug checkpoint at 475M, with `__TEXT` reduced
  to 190,447,616 bytes and `__TEXT,__text` reduced to 131,395,052 bytes.
- The AI telemetry banner now uses an OSS-only empty view while preserving the
  terminal rich-content metadata type and the `should_collect_ai_ugc_telemetry`
  predicate used by settings, terminal, and telemetry call sites. The real
  banner copy, privacy CTA, buttons, and icon layout are no longer compiled
  through this path in OSS. This kept the rounded debug checkpoint at 475M,
  with `__TEXT,__text` reduced to 131,388,868 bytes.
- The AI context menu now uses an OSS-only compatibility module while
  preserving the editor/terminal action, event, category, and helper-function
  surfaces. The real menu view, search bar, async data sources, file/code/rule/
  workflow/notebook/conversation/skill data source modules, repo subscriptions,
  and result rendering are no longer compiled through this path in OSS. This
  moved the debug checkpoint from 475M to 472M, with `__TEXT` at 189,431,808
  bytes and `__TEXT,__text` at 130,619,736 bytes.
- The branch/repo picker views used by tab-config parameter modals now use
  OSS-only local dropdown shims. Custom tab configs still compile through the
  same picker API, but the real branch fetch path no longer imports
  `DiffStateModel`/git branch helpers, and the repo picker no longer subscribes
  to `PersistedWorkspace` or renders the add-repo footer in OSS.
- The guided session-config modal and HOA onboarding flow now use OSS-only
  shims. The session-config modal still creates plain tab configs, but it no
  longer renders or toggles worktree creation. The HOA flow is treated as
  completed in OSS and no longer compiles the welcome/banner/callout worktree
  UI. This reduced debug `__TEXT` to 189,235,200 bytes and `__TEXT,__text` to
  130,480,416 bytes.
- Workspace worktree creation entrypoints are now gated out of OSS. The new
  session menu no longer includes the "New worktree config" submenu, and the
  generated-worktree TOML/default-template materialization paths are compiled
  only for non-OSS local-filesystem builds. This moved the rounded debug
  checkpoint from 472M to 471M, with `__TEXT` at 189,087,744 bytes and
  `__TEXT,__text` at 130,361,620 bytes.
- `session_config::build_tab_config` now treats worktree generation as a
  non-OSS branch. The compatibility API remains available, but OSS always
  emits a plain tab config and no longer links the worktree command template
  strings from this helper. The rounded debug checkpoint stayed 471M, with
  `__TEXT` at 189,071,360 bytes and `__TEXT,__text` at 130,359,316 bytes.
- The app's direct `repo_metadata` dependency is now behind the non-OSS
  `repo_metadata_runtime` feature. OSS builds use a crate-root compatibility
  shim for detected-repository lookup, repository identifiers, repository
  watcher/subscriber types, file-tree metadata types, and the unified
  `RepoMetadataModel` surface still referenced by terminal/search/code/AI
  modules. This removes the real `repo_metadata` package from the normal OSS
  dependency graph while preserving simple `.git` root lookup for terminal
  basics. The rounded debug checkpoint stayed 471M, with `__TEXT` at
  188,891,136 bytes and `__TEXT,__text` at 130,232,996 bytes.
  The release checkpoint moved from 103M app / 101M executable to 102M app /
  100M executable, with `__TEXT` at 97,812,480 bytes and `__TEXT,__text` at
  74,411,104 bytes.
- The app's direct `onboarding` dependency is now behind the non-OSS
  `onboarding_runtime` feature. OSS builds use a crate-root compatibility shim
  for onboarding selections, model IDs, empty onboarding views, and terminal
  tutorial callouts, while OSS root/workspace paths treat local onboarding as
  complete and suppress tutorial dispatch. This removes the real `onboarding`
  package from the normal OSS dependency graph. The rounded debug checkpoint
  moved from 471M to 469M, with `__TEXT` at 188,137,472 bytes and
  `__TEXT,__text` at 129,683,888 bytes. The release checkpoint stayed 102M app
  / 100M executable, with `__TEXT` reduced to 97,615,872 bytes and
  `__TEXT,__text` reduced to 74,265,808 bytes.
- The external `ai` crate's `warp_graphql` dependency is now behind an
  `ai/graphql_runtime` feature. Normal app defaults still enable the real
  GraphQL conversions, but OSS no-default builds use a tiny local GraphQL-shaped
  type shim for disabled code-indexing conversion APIs. The app-side
  code-indexing GraphQL store calls now return unavailable/no-op behavior in OSS
  instead of compiling the real conversion path. This removes the
  `ai -> warp_graphql` edge from the OSS inverse tree; `warp_graphql` remains
  through the app and `warp_server_client`. This moved the rounded debug
  checkpoint from 469M to 468M, with `__TEXT` at 187,826,176 bytes and
  `__TEXT,__text` at 129,473,040 bytes.
- The app's direct `remote_server` dependency is behind the non-OSS
  `remote_server_runtime` feature. OSS builds keep a small local
  `crate::remote_server` compatibility shim for remote setup state,
  terminal-session bookkeeping, remote command/file request types, and the
  auth/SSH transport surfaces still referenced by terminal and AI modules.
  Remote-server CLI subcommands and runtime operations return unsupported/no-op
  behavior in OSS instead of pulling the real `remote_server` crate.
- The direct `firebase` auth helper crate is behind the non-OSS
  `firebase_auth` app feature. OSS builds use a local serialization-compatible
  shim for the Firebase response/error types still referenced by server auth
  code, avoiding the real package edge while auth modules remain in the tree.
- Server-sent event support in `http_client` and the app's direct
  `reqwest-eventsource` dependency are behind the non-OSS `http_eventsource`
  app feature and `http_client/eventsource` crate feature. OSS builds keep the
  stream API shape with an empty typed stream so remaining AI/server event
  callers compile without pulling `reqwest-eventsource`.
- Mermaid SVG rendering is behind the non-OSS `markdown_mermaid` app feature
  and `warp_editor/mermaid_rendering` crate feature. OSS builds keep the editor
  Mermaid-shaped block/layout types but classify Mermaid fences as ordinary
  code and no longer pull `mermaid_to_svg`.
- The `rmcp` package is now optional. Non-OSS builds enable the real package
  through `mcp_runtime` and `ai/mcp_model_types`; OSS builds compile remaining
  MCP-shaped action/result structs through a small AI-crate compatibility
  module and no longer pull `rmcp` or its built-in `oauth2` edge.
- Device-flow OAuth is now behind the non-OSS `auth_oauth` app feature and the
  `http_client/oauth_client` crate feature. OSS builds keep explicit
  unavailable errors for headless device auth and no longer pull `oauth2`.
- The `lsp` runtime crate is now behind the non-OSS `lsp_runtime` app feature.
  OSS builds keep a crate-root compatibility shim for LSP server types,
  manager/model state, request futures, and installation candidates. LSP
  detection, installation, hover, references, formatting, and diagnostics are
  no-op/unavailable in OSS, so the real `lsp` package is no longer in the graph
  while editor UI code is still being split out.
- The `lsp-types` package is now also behind the non-OSS `lsp_runtime` app
  feature. OSS builds keep a small crate-root compatibility shim for the LSP
  value types still used by editor/settings/persistence code, so the real
  protocol-types package is no longer in the graph.
- The `syntax_tree` package is now behind the non-OSS `syntax_tree_runtime`
  app feature. OSS builds keep a crate-root compatibility shim for editor
  syntax-tree state, color maps, decoration events, language indent units,
  bracket pairs, and comment prefixes. Tree-sitter parsing, syntax
  highlighting, and semantic indentation are no-op in OSS, so the real
  `syntax_tree` package is no longer in the graph.
- The `languages` package is now behind the non-OSS `language_runtime` app
  feature, which is enabled by `syntax_tree_runtime` for default builds. OSS
  builds keep a crate-root compatibility shim for language lookup and display
  names, returning no supported language. This removes the remaining
  `arborium` edge from the OSS graph.
- The `warp_files` package is now behind the non-OSS `file_runtime` app
  feature. OSS builds keep a crate-root compatibility shim for `FileModel`,
  file events, and text-file read result types. The shim supports simple local
  async reads/writes for compatibility but drops the real file watcher,
  repository watching, and remote-file backend, so `warp_files` is no longer in
  the normal OSS graph.
- The `repo_metadata` crate no longer forces `cfg(feature = "local_fs")` from
  its build script on non-wasm targets. Its native `async-fs`,
  `notify-debouncer-full`, and `watcher` dependencies are now optional under
  `repo_metadata/local_fs`; OSS builds keep no-op local metadata operations
  where editor/file-tree call sites still compile.
- The app's direct watcher runtime is behind the non-OSS `watcher_runtime`
  feature. OSS builds keep crate-root compatibility shims for
  `HomeDirectoryWatcher`, `BulkFilesystemWatcher`, watcher events, and the
  minimal `notify_debouncer_full::notify` filter/mode types. The real
  `watcher` and `notify-debouncer-full` packages are no longer in the normal
  OSS graph.
- OSS code-editor, local-code-editor, global-buffer-model, diff-viewer, and
  network-log pane modules now use no-op compatibility shims. The real code
  editor view/model stack, local editor wrapper, global buffer diffing model,
  LSP/find-reference roots, inline/diff viewer code, and network-log editor pane
  are no longer compiled into the OSS app.
- OSS MCP tool/resource action execution now uses no-op compatibility shims.
  The real MCP tool-call/resource-read executors, templatable MCP manager
  lookups, schema coercion path, and runtime result conversion stay compiled in
  the default build but are absent from OSS dep-info.
- OSS Warp Drive no longer compiles the MCP server row, MCP server collection
  row, AI fact row, or AI Rules collection row item implementations. Shared
  persisted object/action enums remain for compatibility, but the OSS Drive
  index does not render those AI/MCP-only collection rows.

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
  rg "notify-debouncer-full|\bwatcher|\bwarp_files|\blsp v|lsp-types|lsp feature|syntax_tree|languages|arborium|mcp_runtime|rmcp|oauth2|embed-signatures|embedded-signatures|tantivy|ownedbytes|bitpacking|levenshtein_automata|sketches-ddsketch|datasketches|syntect|onig|aws-config|aws-credential-types|aws-sdk-sts|aws-types|aws-smithy|aws-runtime|warp_managed_secrets|managed_secrets|tink-|hpke|computer_use|computer-use|computer_use_runtime|remote_server v|firebase v|reqwest-eventsource|mermaid_to_svg|input_classifier|natural_language_detection|voice_input|cpal|rubato|hound|nld_onnx|candle|tokenizers|command-signatures-v2|sentry|crash-handler|minidumper|git2|libgit2|repo_metadata|onboarding"
```

The inverse tree also reports that `remote_server` is not part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i remote_server
```

The inverse tree also reports that `repo_metadata` and `onboarding` are not
part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i repo_metadata

cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i onboarding
```

The inverse tree also reports that `firebase` is not part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i firebase
```

The inverse tree also reports that `reqwest-eventsource` is not part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i reqwest-eventsource
```

The inverse tree also reports that `mermaid_to_svg` is not part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i mermaid_to_svg
```

The inverse tree also reports that `rmcp` and `oauth2` are not part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i rmcp

cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i oauth2
```

The inverse tree also reports that `lsp` and `lsp-types` are not part of the
OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i lsp

cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i lsp-types
```

The inverse tree also reports that `syntax_tree` is not part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i syntax_tree
```

The inverse tree also reports that `languages` and `arborium` are not part of
the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i languages

cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i arborium
```

The normal tree no longer contains `warp_files`. The inverse tree still shows
`warp_files` only through app dev-dependencies, not through the normal OSS
build:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i warp_files
```

The normal tree no longer contains `watcher` or `notify-debouncer-full`. Their
inverse trees only show app dev-dependency paths through `warp_files`:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i watcher

cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i notify-debouncer-full
```

The inverse tree also reports that `warp_graphql` is not part of the OSS graph:

```sh
cargo tree -p warp --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui \
  -i warp_graphql
```

## Remaining Removal Targets

The OSS build still has these normal top-level dependencies. They remain the
main blockers to fully removing AI/auth/MCP/editor/git UI code from the binary:

```text
ai
warp_editor
warp_server_client
```

Current anchors:

- `app/Cargo.toml` keeps these dependencies unconditional.
- `app/src/lib.rs` still keeps `mod ai`, `mod auth`, `mod code`, `mod code_review`, and `pub mod editor` unconditional.
- The real `repo_metadata` and `onboarding` crates are gone from the OSS tree;
  remaining metadata/onboarding-shaped app code is local compatibility shim
  surface that should disappear with broader terminal/AI/auth/editor cleanup.
- `ai` is now only pulled directly by `warp`; the previous `onboarding -> ai`
  edge is gone because the real onboarding crate is no longer linked in OSS.
- The real `remote_server` crate is gone from the OSS tree; the remaining
  remote-server-shaped app code is a local shim that should disappear with the
  broader terminal/AI/auth cleanup.
- The real `firebase` crate is gone from the OSS tree; the remaining
  Firebase-shaped app code is a local response-type shim that should disappear
  with broader auth/server API removal.
- The real `reqwest-eventsource` crate is gone from the OSS tree; remaining
  server/AI streaming APIs now compile against the no-op `http_client`
  eventsource surface until those modules are removed.
- The real `mermaid_to_svg` crate is gone from the OSS tree; remaining editor
  and AI output paths compile against local no-op Mermaid classification and
  rendering shims until editor/notebook/AI modules are removed.
- The real `rmcp` crate is gone from the OSS tree; remaining MCP data/result
  paths compile against the AI-crate compatibility module until the broader AI
  and MCP UI modules are removed.
- The real `oauth2` crate is gone from the OSS tree; remaining headless
  device-auth call sites compile against the local unavailable-error path until
  auth/server API code is removed from OSS.
- The real `lsp` crate is gone from the OSS tree; remaining LSP-shaped editor,
  settings, persistence, and init-project call sites compile against the local
  unavailable/no-op shim until editor/code UI modules are removed from OSS.
- The real `lsp-types` crate is gone from the OSS tree; remaining LSP value-type
  call sites compile against the local protocol-type shim until editor/code UI
  modules are removed from OSS.
- The real `syntax_tree` crate is gone from the OSS tree; remaining editor
  syntax-tree state call sites compile against a no-op shim until editor/code
  UI modules are removed from OSS.
- The real `languages` crate is gone from the OSS tree; remaining language
  lookup call sites compile against a no-language-support shim until
  editor/code UI modules are removed from OSS.
- The real `warp_files` crate is gone from the normal OSS tree; remaining file
  model call sites compile against a local compatibility shim until
  editor/notebook/AI file workflows are removed from OSS.
- The real `watcher` and `notify-debouncer-full` crates are gone from the
  normal OSS tree; remaining watcher call sites compile against local no-op
  compatibility shims until AI/MCP/editor filesystem watchers are removed from
  OSS.
- OSS startup registrations for MCP managers, code/editor state, codebase indexing, and managed secrets are now gated behind `not(oss_release)`.
- The gated constructors are the next dependency-removal anchors because they
  are no longer required by the OSS startup path. Managed secrets has already
  moved from a real dependency to a shim; the remaining work is to remove the
  auth/server/agent SDK modules that require the shim.

Verified inverse-tree blockers:

- `warp_editor` is now reached directly from `warp`. The previous `ai`,
  `languages`, and `syntax_tree` edges into `warp_editor` are gone in OSS.
- `warp_files` is no longer reached in the normal OSS graph. It still appears
  through app dev-dependencies for tests.
- `repo_metadata` is no longer reached in the normal OSS graph. The previous
  indirect edges through `warp_files` and `ai`, and the previous direct app
  edge, are gone in OSS.
- `onboarding` is no longer reached in the normal OSS graph. The app keeps a
  local compatibility shim for onboarding settings and no-op onboarding views.
- `arborium` is gone from the OSS graph. The previous app `languages`,
  `syntax_tree`, and `ai/local_fs` edges are gone in OSS.
- `warp_graphql` is no longer reached in the OSS graph. The previous direct
  `warp`, indirect `ai`, and indirect `warp_server_client` edges are gone in
  OSS.
- The local `crate::warp_managed_secrets` shim is still referenced by auth,
  server API adapters, and app AI agent SDK code. Those call sites are now
  compile-time compatible with OSS, but should disappear with the broader
  auth/server/AI module removal.

Latest debug metadata checkpoint:

- Added an OSS `AISettings` shim and gated `app/src/settings/mod.rs` so the
  OSS app keeps the compatibility settings API while dropping public
  AI/Oz/MCP/BYOK setting TOML paths, descriptions, remote-session policy
  roots, and CLI-agent toolbar regex compilation. All AI enablement getters now
  return disabled in OSS; terminal `DefaultSessionMode::TabConfig` and
  `DefaultSessionMode::DockerSandbox` remain available as non-AI session modes.
- Extended `define_settings_group!` to accept per-setting `#[cfg]` attributes,
  then used OSS-private variants for the remaining AI-named public settings in
  `SessionSettings`, `InputSettings`, and external-editor layout preferences.
- Debug `target/aarch64-apple-darwin/debug/warp-oss` remains 374M with
  `__TEXT` at 153,288,704 bytes, `__TEXT,__text` at 104,872,580 bytes,
  `__TEXT,__const` at 11,489,664 bytes, and `__LINKEDIT` at 234,504,192 bytes.
  This is metadata cleanup, so no release bundle was run.
- Targeted artifact scan now returns no matches for
  `agents.warp_agent`, `agents.profiles`, `agents.third_party`,
  `cloud_platform.third_party_api_keys`, `agents.mcp_servers`, or
  `agents.voice` in the OSS debug binary.
- Gated the `CloudAgentSettings` registration/module and the
  `CloudAgentConfig` cloud-object/sync-queue model out of OSS. Generic
  `JsonObjectType::CloudAgentConfig` labels still remain in shared object-type
  metadata, but the app-local cloud-agent config model, setting, server object
  variant, update queue item, and typed update handling no longer compile into
  the OSS app.
- After the cloud-agent object cut, debug `warp-oss` remains 374M with
  `__TEXT` at 153,157,632 bytes, `__TEXT,__text` at 104,777,604 bytes,
  `__TEXT,__const` at 11,489,664 bytes, and `__LINKEDIT` at 234,192,896 bytes.
  This remains below the threshold for a release bundle checkpoint.
- Gated the real app `agent_sdk` module out of OSS. The OSS build no longer
  compiles the CLI/cloud agent command runner, harness environment preparation,
  artifact upload helper, or local-to-cloud handoff snapshot upload path. Docker
  sandbox creation remains available as a terminal session mode, but the
  cloud-agent environment preparation hook is non-OSS-only.
- After the `agent_sdk` cut, debug `warp-oss` is 370M with `__TEXT` at
  152,043,520 bytes, `__TEXT,__text` at 104,008,536 bytes, `__TEXT,__const` at
  11,469,184 bytes, and `__LINKEDIT` at 231,833,600 bytes.
- Replaced the real `app/src/ai/mcp` module with an OSS compatibility shim
  (`app/src/ai/mcp_oss.rs`). The shim preserves shared data shapes and no-op
  manager APIs needed by currently shared persistence, settings, telemetry, and
  action code, but it drops real MCP gallery, file watcher, templatable manager,
  OAuth, reconnecting peer, parser, and runtime code from the OSS app module
  graph.
- After the MCP shim cut, debug `warp-oss` remains 370M with `__TEXT` at
  152,010,752 bytes, `__TEXT,__text` at 103,976,776 bytes, `__TEXT,__const` at
  11,473,280 bytes, and `__LINKEDIT` at 231,702,528 bytes. Cargo dep-info for
  the OSS lib points at `src/ai/mcp_oss.rs`; the default-feature lib still
  points at the real `src/ai/mcp/*` files, as expected.
- Replaced the real app `agent::api` module with an OSS compatibility shim
  (`app/src/ai/agent/api_oss.rs`). The shim keeps the shared
  `ServerConversationToken`, request parameter, stream, history-restoration,
  and message-conversion type surface needed by the currently shared agent
  history/controller code, but it drops real multi-agent request construction,
  response conversion, conversation conversion, and server request execution
  code from the OSS app module graph.
- After the app AI API shim cut, debug `warp-oss` is 367M with `__TEXT` at
  151,027,712 bytes, `__TEXT,__text` at 103,287,080 bytes, `__TEXT,__const` at
  11,455,488 bytes, and `__LINKEDIT` at 229,343,232 bytes. Cargo dep-info for
  the OSS lib points at `src/ai/agent/api_oss.rs`; the default-feature lib
  still points at the real `src/ai/agent/api/{convert_to,convert_from,impl,convert_conversation}.rs`
  files, as expected. A string scan of the OSS debug binary found
  `app/src/ai/agent/api_oss.rs` and no real `app/src/ai/agent/api/*`
  conversion/implementation paths.
- Gated the private `crates/ai` action and action-result conversion modules out
  of OSS, keeping small OSS-only compatibility impls for file contexts,
  document/read-skill result wrapping, file-glob v2 fallback, and passive
  suggestion file-edit actions. The active OSS `ai` dep-info points at
  `src/agent/action/mod.rs` and `src/agent/action_result/mod.rs` but not their
  private `convert.rs` modules; the default-feature `ai` dep-info still points
  at both real conversion modules.
- After the `ai` conversion-module cut, debug `warp-oss` remains 367M with
  `__TEXT` at 151,027,712 bytes, `__TEXT,__text` at 103,286,824 bytes,
  `__TEXT,__const` at 11,455,488 bytes, and `__LINKEDIT` at 229,343,232 bytes.
  The OSS `libai-*.rlib` checkpoint dropped from 10M to 8.0M.
- Replaced the real `app/src/ai/predict/next_command_model.rs` implementation
  with an OSS compatibility shim (`app/src/ai/predict/next_command_model_oss.rs`).
  The shim disables AI next-command generation and preserves local
  history-based autosuggestion fallback behavior plus the shared state/type
  surface still referenced by terminal input code. OSS dep-info points at
  `next_command_model_oss.rs`; the default-feature dep-info still points at the
  real `next_command_model.rs`. The `generate_ai_input_suggestions` API modules
  still compile into OSS because the compatibility state types keep their
  request/response structs alive.
- After the next-command-model cut, debug `warp-oss` is 371,303,544 bytes with
  `__TEXT` at 146,259,968 bytes, `__TEXT,__text` at 99,824,484 bytes,
  `__TEXT,__const` at 11,418,112 bytes, and `__LINKEDIT` at 220,659,712 bytes.
  This moved the checkpoint down from 372,168,536 bytes and remains a
  debug-only checkpoint, so no release bundle was run.
- Replaced the real `app/src/ai/predict/generate_ai_input_suggestions.rs`
  helper/API module with an OSS compatibility shim
  (`app/src/ai/predict/generate_ai_input_suggestions_oss.rs`). The shim keeps
  the shared telemetry and terminal state request/response shapes but drops
  real context-message collection, history-context string merging, and request
  construction helpers from the OSS app module graph. OSS dep-info points at the
  shim and no longer lists the real API submodules; the default-feature dep-info
  still lists the real module and API files.
- After the generate-input-suggestions shim cut, debug `warp-oss` is
  371,300,504 bytes with `__TEXT` at 146,259,968 bytes, `__TEXT,__text` at
  99,824,484 bytes, `__TEXT,__const` at 11,418,112 bytes, and `__LINKEDIT` at
  220,659,712 bytes. This is a small type-surface cleanup, so no release bundle
  was run.
- Replaced the real `app/src/ai/predict/prompt_suggestions` helper with an OSS
  shim (`app/src/ai/predict/prompt_suggestions_oss.rs`). The shim preserves the
  keybinding constants and public helper signatures while returning disabled
  prompt-suggestion state, dropping the real AI block pending-code/unit-test
  inspection helper from the OSS module graph. OSS dep-info points at
  `prompt_suggestions_oss.rs`; default-feature dep-info still points at the
  real `prompt_suggestions/mod.rs`.
- After the prompt-suggestions helper cut, debug `warp-oss` is 371,297,512
  bytes with `__TEXT` at 146,259,968 bytes, `__TEXT,__text` at 99,822,948
  bytes, `__TEXT,__const` at 11,418,112 bytes, and `__LINKEDIT` at 220,659,712
  bytes. This is a small helper cleanup, so no release bundle was run.
- Replaced the real `app/src/ai/predict/generate_am_query_suggestions.rs` and
  `app/src/ai/predict/predict_am_queries.rs` endpoint modules with OSS
  compatibility shims. The shims preserve the request/response type surfaces
  used by shared server and terminal code while dropping the real API submodule
  trees from the OSS module graph. OSS dep-info points at
  `generate_am_query_suggestions_oss.rs` and `predict_am_queries_oss.rs`; the
  default-feature dep-info still points at the real endpoint modules and API
  files.
- After the AM-query endpoint shim cut, debug `warp-oss` is 371,294,600 bytes
  with `__TEXT` at 146,259,968 bytes, `__TEXT,__text` at 99,822,948 bytes,
  `__TEXT,__const` at 11,418,112 bytes, and `__LINKEDIT` at 220,659,712 bytes.
  This is a small API-surface cleanup, so no release bundle was run.
- Gated the OSS server API bodies for AI input suggestions, AM query
  suggestions, and AM query prediction. The OSS-compatible methods now return a
  disabled error instead of compiling the real authenticated network calls and
  endpoint URL construction. `TerminalInput::predict_am_query` is also an OSS
  no-op, so the debug binary no longer contains the
  `generate_input_suggestions`, `generate_am_query_suggestions`, or
  `predict_am_queries` endpoint strings.
- After the AI prediction endpoint-body cut, debug `warp-oss` is 371,188,872
  bytes with `__TEXT` at 146,210,816 bytes, `__TEXT,__text` at 99,790,692
  bytes, `__TEXT,__const` at 11,418,112 bytes, and `__LINKEDIT` at 220,594,176
  bytes. This remains a debug-only checkpoint, so no release bundle was run.
- Replaced the real `app/src/ai/get_relevant_files/controller.rs` with an OSS
  no-op controller shim and gated the OSS `ServerApi::get_relevant_files` body
  to return a disabled error instead of compiling the authenticated
  `/ai/relevant_files` request path. The shim preserves
  `GetRelevantFilesController`, `GetRelevantFilesControllerEvent`, and
  `GetRelevantFilesError` for shared terminal/action code, but drops real
  repo-outline search, full-source embedding retrieval subscription, server
  fallback, and result telemetry from the OSS module graph. OSS dep-info points
  at `controller_oss.rs`; default-feature dep-info still points at the real
  controller. A string scan no longer finds `/ai/relevant_files`, the real
  controller's `get_relevant_files failed` path, or its full-source embedding
  fallback strings.
- After the relevant-files controller cut, debug `warp-oss` is 370,827,784
  bytes with `__TEXT` at 146,096,128 bytes, `__TEXT,__text` at 99,712,772
  bytes, `__TEXT,__const` at 11,418,112 bytes, and `__LINKEDIT` at 220,348,416
  bytes. This remains a debug-only checkpoint, so no release bundle was run.
- Replaced the remaining OSS ambient-agent terminal model, loading screen, and
  view-impl modules with compatibility shims. The shims keep the terminal view
  and input method surface alive while disabling cloud-mode pane creation,
  ambient-agent dispatch/followup, GitHub auth UI, setup progress UI, billing
  footer copy, and conversation-details cloud task refresh paths. A targeted
  string scan no longer finds the ambient GitHub-auth screen copy, cloud setup
  progress step strings, or cloud-agent cancellation status copy.
- Replaced the full AI block status bar with an OSS no-op status bar. The shim
  preserves the constructor, stop event, summarization dialog handle, and typed
  actions needed by terminal input, but drops the real warping indicator, agent
  message bar wiring, child-agent card, cloud-mode setup messages, and AI
  exchange subscriptions from the OSS module graph.
- Replaced the terminal profile/model selector and inline model/profile selector
  modules with OSS no-op views and empty data sources. This removes the real
  model-spec sidecar, API-key/model filtering, reasoning-level UI, profile
  search items, and manage-profile selector menus from the OSS binary.
- Trimmed the OSS execution-profile editor action enum to the close/delete
  surface that its empty view actually handles, removing stale MCP,
  computer-use, web-search, model, and allowlist action variant strings from the
  OSS binary.
- After the ambient-agent/status-bar/profile-selector cuts, debug `warp-oss` is
  365,236,728 bytes with `__TEXT` at 144,113,664 bytes, `__TEXT,__text` at
  98,266,964 bytes, `__TEXT,__const` at 11,397,632 bytes, and `__LINKEDIT` at
  216,776,704 bytes. The same OSS lib check and the default-feature lib check
  both pass; the default-feature check still reports pre-existing dead-code
  warnings in `crates/ai`.
- Replaced the remaining ambient-agent setup block/footer and logged-out AI
  sign-up banner renderers with OSS no-op compatibility shims. The shims keep
  the rich-content block, setup command, footer, and banner state/action
  surfaces intact while dropping the cloud-agent startup footer, setup command
  rows, and login-for-AI banner copy from the OSS build.
- Gated OSS AI output rendering arms and helper functions for MCP resource/tool
  calls, computer-use requests/results, upload-artifact rows, orchestration
  start/run/send-message rows, and imported code-review comments. The shared AI
  block renderer still compiles, but those removed action UI paths no longer
  instantiate in OSS. Matching action-result `Display` impls in `crates/ai`
  now format as a generic unavailable result in OSS while preserving detailed
  output for default builds.
- After the ambient footer/block/banner and AI action-output cuts, debug
  `warp-oss` is 365,111,224 bytes with `__TEXT` at 144,048,128 bytes,
  `__TEXT,__text` at 98,211,412 bytes, `__TEXT,__const` at 11,397,632 bytes,
  and `__LINKEDIT` at 216,711,168 bytes. Targeted scans no longer find the
  cloud-agent setup/footer/banner strings or the MCP/computer-use/orchestration
  action-result summaries listed in this cut. The OSS lib check, default-feature
  lib check, debug OSS binary build, and `git diff --check` all pass; the
  default-feature check still reports pre-existing dead-code warnings in
  `crates/ai`.
- Replaced the AI artifact button row with an OSS no-op compatibility shim while
  preserving artifact serialization and event/action types for shared
  conversation, agent-management, notification, and shared-session code. This
  removes the real plan, branch, pull-request, screenshot, and file artifact
  buttons from OSS without changing the artifact data model.
- After the artifact-button cut, debug `warp-oss` is 365,054,472 bytes with
  `__TEXT` at 144,031,744 bytes, `__TEXT,__text` at 98,194,772 bytes,
  `__TEXT,__const` at 11,397,632 bytes, and `__LINKEDIT` at 216,678,400 bytes.
  Targeted scans no longer find the artifact button labels `Copy branch name`,
  `View screenshots`, `Open pull request`, `Open plan`, or `Download file`.
  The OSS lib check, default-feature lib check, debug OSS binary build, and
  `git diff --check` all pass; the default-feature check still reports
  pre-existing dead-code warnings in `crates/ai`.
- Gated the Oz launch tab-title helper in OSS so it focuses the OpenWarp launch
  placeholder instead of adding a tab named `Introducing Oz`, and renamed the
  OSS-only launch-modal setting/view strings away from Oz/OpenWarp-specific UI
  copy where the shared type surface allowed it. This is mainly string hygiene;
  the remaining `OpenWarpLaunchModal` strings come from shared type and feature
  names.
- After the Oz launch string cleanup, debug `warp-oss` is 365,036,696 bytes
  with `__TEXT` at 144,015,360 bytes, `__TEXT,__text` at 98,193,236 bytes,
  `__TEXT,__const` at 11,397,632 bytes, and `__LINKEDIT` at 216,678,400 bytes.
  Targeted scans no longer find `Introducing Oz`, `DidShowOzLaunchModal`,
  `Agent automations`, `A little gift`, `Break out of your laptop`,
  `Track local and cloud agents`, `1,000 free cloud`, or
  `Sync conversations to cloud`. The OSS lib check and debug OSS binary build
  pass.
- Gated notebook startup and shutdown roots in OSS: cached cloud notebooks are
  no longer collected during app initialization, `NotebookManager` and
  `NotebookKeybindings` are no longer registered, and shutdown no longer tries
  to flush notebook state. This keeps cloud-object/persistence types available
  for now but removes unconditional notebook runtime setup from the OSS app.
- After the notebook startup-root cut, debug `warp-oss` is 364,864,280 bytes
  with `__TEXT` at 143,966,208 bytes, `__TEXT,__text` at 98,154,240 bytes,
  `__TEXT,__const` at 11,397,632 bytes, and `__LINKEDIT` at 216,547,328 bytes.
  The OSS lib check, debug OSS binary build, and `git diff --check` pass.
  Targeted scans still find `NotebookManager`, `Notebook:`, `notebook_pane`,
  `notebook.svg`, and `warp_editor`, so workspace open/restore paths and
  persistence/search roots remain to cut.
- Gated workspace notebook-open roots in OSS: `Workspace::open_notebook` and
  the direct cloud/file notebook tab helpers are now no-ops, and
  cloud-object-to-pane creation returns `None` for notebooks in OSS. This keeps
  existing callers compiling while preventing workspace actions from rooting
  notebook pane creation.
- After the workspace notebook-open cut, debug `warp-oss` is 364,839,560 bytes
  with `__TEXT` at 143,949,824 bytes, `__TEXT,__text` at 98,147,328 bytes,
  `__TEXT,__const` at 11,397,632 bytes, and `__LINKEDIT` at 216,547,328 bytes.
  The OSS lib check, default-feature lib check, debug OSS binary build, and
  `git diff --check` pass; the default-feature check still reports the
  pre-existing `crates/ai` dead-code warnings. Targeted scans still find one
  `NotebookManager` string plus notebook pane/search/persistence/editor strings.
- Replaced the full notebook manager with an OSS shim. The shim preserves
  `NotebookSource`, the singleton type, and inert lookup/reset/raw-text methods
  for shared search/auth/pane code, but removes cached notebook raw-text
  parsing, update subscriptions, pane tracking maps, and notebook pane creation
  from the OSS module graph.
- After the notebook manager shim, debug `warp-oss` is 363,005,784 bytes with
  `__TEXT` at 143,278,080 bytes, `__TEXT,__text` at 97,647,808 bytes,
  `__TEXT,__const` at 11,393,536 bytes, and `__LINKEDIT` at 215,400,448 bytes.
  Targeted scans no longer find `Cached Notebook raw text` or
  `parse_markdown_to_raw_text`. The OSS lib check, default-feature lib check,
  debug OSS binary build, and `git diff --check` pass; the default-feature
  check still reports the pre-existing `crates/ai` dead-code warnings.
- Gated command-search notebook results out of OSS. The command-search
  notebook submodule is no longer compiled, notebook data-source registration
  is skipped, and notebook zero-state chips are not advertised in OSS.
- After the command-search notebook cut, debug `warp-oss` is 362,920,888 bytes
  with `__TEXT` at 143,245,312 bytes, `__TEXT,__text` at 97,633,216 bytes,
  `__TEXT,__const` at 11,393,536 bytes, and `__LINKEDIT` at 215,334,912 bytes.
  Targeted scans no longer find the command-search sample
  `notebooks: deploy production server`; remaining notebook strings come from
  Warp Drive/search, pane restoration/persistence, and the editor crate.
  The OSS lib check, default-feature lib check, debug OSS binary build, and
  `git diff --check` pass; the default-feature check still reports the
  pre-existing `crates/ai` dead-code warnings.
- Gated command-palette Warp Drive notebook and plan search out of OSS. The
  notebook/plan query paths, notebook search item type, notebook index refresh
  work, and command-palette notebook/plan filter metadata are no longer compiled
  or advertised in the OSS command-palette surface.
- After the Warp Drive notebook/plan search and filter-metadata cut, debug
  `warp-oss` is 362,751,288 bytes with `__TEXT` at 143,196,160 bytes,
  `__TEXT,__text` at 97,601,712 bytes, `__TEXT,__const` at 11,389,440 bytes,
  and `__LINKEDIT` at 215,220,224 bytes. Targeted scans no longer find
  `Search notebooks`; remaining strings include `Notebook:`, `CloudNotebook`,
  `NotebookManager`, `notebook_pane`, `notebooks:`, `plans:`,
  `bundled/svg/notebook.svg`, `bundled/svg/compass-3.svg`, and `warp_editor`
  from other roots. The OSS lib check, default-feature lib check, debug OSS
  binary build, and `git diff --check` pass; the default-feature check still
  reports the pre-existing `crates/ai` dead-code warnings.
- Gated embedded-notebook search out of OSS while keeping embedded workflow
  search available. The notebook embedding submodule, cloud-notebook data
  source, embedded notebook search item action, and notebook insertion handler
  are now non-OSS only.
- After the embedded-notebook search cut, debug `warp-oss` is 362,677,352 bytes
  with `__TEXT` at 143,179,776 bytes, `__TEXT,__text` at 97,585,328 bytes,
  `__TEXT,__const` at 11,393,536 bytes, and `__LINKEDIT` at 215,171,072 bytes.
  Targeted scans no longer find `CloudNotebooksDataSource` or
  `NotebookSearchItem` in the OSS binary; remaining matching strings include
  `AcceptNotebook`, `notebook_embedding`, `bundled/svg/notebook.svg`, and
  `bundled/svg/compass-3.svg` from command-search action metadata and broader
  notebook/editor roots. The OSS lib check, default-feature lib check, debug
  OSS binary build, and `git diff --check` pass; the default-feature check
  still reports the pre-existing `crates/ai` dead-code warnings.
- Gated the command-search `AcceptNotebook` action out of OSS now that
  command-search notebook results are already non-OSS. This removes the dead
  action arm from the OSS command-search event surface while leaving the
  non-OSS notebook result flow unchanged.
- After the command-search action metadata cleanup, debug `warp-oss` is
  362,676,680 bytes with `__TEXT` at 143,179,776 bytes, `__TEXT,__text` at
  97,584,816 bytes, `__TEXT,__const` at 11,393,536 bytes, and `__LINKEDIT` at
  215,171,072 bytes. Targeted scans no longer find `AcceptNotebook`,
  `CloudNotebooksDataSource`, or `NotebookSearchItem`; remaining matches
  include `notebook_embedding`, `bundled/svg/notebook.svg`, and
  `bundled/svg/compass-3.svg`. The OSS lib check, default-feature lib check,
  debug OSS binary build, and `git diff --check` pass; the default-feature
  check still reports the pre-existing `crates/ai` dead-code warnings.
- Gated the notebook embedded-object search module out of OSS. The notebook
  editor's block insertion menu no longer creates an embedded-object search view
  or exposes the Embed block-search entry in OSS, so `search::notebook_embedding`
  and its cloud workflow/notebook data sources are no longer compiled into the
  OSS app.
- After the notebook embedded-object search cut, debug `warp-oss` is
  361,851,560 bytes with `__TEXT` at 142,917,632 bytes, `__TEXT,__text` at
  97,391,776 bytes, `__TEXT,__const` at 11,389,440 bytes, and `__LINKEDIT` at
  214,597,632 bytes. Targeted scans no longer find `EmbeddingSearchMenu`,
  `notebook_embedding`, `CloudWorkflowsDataSource`, `CloudNotebooksDataSource`,
  `AcceptNotebook`, or `NotebookSearchItem`; remaining matches include
  `bundled/svg/notebook.svg` and `bundled/svg/compass-3.svg` from broader
  notebook/asset roots. The OSS lib check, default-feature lib check, debug OSS
  binary build, and `git diff --check` pass; the default-feature check still
  reports the pre-existing `crates/ai` dead-code warnings.
- Excluded the removed notebook/plan SVG payloads from the OSS embedded asset
  set. The shared icon metadata can still retain one path string for each icon,
  but the actual `notebook.svg` and `compass-3.svg` XML payloads are no longer
  embedded in the OSS binary.
- After the notebook/plan asset exclusion, debug `warp-oss` is 361,835,032
  bytes with `__TEXT` at 142,901,248 bytes, `__TEXT,__text` at 97,391,776
  bytes, `__TEXT,__const` at 11,381,248 bytes, and `__LINKEDIT` at
  214,597,632 bytes. Targeted scans no longer find
  `bundled/svg/notebook.svg<svg` or `bundled/svg/compass-3.svg<svg`; one path
  string for each icon remains through shared icon metadata. The OSS lib check,
  default-feature lib check, debug OSS binary build, and `git diff --check`
  pass; the default-feature check still reports the pre-existing `crates/ai`
  dead-code warnings.
- Gated command-palette/workspace notebook action pass-through out of OSS. The
  `CommandPaletteItemAction::OpenNotebook`, `CommandPaletteEvent::OpenNotebook`,
  `ItemSummary::Notebook`, and `WorkspaceAction::OpenNotebook` paths are now
  non-OSS only; the OSS command-palette still handles
  `CloudObjectTypeAndId::Notebook` as a generic cloud object where required for
  enum exhaustiveness.
- Gated notebook telemetry payload/event definitions and send sites out of OSS.
  The notebook open/edit/action telemetry metadata and display strings are now
  non-OSS only, while the remaining generic cloud-object telemetry metadata used
  by notebook code is preserved for compile-time compatibility.
- After the command-palette notebook action and notebook telemetry cuts, debug
  `warp-oss` is 361,753,128 bytes with `__TEXT` at 142,868,480 bytes,
  `__TEXT,__text` at 97,366,944 bytes, `__TEXT,__const` at 11,377,152 bytes,
  and `__LINKEDIT` at 214,548,480 bytes. Targeted scans no longer find
  `OpenNotebook`, `ItemSummary::Notebook`,
  `CommandPaletteItemAction::OpenNotebook`, `Opened a notebook`,
  `Notebook Opened`, `Notebook Edited`, `Notebook Action`, `Deleted Notebook`,
  `Deleted notebook from Warp Drive team`, `Took an action on a notebook`,
  `NotebookTelemetryMetadata`, or `NotebookActionEvent` in the OSS binary. The
  OSS lib check, default-feature lib check, debug OSS binary build, and
  `git diff --check` pass; the default-feature check still reports the
  pre-existing `crates/ai` dead-code warnings.
- Replaced the full notebook view/editor/file module tree with OSS shims. The
  full `notebooks::{editor,file,notebook,telemetry,...}` modules are now
  non-OSS only; OSS retains shared notebook IDs/cloud models/export helpers plus
  inert `NotebooksEditorModel`, `RichTextEditorView`, `FileNotebookView`, and
  `NotebookView` compatibility shims for remaining pane and document type
  references.
- After the notebook shim cut, debug `warp-oss` is 342,139,768 bytes with
  `__TEXT` at 134,479,872 bytes, `__TEXT,__text` at 91,036,820 bytes,
  `__TEXT,__const` sections at 11,199,168 and 3,686,832 bytes, and
  `__LINKEDIT` at 203,882,496 bytes. Targeted scans no longer find
  `OpenNotebook`, `NotebookTelemetryMetadata`, `NotebookActionEvent`, or
  `warp_editor`; remaining notebook matches are the lightweight shim type names
  and pane names. The OSS lib check, default-feature lib check, debug OSS binary
  build, and `git diff --check` pass; the default-feature check still reports
  the pre-existing `crates/ai` dead-code warnings.
- Replaced the OSS notebook/file pane wrappers with inert pane shims and made
  the app's direct `warp_editor` dependency default-only. OSS now aliases a
  tiny in-crate `warp_editor_shim` for shared types such as `NavigationKey`,
  `LineCount`, `EditOrigin`, and rich-text style placeholders; default-feature
  builds still enable and compile the real `crates/editor` dependency.
- After making `warp_editor` optional for OSS, debug `warp-oss` is 341,871,048
  bytes with `__TEXT` at 134,529,024 bytes, `__TEXT,__text` at 91,087,680
  bytes, `__TEXT,__const` sections at 11,195,072 and 3,686,864 bytes, and
  `__LINKEDIT` at 203,571,200 bytes. `cargo tree -i warp_editor` for the OSS
  feature set shows only the app dev-dependency path. Targeted scans still show
  the lightweight notebook shim type and pane names plus one `warp_editor` path
  string from the local compatibility shim, but no real `warp_editor` dependency
  is present in the OSS build graph. The OSS lib check, default-feature lib
  check, debug OSS binary build, and `git diff --check` pass; the
  default-feature check still reports the pre-existing `crates/ai` dead-code
  warnings.
- Replaced the OSS code-review comments module with a small compatibility shim
  and gated the code-review undo fixed binding out of OSS. The full GitHub
  imported-comment conversion path, diff-hunk parser, thread flattener, and
  batch relocation storage are now non-OSS only; OSS keeps the comment/batch
  types needed by still-shared AI and pane event signatures but makes imported
  comments inert.
- After the code-review comments shim, debug `warp-oss` is 341,610,568 bytes
  with `__TEXT` at 134,447,104 bytes, `__TEXT,__text` at 91,015,780 bytes,
  `__TEXT,__const` sections at 11,195,072 and 3,685,344 bytes, and
  `__LINKEDIT` at 203,390,976 bytes. Targeted scans no longer find
  `diff_hunk_parser`, `convert_insert_review_comments`,
  `attach_pending_imported_comments`, `ImportedFromGitHub`, `CodeReviewView`,
  `UndoRevert`, `ReviewCommentBatch`, `PendingImportedReviewComment`,
  `github_parent_comment_id`, or `comment_line_range` in the OSS binary. The
  OSS lib check, default-feature lib check, debug OSS binary build, and
  `git diff --check` pass; the default-feature check still reports the
  pre-existing `crates/ai` dead-code warnings.
- Replaced code-review telemetry and diff-size modules with OSS shims and gated
  the code-review editor-state module out of OSS. The OSS build keeps the
  public telemetry enum variants and `DiffSize` values required by shared
  terminal/AI/right-panel signatures, but drops code-review telemetry payload
  construction, event-description registration, diff-size heuristics, and
  `LocalCodeEditorView` state glue.
- After the code-review telemetry/editor-state shims, debug `warp-oss` is
  341,557,640 bytes with `__TEXT` at 134,414,336 bytes, `__TEXT,__text` at
  90,997,348 bytes, `__TEXT,__const` sections at 11,191,040 and 3,684,568
  bytes, and `__LINKEDIT` at 203,374,592 bytes. Targeted scans no longer find
  `CodeReview.PaneOpened`, `CodeReview.AddToContext`,
  `CodeReview.GitDialogCompleted`, `Code review pane opened`,
  `Content added to AI context from code review`, `Git operation dialog
  reached`, `GitButtonTriggered`, `GitDialogStatus`, `CodeReviewEditorState`,
  `compute_diff_size`, or `MAX_DIFF_SIZE` in the OSS binary. The OSS lib check,
  default-feature lib check, debug OSS binary build, and `git diff --check`
  pass; the default-feature check still reports the pre-existing `crates/ai`
  dead-code warnings.
- Replaced the OSS conversation details panel with an inert compatibility view.
  The full details panel UI, copy-link/copy-field machinery, artifact button
  row wiring, Oz/continue-locally actions, environment/credit/status rendering,
  and cloud-run metadata formatting are now non-OSS only. OSS keeps
  `ConversationDetailsData`, `ConversationDetailsPanel`, and event constructor
  signatures so terminal/workspace/management call sites remain type-compatible.
- After the conversation details panel shim, debug `warp-oss` is 341,016,728
  bytes with `__TEXT` at 134,184,960 bytes, `__TEXT,__text` at 90,828,116
  bytes, `__TEXT,__const` sections at 11,191,040 and 3,679,352 bytes, and
  `__LINKEDIT` at 203,079,680 bytes. Targeted scans no longer find `View in
  Oz`, `Environment details`, `Credits spent on AI model requests`, `Created
  by`, or `Copied branch name` in the OSS binary. A few generic details strings
  such as `ConversationDetailsPanel`, `Continue locally`, `Open in GitHub`, and
  `Cloud agent run` still remain through other AI/management surfaces. The OSS
  lib check, default-feature lib check, debug OSS binary build, and
  `git diff --check` pass; the default-feature check still reports the
  pre-existing `crates/ai` dead-code warnings.
- Gated the agent-management details action button row out of OSS and replaced
  agent-management telemetry with an OSS compatibility shim. The OSS build keeps
  the telemetry enum variants and small origin/artifact/filter enums needed by
  remaining workspace, conversation-list, slash-command, and tombstone call
  sites, but drops JSON payload construction, strum discriminants, event
  descriptions, and details-button UI tooltips from the OSS binary.
- After the agent-management details-button/telemetry shim, debug `warp-oss` is
  341,013,160 bytes with `__TEXT` at 134,184,960 bytes, `__TEXT,__text` at
  90,816,596 bytes, `__TEXT,__const` sections at 11,191,040 and 3,678,504
  bytes, and `__LINKEDIT` at 203,063,296 bytes. Targeted scans no longer find
  `Open conversation`, `Cancel task`, `View details`, `Copy link to run`,
  `AgentManagement.ViewToggled`, `AgentManagement.OpenSetupGuide`,
  `ConversationLinkCopied`, `SetupGuideStepRun`,
  `DetailsPanelContinueLocally`, or `SlashCommandContinueLocally` in the OSS
  binary; one generic `Fork conversation` string still remains elsewhere. The
  OSS lib check, default-feature lib check, and debug OSS binary build pass; the
  default-feature check still reports the pre-existing `crates/ai` dead-code
  warnings.
- Replaced the OSS agent-tips module with an inert compatibility shim. The OSS
  build keeps the `AITip`, `AITipModel`, `AgentTip`, and
  `WorkspaceAction::display_text` surface required by status-bar and cloud-mode
  loading-screen signatures, but drops the static tips list, docs URLs,
  keybinding lookup, random selection/cooldown logic, and codebase-index
  applicability checks.
- After the agent-tips shim, debug `warp-oss` is 340,992,216 bytes with
  `__TEXT` at 134,168,576 bytes, `__TEXT,__text` at 90,814,548 bytes,
  `__TEXT,__const` sections at 11,191,040 and 3,678,016 bytes, and
  `__LINKEDIT` at 203,063,296 bytes. Targeted scans no longer find
  `agent-platform/capabilities/slash-commands`,
  `agent-platform/capabilities/mcp`, `Right-click a block to fork the
  conversation`, `Drag an image into the pane`, `Use the \`oz\` command`,
  `AgentTip Shown`, `AgentTip Clicked`, or `Toggle Show Agent Tips` in the OSS
  binary. One generic `Tip: ` string remains elsewhere. The OSS lib check,
  default-feature lib check, and debug OSS binary build pass; the
  default-feature check still reports the pre-existing `crates/ai` dead-code
  warnings.
- Replaced skills telemetry with an OSS compatibility shim. The OSS build keeps
  `SkillOpenOrigin` and `SkillTelemetryEvent` variants used by read/open skill
  call sites, but drops skill telemetry payload construction, strum
  discriminants, event descriptions, and the `ListSkills` feature-gated
  telemetry registration metadata.
- After the skills telemetry shim, debug `warp-oss` is 340,979,304 bytes with
  `__TEXT` at 134,168,576 bytes, `__TEXT,__text` at 90,809,428 bytes,
  `__TEXT,__const` sections at 11,191,040 and 3,678,240 bytes, and
  `__LINKEDIT` at 203,046,912 bytes. Targeted scans no longer find
  `Skill.Read`, `Skill.Opened`, `A skill was read via the ReadSkill tool call`,
  or `A skill was opened from an 'open skill' button` in the OSS binary.
  Generic skill strings such as `ReadSkill` still remain from the shared skill
  command/action model. The OSS lib check, default-feature lib check, and debug
  OSS binary build pass; the default-feature check still reports the
  pre-existing `crates/ai` dead-code warnings.
- Replaced ambient/cloud-agent telemetry with an OSS compatibility shim. The
  OSS build keeps the cloud-mode entry-point and cloud-agent event enum surface
  required by environment form, environment selector, workspace, and ambient
  agent call sites, but drops telemetry payload construction, strum
  discriminants, event descriptions, and the cloud-mode feature-flagged
  registration metadata from the OSS build. The OSS lib check passed after this
  cut; a debug binary size measurement was deferred because the local build
  artifacts were removed to recover disk space.
- Replaced agent SDK CLI telemetry with an OSS compatibility shim. The OSS
  build keeps `CliTelemetryEvent` variants required by `command_to_telemetry_event`,
  but drops CLI event names/descriptions, payload construction, strum
  discriminants, and feature-flagged telemetry registration metadata for agent,
  MCP, environment, artifact, schedule, secret, federate, and harness-support
  commands.
- Replaced blocklist orchestration telemetry with an OSS compatibility shim. The
  OSS build keeps the team-agent communication failure enums and struct required
  by send-message and orchestration call sites, but drops JSON serialization,
  strum discriminants, event names, and event descriptions.
- After the ambient telemetry, agent SDK telemetry, and blocklist telemetry
  shims, debug `warp-oss` is 340,940,264 bytes with `__TEXT` at 134,152,192
  bytes, `__TEXT,__text` at 90,794,580 bytes, `__TEXT,__const` sections at
  11,191,040 and 3,677,304 bytes, and `__LINKEDIT` at 203,030,528 bytes.
  Targeted scans no longer find ambient cloud-mode event strings, CLI agent/MCP
  event strings, CLI harness-support descriptions, or
  `AgentMode.Orchestration.TeamAgentCommunicationFailed` in the OSS binary. The
  OSS lib check, debug OSS binary build, and `git diff --check` pass.
- Replaced request-file-edits telemetry with an OSS compatibility shim. The OSS
  build keeps code-diff event structs, `RequestFileEditsFormatKind`, and the
  telemetry event enum required by executor and code-diff view call sites, but
  drops JSON serialization, strum discriminants, event names, event
  descriptions, and UGC metadata dispatch.
- Replaced LSP telemetry with an OSS compatibility shim. The OSS build keeps the
  source/action/event enum surface required by settings, init-project, footer,
  persisted-workspace, and editor call sites, but drops payload serialization,
  strum discriminants, event names, and descriptions for LSP enablement,
  hover/goto/find-references, and server lifecycle events.
- After the request-file-edits and LSP telemetry shims, debug `warp-oss` is
  340,871,320 bytes with `__TEXT` at 134,119,424 bytes, `__TEXT,__text` at
  90,769,748 bytes, `__TEXT,__const` sections at 11,191,040 and 3,675,496
  bytes, and `__LINKEDIT` at 202,997,760 bytes. Targeted scans no longer find
  `AgentMode.Code.*` suggested-edit event strings or `Lsp.*` event strings in
  the OSS binary. The OSS lib check, debug OSS binary build, and
  `git diff --check` pass.
- Replaced tab-config telemetry with an OSS compatibility shim. The OSS build
  keeps the existing/open/new-worktree/guided-modal enum surface and
  `GuidedModalSessionType::from(&SessionType)` conversion required by workspace
  call sites, but drops payload serialization, strum discriminants, event names,
  and descriptions for saved tab config and worktree flows.
- After the tab-config telemetry shim, debug `warp-oss` is 340,863,880 bytes
  with `__TEXT` at 134,119,424 bytes, `__TEXT,__text` at 90,765,652 bytes,
  `__TEXT,__const` sections at 11,191,040 and 3,678,560 bytes, and
  `__LINKEDIT` at 202,981,376 bytes. Targeted scans no longer find
  `TabConfigs.*` event strings or saved-tab/worktree telemetry descriptions in
  the OSS binary. The OSS lib check, debug OSS binary build, and
  `git diff --check` pass.
- Gated the central `server::telemetry::events::TelemetryEvent` payload,
  enablement, and UGC methods for OSS. The OSS build already returned
  `TelemetryDisabled` for central event names and empty descriptions; it now
  also returns no payload, `Always` enablement, and no UGC classification
  without compiling the full JSON payload and discriminant match logic. This
  preserves the large central enum surface required by existing call sites while
  dropping telemetry payload plumbing for AI/auth/git/editor/server events from
  the OSS binary.
- After the central telemetry gate, debug `warp-oss` is 339,676,616 bytes with
  `__TEXT` at 133,480,448 bytes, `__TEXT,__text` at 90,281,500 bytes,
  `__TEXT,__const` sections at 11,170,560 and 3,670,448 bytes, and
  `__LINKEDIT` at 202,440,704 bytes. Representative scans no longer find
  central payload keys such as `insertion_length`,
  `block_finished_to_precmd_delay_ms`, `is_in_agent_view`, or
  `terminal_session_id`, nor central event strings such as `AgentView.Entered`,
  `AgentMode.CodeSelectionAddedAsContext`, `SearchCodebaseRequested`,
  `AIInputNotSent`, or `FreeTierLimitHitInterstitial`. Some generic strings
  such as `logging_id`, `conversation_id`, and `request_id` remain via
  non-telemetry code. The OSS lib check, default-feature lib check, debug OSS
  binary build, and `git diff --check` pass; the default-feature check still
  reports the pre-existing `crates/ai` dead-code warnings.
- Gated `crates/ai` and `crates/repo_metadata` telemetry payload, enablement,
  and UGC methods for OSS. This removes codebase-index/repo-metadata telemetry
  payload keys and event metadata from the OSS binary while keeping the event
  enum surfaces used by those crates.
- After the `crates/ai`/`repo_metadata` telemetry gates, debug `warp-oss` is
  339,678,056 bytes with `__TEXT` at 133,480,448 bytes, `__TEXT,__text` at
  90,281,500 bytes, `__TEXT,__const` sections at 11,170,560 and 3,670,288
  bytes, and `__LINKEDIT` unchanged at 202,440,704 bytes. Targeted scans no
  longer find codebase-index telemetry strings such as
  `AgentMode.SyncCodebaseContext.Success`, `total_sync_duration`,
  `flushed_node_count`, or repo-metadata strings such as
  `RepoMetadata.BuildTree.Failed`. The measured debug binary moved by about
  +1.4 KB versus the prior checkpoint, so this is recorded as a telemetry
  cleanup/verification cut rather than a size win. The OSS lib check, debug OSS
  binary build, and `git diff --check` pass.
- Gated AI-only search filter metadata for OSS. Natural-language command
  suggestions, AI rules, skill browsing, base-model, and full-terminal-use model
  filters now return empty/no-op placeholder, display, filter-atom, and icon
  metadata under `oss_release`, matching the existing notebook/plan OSS pattern.
- After the AI search-filter metadata cut, debug `warp-oss` is 339,668,184
  bytes with `__TEXT` unchanged at 133,480,448 bytes, `__TEXT,__text` at
  90,279,708 bytes, `__TEXT,__const` sections at 11,170,560 and 3,670,520
  bytes, and `__LINKEDIT` at 202,424,320 bytes. Targeted scans no longer find
  `e.g. replace string in file`, `Search AI rules`, `Search base models`,
  `Search full terminal use models`, `full terminal use models`, or
  `base models`. `AI command suggestions` and `Search skills` still remain via
  other UI paths. The OSS lib check, debug OSS binary build, and
  `git diff --check` pass.
- Gated the terminal input AI command-search hint and skill-menu placeholder for
  OSS. After this follow-up, targeted scans no longer find
  `AI command suggestions`, `Type '#'`, `Search skills`, `Search base models`,
  or `full terminal use models`. The debug `warp-oss` size is unchanged at
  339,668,184 bytes; the OSS lib check, default-feature lib check, debug OSS
  binary build, and `git diff --check` pass. The default-feature check still
  reports the pre-existing `crates/ai` dead-code warnings.
- Gated provider-specific managed-secret command/schema/value metadata for OSS.
  The OSS `warp_cli` secret command no longer exposes Anthropic/Bedrock
  provider subcommands or secret-type variants, the app-local managed-secret
  shim only keeps raw/dotenvx secret types, and the agent SDK secret/env-var
  branches for Anthropic and Bedrock are now non-OSS only.
- After the managed-secret provider metadata cut, debug `warp-oss` is
  339,674,120 bytes with `__TEXT` unchanged at 133,480,448 bytes,
  `__TEXT,__text` at 90,279,708 bytes, `__TEXT,__const` sections at
  11,170,560 and 3,670,432 bytes, and `__LINKEDIT` at 202,424,320 bytes.
  Targeted scans no longer find provider command/schema/runtime strings such as
  `anthropic-api-key`, `anthropic-bedrock`, `anthropic_api_key`,
  `anthropic_bedrock`, `ManagedSecret::Anthropic`, `CLAUDE_CODE_USE_BEDROCK`,
  `AWS_BEARER_TOKEN_BEDROCK`, `AWS_SECRET_ACCESS_KEY`, `AWS_ACCESS_KEY_ID`,
  `Bedrock API key`, or `Bedrock access key`. Generic secret-redaction patterns
  still include labels such as `Anthropic API Key` and `AWS Access ID` because
  those are terminal-safe privacy defaults rather than managed-secret UI. The
  OSS lib check, default-feature lib check, debug OSS binary build, and
  `git diff --check` pass; the default-feature check still reports the
  pre-existing `crates/ai` dead-code warnings.
- Replaced the OSS `ai::api_keys` and `ai::aws_credentials` modules with
  compatibility shims. OSS still exposes the `ApiKeyManager`, API-key state, and
  AWS credential state methods expected by remaining shared call sites, but no
  longer loads/stores AI API keys in secure storage, builds provider API-key
  request payloads, or keeps the AWS Bedrock credential status copy.
- After the API-key/AWS credential shims, debug `warp-oss` is 339,643,176 bytes
  with `__TEXT` at 133,464,064 bytes, `__TEXT,__text` at 90,268,848 bytes,
  `__TEXT,__const` sections at 11,170,368 and 3,670,320 bytes, and
  `__LINKEDIT` at 202,424,320 bytes. Targeted scans no longer find `AiApiKeys`,
  `Failed to deserialize API keys`, `Failed to read API keys`,
  `Failed to write API keys`, `struct ApiKeys`,
  `AWS credentials not configured`, `AWS Bedrock Disabled`,
  `Loading your AWS CLI credentials`, or `Unable to load credentials`. The
  `open_router` and `allow_use_of_warp_credits` strings still remain through
  shared `warp_multi_agent_api` request field names. The OSS lib check,
  default-feature lib check, debug OSS binary build, and `git diff --check`
  pass; the default-feature check still reports the pre-existing `crates/ai`
  dead-code warnings.
- Replaced the OSS `ai::skills` module with a compatibility shim. OSS still
  exposes the skill provider/scope/reference/parsed-skill types and API-model
  conversions that remaining shared signatures expect, but no longer keeps the
  real provider path table, provider icon/display metadata, markdown skill
  parser, skill directory walking, bundled-skill reference display prefix, or
  parser error strings.
- After the skill shim, debug `warp-oss` is 339,581,608 bytes with `__TEXT` at
  133,447,680 bytes, `__TEXT,__text` at 90,251,440 bytes, `__TEXT,__const`
  sections at 11,170,368 and 3,669,696 bytes, and `__LINKEDIT` at 202,375,168
  bytes. Targeted scans no longer find real skill parser/provider strings such
  as `Block separator regex`, `Incomplete sentence regex`,
  `Could not derive skill name`, `.claude/skills`, `.codex/skills`,
  `.cursor/skills`, `.gemini/skills`, `.opencode/skills`, `.factory/skills`,
  `Read all skills from`, `@warp-skill`, `No descriptor provided`,
  `Invalid provider`, `Invalid scope`, or `Invalid content`. The broader scan
  still sees global icon enum names such as `ClaudeLogo`, `GeminiLogo`,
  `OpenAILogo`, `DroidLogo`, and `OpenCodeLogo`; those come from the shared
  icon set rather than the skill provider table. The OSS lib check,
  default-feature lib check, debug OSS binary build, and `git diff --check`
  pass; the default-feature check still reports the pre-existing `crates/ai`
  dead-code warnings.
- Replaced the OSS `app::ai::llms` module with a compatibility shim. OSS still
  exposes the model metadata types, model-preferences singleton, default-model
  accessors, profile override methods, and API conversion targets used by shared
  signatures, but no longer subscribes to auth/network/workspace changes, fetches
  LLM metadata from the server, reads or writes the cached model list, checks
  BYOK state, maps providers to AI logo icons, or keeps the production model
  update/popup logic.
- After the LLM shim, debug `warp-oss` is 339,557,400 bytes with `__TEXT` at
  133,431,296 bytes, `__TEXT,__text` at 90,243,688 bytes, `__TEXT,__const`
  sections at 11,170,368 and 3,668,776 bytes, and `__LINKEDIT` at 202,358,784
  bytes. Targeted scans no longer find LLM fetch/cache/model-default strings such
  as `Failed to fetch LLMs from server`,
  `Failed to fetch free-tier LLMs from server`, `Failed to cache LLMs`,
  `Failed to serialize LLMs`, `Failed to deserialize cached LLMs`,
  `Duplicate LLMModelHost entry`, `Tried to create AvailableLLMs`,
  `Default LLM ID`, `This model has been disabled by your team admin`,
  `Please upgrade your plan`, `provider outage`, `auto (cost-efficient)`,
  `auto (responsive)`, `computer-use-agent-auto`, `cli-agent-auto`, or
  `AvailableLLMs`. The OSS lib check, default-feature lib check, debug OSS
  binary build, and `git diff --check` pass; the default-feature check still
  reports the pre-existing `crates/ai` dead-code warnings.
- Routed `crates::ai::index::full_source_code_embedding` to the existing
  unavailable embedding shim in OSS. This keeps the public indexing API shape
  available to shared call sites, but avoids compiling the real local source
  embedding implementation into the OSS feature set.
- After the source-code embedding route, debug `warp-oss` remains 339,557,400
  bytes with `__TEXT` at 133,431,296 bytes, `__TEXT,__text` at 90,243,688 bytes,
  `__TEXT,__const` sections at 11,170,368 and 3,668,776 bytes, and `__LINKEDIT`
  at 202,358,784 bytes. Targeted scans no longer find heavy real implementation
  strings from the local embedding path; remaining matches are feature/settings
  names and generic unavailable-shim errors. The OSS lib check, default-feature
  lib check, and debug OSS binary build pass; the default-feature check still
  reports the pre-existing `crates/ai` dead-code warnings.
- Trimmed the OSS `AISettings` compatibility surface by removing disabled
  setting entries whose remaining references were confined to already-shimmed AI
  settings pages, inline action views, agent-management views, billing views, or
  no-op plugin chip methods. The remaining OSS `AISettings` still keeps the
  fields read by shared terminal/editor/workspace code.
- After the OSS `AISettings` trim, debug `warp-oss` is 338,840,056 bytes with
  `__TEXT` at 133,103,616 bytes, `__TEXT,__text` at 89,988,712 bytes,
  `__TEXT,__const` sections at 11,170,368 and 3,665,040 bytes, and
  `__LINKEDIT` at 201,998,336 bytes. Targeted scans no longer find removed
  setting type names including `NaturalLanguageAutosuggestionsEnabled`,
  `SharedBlockTitleGenerationEnabled`, `IntelligentAutosuggestionsEnabled`,
  `CodeSuggestionsEnabled`, `GitOperationsAutogenEnabled`,
  `DidDismissAgentManagementHelpPage`, `FtuModelCalloutDismissed`,
  `AmbientAgentTrialWidgetDismissed`, `AgentAttributionEnabled`,
  `PluginInstallChipDismissedMap`, or
  `PluginUpdateChipDismissedForVersionMap`. The OSS lib check, debug OSS binary
  build, and `git diff --check` pass.
- Replaced the OSS terminal `use_agent_footer` implementation with a
  compatibility shim. The shim preserves the `TerminalView` and
  `UseAgentToolbar` methods that shared terminal/session code calls, but it no
  longer renders or subscribes to the CLI-agent footer, rich-input controls,
  file/image paste path, remote-control chips, code-review/file-explorer
  toggles, or Warpify footer UI in OSS.
- After the terminal footer shim, debug `warp-oss` is 337,838,968 bytes with
  `__TEXT` at 132,677,632 bytes, `__TEXT,__text` at 89,667,284 bytes,
  `__TEXT,__const` sections at 11,157,504 and 3,631,984 bytes, and `__LINKEDIT`
  at 201,457,664 bytes. This removes about 1.0 MB from the debug binary
  compared with the previous checkpoint. The OSS lib check and debug OSS binary
  build pass. Some CLI-agent command/keybinding/settings strings still remain
  through shared terminal actions, keyboard context flags, onboarding/settings
  compatibility, and broader agent input surfaces; those are follow-up cuts.
- Removed the OSS CLI-agent footer settings metadata that is now redundant with
  the terminal footer shim, and gated the remaining shared terminal/workspace/
  onboarding reads behind non-OSS cfgs. Debug `warp-oss` is 337,367,832 bytes
  with `__TEXT` at 132,481,024 bytes, `__TEXT,__text` at 89,515,008 bytes,
  `__TEXT,__const` sections at 11,157,504 and 3,630,688 bytes, and `__LINKEDIT`
  at 201,179,136 bytes. This removes another 471,136 bytes from the debug
  binary. The OSS lib check and debug OSS binary build pass.
- Replaced the OSS CLI-agent plugin-manager and plugin-instructions rich-content
  view with compatibility shims. The manager was already functionally disabled
  in OSS, so the measurable win came from dropping the real instructions view
  rendering path and its code-block/copy UI. Debug `warp-oss` is 337,201,144
  bytes with `__TEXT` at 132,415,488 bytes, `__TEXT,__text` at 89,475,072 bytes,
  `__TEXT,__const` sections at 11,157,504 and 3,629,768 bytes, and `__LINKEDIT`
  at 201,080,832 bytes. This removes another 166,688 bytes from the debug
  binary. The OSS lib check and debug OSS binary build pass.
- Replaced the OSS CLI-agent event parser and PTY listener with compatibility
  shims. The public event/session API remains available to shared terminal code,
  but OSS no longer parses CLI-agent OSC payloads or subscribes listener models
  to PTY notification events. Debug `warp-oss` is 337,125,992 bytes with
  `__TEXT` at 132,382,720 bytes, `__TEXT,__text` at 89,441,536 bytes,
  `__TEXT,__const` sections at 11,157,504 and 3,629,008 bytes, and `__LINKEDIT`
  at 201,031,680 bytes. This removes another 75,152 bytes from the debug binary.
  The OSS lib check and debug OSS binary build pass.
- Gated the shared long-running-command timer's CLI-agent detection/session
  creation path out of OSS. The compatibility footer already returns no
  detection, so this mostly removes now-unreachable glue rather than a major
  feature root. Debug `warp-oss` is 337,121,192 bytes with `__TEXT` at
  132,382,720 bytes, `__TEXT,__text` at 89,438,720 bytes, `__TEXT,__const`
  sections at 11,157,504 and 3,628,984 bytes, and `__LINKEDIT` at 201,031,680
  bytes. This removes another 4,800 bytes from the debug binary. The OSS lib
  check and debug OSS binary build pass.
- Made `CLIAgentSessionsModel` inert in OSS while preserving its shared
  terminal API. OSS now keeps the session/event types available for call sites,
  but the singleton stores no sessions, emits no CLI-agent lifecycle events, and
  returns `None`/`false` from session/input/plugin-failure queries. Debug
  `warp-oss` is 337,059,048 bytes with `__TEXT` at 132,349,952 bytes,
  `__TEXT,__text` at 89,423,344 bytes, `__TEXT,__const` sections at 11,157,504
  and 3,628,640 bytes, and `__LINKEDIT` at 200,998,912 bytes. This removes
  another 62,144 bytes from the debug binary. The OSS lib check and debug OSS
  binary build pass.
- Gated the remaining `CLIAgentSessionsModel` subscription closures out of OSS
  across workspace, right-panel, terminal view/input, and shared-session
  managers. Since the OSS sessions model is inert, these subscriptions could
  never observe a lifecycle/input event. Debug `warp-oss` is 336,999,480 bytes
  with `__TEXT` at 132,333,568 bytes, `__TEXT,__text` at 89,403,632 bytes,
  `__TEXT,__const` sections at 11,157,504 and 3,627,448 bytes, and `__LINKEDIT`
  at 200,949,760 bytes. This removes another 59,568 bytes from the debug
  binary. The OSS lib check and debug OSS binary build pass.
- Stopped exporting `WARP_CLI_AGENT_PROTOCOL_VERSION` from OSS PTY environment
  setup. OSS no longer parses CLI-agent OSC events, so advertising a protocol
  version to shells and Docker sessions was dead protocol plumbing. Debug
  `warp-oss` is 336,994,152 bytes with `__TEXT` at 132,333,568 bytes,
  `__TEXT,__text` at 89,403,376 bytes, `__TEXT,__const` sections at 11,157,504
  and 3,628,144 bytes, and `__LINKEDIT` at 200,949,760 bytes. This removes
  another 5,328 bytes from the debug binary. The OSS lib check and debug OSS
  binary build pass.
- Gated shared-session CLI-agent state application and local broadcast plumbing
  out of OSS. OSS still serializes the inactive compatibility state required by
  the shared-session protocol, but it no longer subscribes to local CLI-agent
  lifecycle events or applies viewer-provided CLI-agent rich-input state.
  Debug `warp-oss` is 336,962,696 bytes with `__TEXT` at 132,317,184 bytes,
  `__TEXT,__text` at 89,390,492 bytes, `__TEXT,__const` sections at 11,157,504
  and 3,627,600 bytes, and `__LINKEDIT` at 200,933,376 bytes. This removes
  another 31,456 bytes from the debug binary. The OSS lib check and debug OSS
  binary build pass.
- Gated remaining CLI-agent session lookups out of OSS agent-icon and vertical
  tab display helpers. The OSS sessions model is inert, so these surfaces always
  fell through to their non-CLI-agent terminal/conversation display paths.
  Debug `warp-oss` is 336,958,104 bytes with `__TEXT` at 132,317,184 bytes,
  `__TEXT,__text` at 89,390,236 bytes, `__TEXT,__const` sections at 11,157,504
  and 3,627,552 bytes, and `__LINKEDIT` at 200,933,376 bytes. This removes
  another 4,592 bytes from the debug binary. The OSS lib check and debug OSS
  binary build pass.
- Gated long-running-command agent interaction state out of OSS shared-session
  flows. OSS still carries normal shared terminal/input context, but no longer
  computes, applies, emits, or broadcasts agent-control/tagged-in state for long
  running commands. Debug `warp-oss` is 336,934,456 bytes with `__TEXT` at
  132,300,800 bytes, `__TEXT,__text` at 89,380,252 bytes, `__TEXT,__const`
  sections at 11,157,504 and 3,627,424 bytes, and `__LINKEDIT` at 200,916,992
  bytes. This removes another 23,648 bytes from the debug binary. The OSS lib
  check and debug OSS binary build pass.
- Gated the CLI-agent plugin-installed listener registration path and
  sentinel/Codex notification handling out of OSS. The OSS footer shim no
  longer exposes the plugin-installed event, and terminal view no longer
  compiles the listener registration helpers or OSC sentinel parser in OSS.
  Debug `warp-oss` is 336,900,360 bytes with `__TEXT` at 132,284,416 bytes,
  `__TEXT,__text` at 89,370,780 bytes, `__TEXT,__const` sections at 11,157,504
  and 3,627,392 bytes, and `__LINKEDIT` at 200,900,608 bytes. This removes
  another 34,096 bytes from the debug binary. The OSS lib check and debug OSS
  binary build pass.
- Short-circuited remaining isolated CLI-agent session lookups in OSS chrome,
  context-chip, skill, and slash-command data-source helpers. These helpers now
  return the same OSS outcomes directly (`None`/`false`) and skip the
  CLI-agent input-session subscription in OSS. Debug `warp-oss` is 336,892,472
  bytes with `__TEXT` at 132,284,416 bytes, `__TEXT,__text` at 89,367,964
  bytes, `__TEXT,__const` sections at 11,157,504 and 3,627,504 bytes, and
  `__LINKEDIT` at 200,900,608 bytes. This removes another 7,888 bytes from the
  debug binary. The OSS lib check and debug OSS binary build pass.
- Short-circuited the last small CLI-agent session lookups in terminal/workspace
  command helpers. OSS now uses the same direct fallback values for prompt
  prefixes, active-agent checks, rich-input visibility, cursor suppression, and
  keybinding context instead of querying the inert CLI-agent sessions model.
  Debug `warp-oss` is 336,889,688 bytes with `__TEXT` at 132,284,416 bytes,
  `__TEXT,__text` at 89,366,940 bytes, `__TEXT,__const` sections at 11,157,504
  and 3,627,504 bytes, and `__LINKEDIT` at 200,900,608 bytes. This removes
  another 2,784 bytes from the debug binary. The OSS lib check and debug OSS
  binary build pass.
- Short-circuited OSS MCP permission checks in `BlocklistAIPermissions`.
  MCP tool/resource execution is unavailable in OSS, so permission checks now
  return `false` directly and protected-path checks no longer consult MCP config
  path detection in the OSS build. Debug `warp-oss` is 336,870,056 bytes with
  `__TEXT` at 132,268,032 bytes, `__TEXT,__text` at 89,363,868 bytes,
  `__TEXT,__const` sections at 11,153,408 and 3,627,392 bytes, and
  `__LINKEDIT` at 200,884,224 bytes. This removes another 19,632 bytes from the
  debug binary. The OSS lib check and debug OSS binary build pass.
- Removed the OSS-only code-diff `OpenMCPConfig` event/action compatibility
  path. OSS no longer exposes MCP config opening from code-diff inline actions,
  matching the inert MCP surface in this build. Debug `warp-oss` is 336,870,008
  bytes with `__TEXT` at 132,268,032 bytes, `__TEXT,__text` at 89,363,100 bytes,
  `__TEXT,__const` sections at 11,157,504 and 3,627,696 bytes, and
  `__LINKEDIT` at 200,884,224 bytes. This removes another 48 bytes from the
  debug binary. The OSS lib check and debug OSS binary build pass.
- Trimmed additional full-code-diff-only event/action variants out of the OSS
  `CodeDiffView` shim and gated the corresponding block/terminal handlers.
  The OSS shim has no rendered controls or keybindings for these paths, so it
  now keeps only the event/action surface that can actually be emitted by the
  shim or called by shared parents. Debug `warp-oss` is 336,861,640 bytes with
  `__TEXT` at 132,268,032 bytes, `__TEXT,__text` at 89,359,004 bytes,
  `__TEXT,__const` sections at 11,153,408 and 3,626,720 bytes, and
  `__LINKEDIT` at 200,884,224 bytes. This removes another 8,368 bytes from the
  debug binary. The OSS lib check and debug OSS binary build pass.
- Short-circuited remaining terminal-input CLI-agent session lookups in OSS.
  The OSS build now returns direct fallback values for rich input visibility,
  bash-mode input state, skill command prefixes, and CLI-agent input rendering
  checks instead of querying the inert session model. Debug `warp-oss` is
  336,860,472 bytes with `__TEXT` at 132,268,032 bytes, `__TEXT,__text` at
  89,357,980 bytes, `__TEXT,__const` sections at 11,153,408 and 3,626,520
  bytes, and `__LINKEDIT` at 200,884,224 bytes. This removes another 1,168
  bytes from the debug binary. The OSS lib check and debug OSS binary build
  pass.
- Replaced the OSS terminal CLI-agent rich-input renderer with inert stubs and
  gated the model-layer CLI-agent input-session restore subscription out of
  OSS. The rich-input renderer can never be reached in OSS, and the input model
  can treat CLI-agent rich input as permanently closed. Debug `warp-oss` is
  336,798,632 bytes with `__TEXT` at 132,235,264 bytes, `__TEXT,__text` at
  89,343,132 bytes, `__TEXT,__const` sections at 11,153,408 and 3,625,928
  bytes, and `__LINKEDIT` at 200,851,456 bytes. This removes another 61,840
  bytes from the debug binary. The OSS lib check and debug OSS binary build
  pass.
- Short-circuited terminal-view agent icon resolution in OSS. Terminal agent
  surfaces are inert in this build, so the terminal icon adapter now returns
  `None` directly instead of walking CLI-agent, ambient-task, and selected
  conversation state. Debug `warp-oss` is 336,786,312 bytes with `__TEXT` at
  132,235,264 bytes, `__TEXT,__text` at 89,337,244 bytes, `__TEXT,__const`
  sections at 11,153,408 and 3,625,856 bytes, and `__LINKEDIT` at 200,835,072
  bytes. This removes another 12,320 bytes from the debug binary. The OSS lib
  check and debug OSS binary build pass.
- Short-circuited vertical-tab terminal agent-title text in OSS. The OSS build
  now returns default plain-terminal tab text instead of resolving ambient
  agent state, selected conversation titles/prompts, or CLI-agent titles for
  terminal tabs. Debug `warp-oss` is 336,784,648 bytes with `__TEXT` at
  132,235,264 bytes, `__TEXT,__text` at 89,335,452 bytes, `__TEXT,__const`
  sections at 11,153,408 and 3,626,320 bytes, and `__LINKEDIT` at 200,835,072
  bytes. This removes another 1,664 bytes from the debug binary. The OSS lib
  check and debug OSS binary build pass.
- Short-circuited remaining vertical-tab terminal agent chrome in OSS. Terminal
  detail badges and sidecar status now use plain terminal fallbacks instead of
  resolving CLI-agent icons, rich agent status, ambient agent state, or selected
  conversation state. Debug `warp-oss` is 336,774,248 bytes with `__TEXT` at
  132,235,264 bytes, `__TEXT,__text` at 89,332,892 bytes, `__TEXT,__const`
  sections at 11,153,408 and 3,625,840 bytes, and `__LINKEDIT` at 200,835,072
  bytes. This removes another 10,400 bytes from the debug binary. The OSS lib
  check and debug OSS binary build pass.
- Gated terminal-view CLI-agent rich-input cleanup on block completion out of
  OSS. OSS never opens CLI-agent rich input or registers CLI-agent sessions, so
  completed user blocks no longer call the rich-input close path or remove an
  inert CLI-agent session. Debug `warp-oss` is 336,757,352 bytes with `__TEXT`
  at 132,218,880 bytes, `__TEXT,__text` at 89,331,868 bytes, `__TEXT,__const`
  sections at 11,153,408 and 3,625,840 bytes, and `__LINKEDIT` at 200,835,072
  bytes. This removes another 16,896 bytes from the debug binary. The OSS lib
  check and debug OSS binary build pass.
- Gated the CLI-agent rich-input action and Ctrl-G binding out of OSS, and made
  the CLI-agent toolbar setting change handler skip rich-input close behavior in
  OSS. The fork never enables the CLI-agent footer context or rich-input
  session, so this removes another inert action path from the OSS terminal
  surface. Debug `warp-oss` is 336,751,048 bytes with `__TEXT` at 132,218,880
  bytes, `__TEXT,__text` at 89,328,796 bytes, `__TEXT,__const` sections at
  11,153,408 and 3,625,832 bytes, and `__LINKEDIT` at 200,818,688 bytes. This
  removes another 6,304 bytes from the debug binary. The OSS lib check and debug
  OSS binary build pass.
- Release checkpoint after the current OSS cuts: `warp-oss` built with the
  release profile is 105,778,696 bytes, with `__TEXT` at 66,158,592 bytes,
  `__TEXT,__text` at 48,876,900 bytes, `__TEXT,__const` sections at 6,736,504
  and 2,073,464 bytes, and `__LINKEDIT` at 37,486,592 bytes. This is below the
  300 MB release-size concern for the executable. The release OSS binary build
  passes.

## Next Cut

Continue shrinking shared git-ui/AI compatibility surfaces. The next promising
area is the broader `crate::ai` dependency and code-review diff/view type
surface that still remains for shared signatures; the direct external
`warp_editor` dependency and the code-review imported-comment implementation are
already out of the OSS build graph.

Iterate with debug-speed validation first:

```sh
cargo check -p warp --lib --target aarch64-apple-darwin \
  --no-default-features \
  --features release_bundle,extern_plist,oss_release,gui
```

After each dependency group, rerun the same `cargo tree` audit and remove any
module declarations that become unused. Reserve `./script/bundle --channel oss
--nosign` for larger checkpoints where the release-size delta is worth the
extra build time.

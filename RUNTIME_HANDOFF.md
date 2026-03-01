# ZeroClaw — Runtime Handoff

## System Overview

ZeroClaw is an autonomous agent runtime (Rust, ~72k LOC). A Next.js chat frontend at `https://ay8.app` proxies requests to the runtime via a Cloudflare Tunnel at `https://gateway.ay8.app` → `localhost:8080`.

```
Browser → ay8.app (Cloudflare Worker)
       → Next.js API routes (server-side, adds Bearer token)
       → gateway.ay8.app (Cloudflare Tunnel)
       → localhost:8080 (ZeroClaw gateway, PairingGuard auth)
       → Agent loop with full tool execution → AI provider
```

The browser never talks to the gateway directly. Auth is bearer tokens via PairingGuard (SHA-256 compare). No Cloudflare Zero Trust / Access — that was abandoned.

---

## Repos and Locations

| Item | Path |
|------|------|
| Runtime | `~/zeroclaw-main` |
| Chat frontend | `~/zeroclaw-chat` |
| Gateway config | `~/.zeroclaw/config.toml` |
| Tunnel config | `~/.cloudflared/config.yml` |
| Tunnel ID | `0e0ff8b1-e91a-4861-a762-5031ad8e71c8` |
| NCB MCP config | `~/.claude.json` |

---

## Current State

Everything is committed, deployed, and working end-to-end.

### zeroclaw-main — latest

The gateway webhook runs the full agent loop with tools instead of raw LLM chat. Native Anthropic tool calling is working end-to-end — MCP tools execute via structured API tool use (not XML fallback). Key files:

- `src/agent/loop_.rs` — `ToolCallRecord` struct, `agent_turn()` and `run_tool_call_loop()` accept optional tool record collection, MCP tools wired into both `run()` and `process_message()`
- `src/gateway/mod.rs` — `agent_turn()` replaces `simple_chat()` in webhook handler, `GET /info` endpoint, `AppState` includes `mcp_manager`, 120s timeout
- `src/gateway/memory_api.rs` — Memory REST API: `POST /memory`, `GET /memory`, `GET /memory/search`, `GET /memory/key/{key}`, `DELETE /memory/key/{key}`, `GET /memory/count`
- `src/gateway/conversations.rs` — Conversation threading: `GET /conversations`, `GET /conversations/{id}`, `DELETE /conversations/{id}`
- `src/channels/mod.rs` — passes `None` for new tool_records param
- `src/providers/anthropic.rs` — `chat_with_tools()` override: converts OpenAI-format tool JSON to Anthropic `NativeToolSpec`, sends via `/v1/messages` with native tool definitions
- `src/providers/reliable.rs` — `supports_native_tools()` and `chat_with_tools()` delegation to inner provider
- `src/mcp/` — **MCP client integration** (see below)

1767 tests pass (55 gateway, 8 conversation, 4 auth, 4 response, 1 parse_category). Pre-existing flaky `memory::lucid` test (timing-dependent, unrelated).

**Gateway API extensions**:
- Webhook body accepts `conversation_id` for multi-turn threading and `agent_id` for delegate routing
- `agent_id` echoed in response JSON — enables per-agent attribution in group chats
- Conversations stored in `conversations.db` (SQLite, WAL mode) in workspace `memory/` dir
- Response envelope: `{"success": true, "data": ...}` or `{"success": false, "error": "..."}`
- OpenAPI 3.1.0 spec at `~/zeroclaw-main/openapi.yaml`

### MCP Client Integration

ZeroClaw can now connect to MCP (Model Context Protocol) servers and expose their tools to the agent. Each MCP tool becomes a first-class ZeroClaw tool named `mcp__<server>__<tool>`.

**Module structure** (`src/mcp/`):

| File | Purpose |
|------|---------|
| `config.rs` | `McpConfig`, `McpServerConfig` — TOML config types |
| `protocol.rs` | JSON-RPC 2.0 types + MCP protocol structs |
| `transport.rs` | `McpTransport` trait + `StdioTransport` (subprocess) + `SseTransport` (HTTP) |
| `client.rs` | `McpClient` — initialize, tools/list, tools/call, resources |
| `bridge.rs` | `McpBridgedTool` (impl `Tool`), `McpListResourcesTool`, `McpReadResourceTool` |
| `mod.rs` | `McpManager::create_mcp_tools()` — public API |

**Design**: Per-tool bridging with zero new dependencies. Disabled by default (`mcp.enabled = false`). Servers that fail to connect are warned and skipped (graceful degradation). Resource-capable servers also get `list_resources` and `read_resource` synthetic tools.

**Hardening**: Auto-restart on crash (`auto_restart = true` default) — `StdioTransport` holds spawn config and respawns on EOF, retries once. Graceful shutdown via `with_graceful_shutdown` on gateway ctrl+c. Health monitoring via `McpManager::health_status()` exposed in `/info` endpoint as `mcp_servers` array.

**Wired into**: gateway (`src/gateway/mod.rs`), CLI agent (`src/agent/loop_.rs` `run()`), channel processing (`src/agent/loop_.rs` `process_message()`), config schema, onboard wizard.

**Native tool calling**: The Anthropic provider's `chat_with_tools()` converts OpenAI-format tool JSON (`{"type":"function","function":{...}}`) to Anthropic's `NativeToolSpec` format and sends them via the `/v1/messages` API. `ReliableProvider` delegates `supports_native_tools()` and `chat_with_tools()` to the inner provider. Without these overrides, the agent loop falls back to prompt-based XML tool injection which doesn't parse correctly.

**Verified**: `mcp__filesystem__list_directory` executes end-to-end via gateway webhook — native structured tool calls, 138ms tool execution, results in `tool_calls` array.

### zeroclaw-chat — latest (`4ad5dec`)

All features deployed at `ay8.app`:

**Core features (from earlier commits):**
- **Token hardened** — `NEXT_PUBLIC_GATEWAY_TOKEN` → `GATEWAY_TOKEN` (server-only)
- **Tool call rendering** — collapsible tool blocks in agent messages (name, success/fail, duration, result)
- **Agent panel** — sidebar shows delegates, tools, runtime channel status via `/api/info`
- **NCB persistence** — awaited writes to NoCodeBackend public data API, history loading on conversation switch
- **Voice input (STT)** — MediaRecorder captures audio, Cloudflare Workers AI Whisper transcribes, auto-sends to agent
- **Voice output (TTS)** — Browser `speechSynthesis` with iOS audio unlock, sentence chunking, auto-speaks agent responses to voice messages
- **Manual TTS** — speaker button on agent messages for on-demand read-aloud
- **Virtualized messages** — `@tanstack/react-virtual` with `useVirtualizer`, dynamic heights, auto-scroll with user-scroll-up detection

**PWA & mobile (`b763234`, `250c1ed`):**
- **PWA install** — `manifest.json`, app icons (192/512/180), `appleWebApp` metadata
- **44px touch targets** — all tappable elements meet iOS/Android minimum
- **Swipe gestures** — `useSwipeGesture` hook: edge-zone detection, follow-finger animation, velocity/distance thresholds
- **Keyboard handling** — `useKeyboardHeight` hook: Visual Viewport API tracks virtual keyboard
- **Safe area insets** — `env(safe-area-inset-*)` for iOS notch/status bar on all fixed/sticky elements

**Conversation threading (`0ad1423`):**
- `conversation_id` forwarded to gateway webhook body for multi-turn threading
- Gateway maintains context across messages in the same conversation

**Polish (`81824b5`):**
- **Model display** — shows which model responded (e.g. "claude-sonnet-4") under agent messages
- **Skeleton loader** — loading states for conversations and messages
- **Scroll memory** — preserves scroll position when switching conversations
- **Notification sound** — plays when agent responds (configurable)
- **Image lightbox** — click-to-zoom on images in messages
- **Archive/restore** — archive conversations, view archived, restore to active

**Group chats & multi-agent (`aa5e12b`):**
- **Conversation types** — `'individual' | 'group'`, stored in NCB `user_email` field (`"zeroclaw"` or `"group:zeroclaw,researcher,coder"`)
- **@mention routing** — `parseAtMentions(content, validAgents)` extracts `@name` mentions, routes to target agents
- **Fan-out** — `useGroupChat` hook sends to multiple agents via `Promise.allSettled()`, per-agent typing indicators
- **agent_id delegation** — webhook body `agent_id` routes to matching gateway delegate (provider, model, system_prompt, temperature)
- **Agent attribution** — NCB `model` field stores `"claude-sonnet-4|researcher"` format; parsed by `parseModelField()`
- **Unified dispatch** — `useChatDispatch` routes to `useChat` (individual) or `useGroupChat` (group)
- **Tabbed multi-chat** — `useTabs` hook: max 5 tabs, first tab cannot close, `assignConversation(tabId, convId)`
- **ChatPane** — extracted reusable chat column, tab bar shown when 2+ tabs

**Agent CRUD & chat management (`4ad5dec`):**
- **Dynamic agents** — `useAgents` hook fetches from NCB, merges gateway delegates as system agents, seeds from `AGENT_PERSONAS` on first load
- **Agent persistence** — agents stored in NCB `agents` table, localStorage cache + deleted set for offline/update support
- **Agent manager** — `AgentManagerDialog` full-screen dialog with search, Your Agents / System Agents sections, create/edit/delete/toggle/duplicate
- **Agent editor** — `AgentEditorDialog` form with icon picker (20 lucide icons), color picker (10 presets), system prompt textarea, advanced section (model override, temperature slider, tools multi-select)
- **ICON_REGISTRY** — 20 icons: bot, search, code, pen, zap, shield, brain, globe, sparkles, heart, cpu, wrench, book-open, music, camera, message-square, rocket, star, flame, eye
- **COLOR_PRESETS** — 10 tailwind colors: accent, emerald, amber, rose, purple, cyan, sky, orange, pink, lime
- **Bulk operations** — selection mode with checkboxes, bulk delete, bulk archive, delete all (with confirmation)
- **Folders** — user-created folder groupings, move conversations to folders, collapsible folder sections in sidebar
- **Tags** — freeform labels per conversation, tag filter chips, add/remove from context menu
- **Export** — Markdown (copy to clipboard), JSON (file download), PDF (browser print) via `ExportDialog`
- **Enhanced context menu** — right-click/long-press with Move to Folder, Add Tag, Export, Open in New Tab submenus

Routes: `/api/chat`, `/api/agents`, `/api/conversations`, `/api/health`, `/api/info`, `/api/messages`, `/api/pair`, `/api/transcribe`.

---

## Project Structure

```
app/api/agents/        — Agent CRUD API (GET/POST)
app/api/chat/          — Proxy to gateway /webhook
app/api/conversations/ — Conversation management (GET/POST/PUT)
app/api/health/        — Gateway health check proxy
app/api/info/          — Runtime info proxy (delegates, tools, channels)
app/api/messages/      — Load persisted message history from NCB
app/api/pair/          — Pairing code exchange proxy
app/api/transcribe/    — Voice STT via Workers AI Whisper

components/            — 19 React UI components
lib/                   — Shared utilities, types, API clients
lib/hooks/             — 13 React hooks
public/                — PWA manifest, icons
.open-next/            — Build output (git-ignored)
wrangler.jsonc         — Cloudflare Worker config
open-next.config.ts    — OpenNext adapter config (minimal)
```

### Components (19)

| Component | Purpose |
|-----------|---------|
| `ChatContainer.tsx` | Root container — wires useAgents, useConversations, useTabs, renders dialogs |
| `ChatPane.tsx` | Reusable chat column (MessageList + MessageInput), used per tab |
| `Sidebar.tsx` | Agent panel, folder sections, tag filters, bulk action bar, manage agents button |
| `MessageList.tsx` | Virtualized message rendering, tool calls, TTS, agent attribution in groups |
| `MessageInput.tsx` | Text input, voice mic, @mention autocomplete from dynamic agents |
| `ConversationList.tsx` | Conversation items grouped by agent, separate group chats section |
| `ConversationItem.tsx` | Single conversation row with selection checkbox, tag chips, context menu |
| `ConversationContextMenu.tsx` | Right-click menu: rename, pin, archive, delete, folder, tag, export, new tab |
| `NewChatButton.tsx` | Agent selector dropdown for new individual chats |
| `NewGroupChatDialog.tsx` | Multi-agent selection for group chat creation |
| `AgentManagerDialog.tsx` | Full-screen agent list with CRUD actions |
| `AgentEditorDialog.tsx` | Agent create/edit form with icon picker, color picker, advanced settings |
| `AgentCard.tsx` | Reusable agent display card (icon, name, toggle, edit, duplicate, delete) |
| `ExportDialog.tsx` | Conversation export: Markdown, JSON, PDF format picker |
| `MarkdownContent.tsx` | Markdown renderer with syntax highlighting |
| `PairingDialog.tsx` | Token pairing UI |
| `RenameDialog.tsx` | Conversation rename dialog |
| `ConfirmDialog.tsx` | Generic confirmation dialog |
| `ThemeProvider.tsx` | Dark theme provider |

### Hooks (13)

| Hook | Purpose |
|------|---------|
| `useAgents.ts` | Agent state: fetch from NCB, merge delegates, seed, CRUD, toggle, reorder, duplicate |
| `useChat.ts` | Individual chat: messages, send, NCB persistence, history loading, auto-title |
| `useGroupChat.ts` | Group chat: @mention parsing, fan-out to agents, per-agent typing, parallel responses |
| `useChatDispatch.ts` | Unified wrapper: routes to useChat or useGroupChat based on conversation type |
| `useConversations.ts` | Conversation state: CRUD, pin, archive, bulk ops, folders, tags, search, filtering |
| `useTabs.ts` | Tab state: max 5, add/remove/activate, assign conversation to tab |
| `useChannels.ts` | Runtime channels from /info |
| `useConnection.ts` | Gateway connection state |
| `useRuntimeInfo.ts` | Fetches delegates, tools, channels, MCP servers from /info |
| `useSpeechRecognition.ts` | MediaRecorder + Workers AI Whisper STT |
| `useSpeechSynthesis.ts` | Browser speechSynthesis TTS, iOS audio unlock, chunking |
| `useSwipeGesture.ts` | Touch sidebar swipe open/close |
| `useKeyboardHeight.ts` | Virtual keyboard height via Visual Viewport API |

### Key Libraries

| File | Purpose |
|------|---------|
| `lib/agents.ts` | ICON_REGISTRY (20 icons), COLOR_PRESETS, getPersona(), injectPersona(), parseAtMentions(), slugify() |
| `lib/api.ts` | Client-side API: sendMessage, fetchConversations, fetchAgents, createAgent, etc. |
| `lib/ncb.ts` | Server-side NCB REST client: conversations, messages, agents, generic CRUD |
| `lib/types.ts` | TypeScript interfaces: Agent, Conversation, ChatMessage, RuntimeInfo, etc. |
| `lib/voice-utils.ts` | sanitizeForSpeech(), chunkText() |
| `lib/export.ts` | exportToClipboard() — markdown export |

---

## Data Model

### Agent type (`lib/types.ts`)

```typescript
interface Agent {
  id: number              // NCB row ID (negative for unseeded/system)
  agentId: string         // unique slug, e.g. "my-writer"
  name: string
  description: string
  icon: string            // key into ICON_REGISTRY
  color: string           // tailwind class, e.g. "text-emerald-400"
  systemPrompt: string
  model?: string          // e.g. "claude-sonnet-4"
  provider?: string       // e.g. "anthropic"
  temperature?: number    // 0.0-2.0
  tools?: string[]        // enabled tool names
  openingMessage?: string
  suggestedQuestions?: string[]
  isSystem: boolean       // true = gateway delegate (read-only)
  isEnabled: boolean
  sortOrder: number
  createdAt: string
  updatedAt: string
}
```

### Conversation type (`lib/types.ts`)

```typescript
interface Conversation {
  id: number
  channel: string
  agentId: string
  title: string
  isPinned: boolean
  isArchived: boolean
  messageCount: number
  lastMessageAt: string | null
  lastMessagePreview: string | null
  createdAt: string
  updatedAt: string
  type: 'individual' | 'group'
  agentIds: string[]      // group: all agents; individual: [agentId]
  folder?: string         // user-created folder (localStorage)
  tags?: string[]         // freeform labels (localStorage)
}
```

### NCB encoding conventions

- **Agent in conversations**: `user_email` field — `"zeroclaw"` for individual, `"group:zeroclaw,researcher,coder"` for group
- **Agent attribution in messages**: `model` field — `"claude-sonnet-4|researcher"` format, parsed by `parseModelField()`
- **Mutable metadata**: NCB public RLS = read + create only (no updates/deletes). Pin, rename, archive, folder, tags stored in localStorage overlay (`zeroclaw_conv_meta`)
- **Agent deletes**: localStorage deleted set (`zeroclaw_deleted_agents`), agent updates cached in `zeroclaw_agents_cache`
- **Folders**: Stored in `zeroclaw_folders` localStorage key

---

## NCB Database

Data API: `https://app.nocodebackend.com/api/data`
Instance: `36905_zeroclaw_chat`
Path format: `/create/<table>`, `/read/<table>`, `/search/<table>` with `?Instance=36905_zeroclaw_chat`

RLS policies set to `public_readwrite` — no session cookies needed. Public RLS only supports read + create (no updates, no deletes).

| Table | Fields | RLS |
|-------|--------|-----|
| `conversations` | `channel`, `user_email`, `title`, `created_at`, `updated_at` | `public_readwrite` |
| `messages` | `conversation_id`, `role`, `content`, `model`, `client_message_id`, `created_at` | `public_readwrite` |
| `agents` | `agent_id`, `name`, `description`, `icon`, `color`, `system_prompt`, `model`, `provider`, `temperature`, `tools`, `opening_message`, `suggested_questions`, `is_system`, `is_enabled`, `sort_order`, `created_at`, `updated_at` | `public_readwrite` |
| `user_sessions` | `email`, `cf_access_sub`, `last_seen`, `created_at` | private (default) |

MCP token (for MCP tools only, NOT for REST API): `ncb_5555d9c08f06607289b6bc7296b228436103afcee5ec30a5`

---

## Config

### Gateway (`~/.zeroclaw/config.toml`)

```toml
[gateway]
port = 8080
host = "127.0.0.1"
require_pairing = true
allow_public_bind = false
paired_tokens = ["zc_local_dev_2026", "78e80f32166e97b07b2814e70e808071f5496276c5dd22261b13976695efaa1f"]
```

### Browser tool (`~/.zeroclaw/config.toml`)

```toml
[browser]
enabled = true
allowed_domains = ["*"]      # "*" = all domains, [] = block all (fails with error)
backend = "agent_browser"    # uses `agent-browser` npm CLI (Playwright + Chromium)
native_headless = true
native_webdriver_url = "http://127.0.0.1:9515"
```

**Setup**: `npm install -g agent-browser && agent-browser install` (downloads Chromium).

Registers two tools: `browser` (full automation: open, snapshot, click, fill, get_text, screenshot, scroll, wait, press, hover, find, close) and `browser_open` (simple URL open). Domain validation: `"*"` allows all, `"*.example.com"` for wildcard subdomain, `"example.com"` for exact + subdomains. SSRF protection blocks all local/private IPs regardless of allowlist.

### HTTP request tool (`~/.zeroclaw/config.toml`)

```toml
[http_request]
enabled = false              # enable for raw HTTP API calls (JSON endpoints)
allowed_domains = []         # same allowlist semantics as browser
max_response_size = 0        # 0 = unlimited
timeout_secs = 0
```

### MCP servers (`~/.zeroclaw/config.toml`)

```toml
[mcp]
enabled = true

[mcp.servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me"]

[mcp.servers.github]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-github"]
env = { GITHUB_TOKEN = "ghp_..." }
```

### Chat frontend (`~/zeroclaw-chat/.env.local`)

```env
NEXT_PUBLIC_GATEWAY_URL=http://localhost:8080
GATEWAY_URL=http://localhost:8080
GATEWAY_TOKEN=zc_local_dev_2026
NCB_API_TOKEN=ncb_5555d9c08f06607289b6bc7296b228436103afcee5ec30a5
```

---

## Commands

```bash
# Runtime (MUST use run.sh for OAuth token)
cd ~/zeroclaw-main && ./run.sh daemon --port 8080

# Tunnel
cloudflared tunnel run zeroclaw-gateway

# Frontend (local)
cd ~/zeroclaw-chat && npm run dev

# Frontend (build)
cd ~/zeroclaw-chat && npx opennextjs-cloudflare build

# Frontend (deploy — MUST use wrangler directly, NOT opennextjs-cloudflare deploy)
cd ~/zeroclaw-chat && npx wrangler deploy

# One-liner (build + deploy)
cd ~/zeroclaw-chat && npx opennextjs-cloudflare build && npx wrangler deploy
```

**Deploy warnings:**
- **DO NOT** use `npx opennextjs-cloudflare deploy` — it has a broken remote proxy session that causes socket errors. Always use `npx wrangler deploy` directly after the build step.
- **esbuild goroutine deadlock** — The OpenNext build sometimes prints `fatal error: all goroutines are asleep - deadlock!` to stderr. This is cosmetic — look for "OpenNext build complete." to confirm success.
- **Port conflict** — Kill stale processes first: `pkill -f wrangler; pkill -f workerd; pkill -f esbuild; sleep 2`

---

## Voice Integration

### Architecture

```
Tap mic → getUserMedia (mic permission) → MediaRecorder.start(1000)
Tap again → MediaRecorder.stop() → audio Blob
  → POST /api/transcribe (FormData with audio file)
  → Cloudflare Workers AI (@cf/openai/whisper)
  → transcribed text → auto-send via onSendMessage()
  → agent responds → TTS auto-speaks response
```

### Key files

| File | Role |
|------|------|
| `lib/hooks/useSpeechRecognition.ts` | MediaRecorder + `/api/transcribe` hook. Accepts `onTranscription` callback for auto-send. |
| `lib/hooks/useSpeechSynthesis.ts` | Browser `speechSynthesis` TTS. iOS audio unlock, voice persistence, sentence chunking. |
| `lib/voice-utils.ts` | `sanitizeForSpeech()` strips markdown. `chunkText()` splits at sentence boundaries (~200 chars). |
| `lib/hooks/useSwipeGesture.ts` | Touch event handlers for sidebar swipe open/close. Edge zone (30px), velocity + distance thresholds, passive listeners. |
| `lib/hooks/useKeyboardHeight.ts` | Visual Viewport API. Returns keyboard height in px for layout adjustment. |
| `public/manifest.json` | PWA manifest — standalone display, dark theme, app icons. |
| `public/icons/` | `icon-192.png`, `icon-512.png`, `apple-touch-icon.png`, `icon.svg` (source). |
| `app/api/transcribe/route.ts` | Receives audio FormData, runs `env.AI.run('@cf/openai/whisper', ...)`. |
| `wrangler.jsonc` | `"ai": { "binding": "AI" }` for Workers AI access. |

### Why not Web Speech API SpeechRecognition

Safari's `webkitSpeechRecognition` requires an OS-level "Speech Recognition" permission (System Preferences > Privacy & Security > Speech Recognition) — separate from mic permission. Users get `not-allowed` errors even with mic granted. `getUserMedia` + `MediaRecorder` only needs mic permission, which Safari handles reliably.

### Safari-specific

- `MediaRecorder.start(1000)` — timeslice required or Safari produces blobs with broken metadata
- iOS audio unlock: play silent MP3 + empty `SpeechSynthesisUtterance` at volume 0 during user gesture
- TTS chunking at ~200 char sentence boundaries to avoid Safari's ~15s playback cutoff
- Audio format detection: Safari uses `audio/mp4`, Chrome uses `audio/webm;codecs=opus`

### Bindings

- `env.AI` — Cloudflare Workers AI binding (no API key needed, declared in `wrangler.jsonc`)
- Access via `getCloudflareContext()` from `@opennextjs/cloudflare`

---

## Gateway Registered Tools

Currently active tools (from `/info` endpoint):

| Tool | Source | Notes |
|------|--------|-------|
| `shell` | Built-in | 60s timeout, 1MB output cap, sandboxed to workspace |
| `file_read`, `file_write` | Built-in | Sandboxed to workspace, 10MB limit |
| `cron_add/list/remove/update/run/runs` | Built-in | Scheduled jobs (shell or agent type) |
| `memory_store/recall/forget` | Built-in | Persistent key-value memory (SQLite) |
| `schedule` | Built-in | Create/list/cancel scheduled tasks |
| `git_operations` | Built-in | Git within workspace |
| `browser` | `agent_browser` CLI | Full web automation: open, click, type, snapshot, get_text, screenshot |
| `browser_open` | `agent_browser` CLI | Simple URL open |
| `screenshot`, `image_info` | Built-in | Screen capture, image metadata |
| `pushover` | Built-in | Push notifications |
| `mcp__filesystem__*` | MCP server | File operations via `@modelcontextprotocol/server-filesystem` |

Conditionally available (disabled in config): `http_request`, `composio`, `delegate`, `gpio_*`, `arduino_*`, `hardware_*`.

---

## PWA Safe Area Rules

All UI changes MUST follow these rules for iOS notch/status bar compatibility:

- Fixed/sticky headers: `pt-[env(safe-area-inset-top)]`
- Fixed/sticky footers: `pb-[max(0.5rem,env(safe-area-inset-bottom))]`
- Left-aligned fixed panels: `pl-[env(safe-area-inset-left)]`
- Right-edge content: `pr-[max(0.75rem,env(safe-area-inset-right))]`
- Never use bare `100vh` — always `100dvh`
- All interactive elements: `min-h-[44px] min-w-[44px]`

---

## Rules

- `GATEWAY_TOKEN` is server-only. Never use `NEXT_PUBLIC_` prefix for tokens.
- NCB failures never block chat. Writes are awaited but wrapped in try/catch.
- **Must await NCB writes** — Cloudflare Workers kill async work after response is sent. Fire-and-forget (`promise.then().catch()`) does NOT work.
- **Build**: `npx opennextjs-cloudflare build`. **Deploy**: `npx wrangler deploy`. Do NOT use `npx opennextjs-cloudflare deploy` (broken socket).
- Do not add `export const runtime = 'edge'` to routes — OpenNext handles the runtime.
- Structured JSON responses, not SSE streaming. Tool calls return after the agent loop completes.
- **Always use `run.sh`** to start the gateway — it extracts the Claude Code OAuth token from macOS Keychain (`security find-generic-password -s "Claude Code-credentials"`) and exports it as `ANTHROPIC_OAUTH_TOKEN`. Running `cargo run` directly will fail with "Anthropic credentials not set".
- NCB data API paths: `/create/<table>`, `/read/<table>` etc. Always include `?Instance=36905_zeroclaw_chat`.
- NCB public RLS = read + create only. Updates and deletes are managed client-side via localStorage overlay.
- **Do not use browser `SpeechRecognition` API** — use `MediaRecorder` + Workers AI Whisper instead (Safari compatibility).
- Voice reference implementation: `~/aismb` repo (`github.com/elev8tion/aismb`) — VoiceOperator component.

---

## Commit History (zeroclaw-chat)

| Commit | Description |
|--------|-------------|
| `4ad5dec` | Agent CRUD, dynamic agents, chat management (bulk ops, folders, tags, export) |
| `250c1ed` | PWA safe area insets for iOS notch/status bar overlap |
| `aa5e12b` | Group chats, @mention routing, agent_id delegation, tabbed multi-chat |
| `81824b5` | Polish: model display, skeleton loader, scroll memory, notification sound, image lightbox, archive/restore |
| `0ad1423` | Forward conversation_id to gateway webhook for multi-turn threading |
| `c937c8e` | Docs: runtime handoff update with browser tool, mobile UX, gateway tools |
| `b763234` | Mobile UX: PWA install, touch targets, swipe gestures, virtualized messages |

---

## Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `next` | 16.1.6 | Framework |
| `@opennextjs/cloudflare` | ^1.17.0 | Cloudflare Workers adapter |
| `tailwindcss` | ^4 | Styling |
| `lucide-react` | ^0.575.0 | Icons (20 in ICON_REGISTRY) |
| `@tanstack/react-virtual` | ^3.13.18 | Message list virtualization |
| `react-markdown` | ^10.1.0 | Markdown rendering |
| `date-fns` | ^4.1.0 | Date formatting |

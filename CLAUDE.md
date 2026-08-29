# CLAUDE.md — project facts for coding agents

Read this before touching anything. It records the decisions an agent is most likely to get wrong
because older files, older releases, and stale memory say otherwise.

## What this project is

**Aizen** — a terminal-native agentic coding CLI. Pure Rust, shipped as **one static binary**, points
at any OpenAI-compatible `/chat/completions` endpoint. The command is `aizen`.

Hard constraints (a change that breaks one of these will be rejected):

- **Single static binary.** No C/native dependency, no external runtime. TLS is rustls-only — never
  reintroduce OpenSSL or anything needing a native toolchain.
- **Pure Rust.** If a crate pulls a C build, it doesn't go in.
- Startup time and binary size are features. Measured 2026-08-02: **10.8 ms** startup, **34.1 MB**
  binary. This is the project's main advantage over Claude Code (needs Node) and Hermes Agent
  (needs Python + ~2.4 GB) — do not regress it casually.

## License — Apache-2.0 (changed 2026-08-03)

**The project is Apache-2.0. It is NOT PolyForm Noncommercial anymore.** It is open source, and
commercial use is allowed.

- `LICENSE` is the verbatim Apache License 2.0 — the TERMS AND CONDITIONS section is a byte-exact
  copy of <https://www.apache.org/licenses/LICENSE-2.0.txt>. **Never reword, reformat, or "tidy" it**;
  GitHub's license detection and every corporate legal review depend on it matching exactly. Only the
  APPENDIX carries the filled-in `Copyright 2026 Aizen authors`.
- `Cargo.toml` uses `license = "Apache-2.0"` (the SPDX id), **not** `license-file`. Using
  `license-file` makes crates.io and GitHub report an unrecognized license.
- `NOTICE` must ship with any redistribution (Apache §4d). Keep it in sync when attribution changes.
- **There is no CLA.** Contributions come in under **Apache-2.0 §5** (inbound=outbound) plus a DCO
  sign-off checked by `.github/workflows/dco.yml`. `CLA.md` and `.github/workflows/cla.yml` were
  deleted on purpose — **do not restore them.**
- Trademark: Apache §6 grants no rights to the "Aizen" name or logo. Forks may use the code, not the
  brand.
- Releases **up to and including v0.5.5** went out under PolyForm Noncommercial. That is history and
  cannot be retracted; everything from the relicensing commit onward is Apache-2.0. If you find a
  file still saying PolyForm, it is a leftover — fix it.

## Git remotes — two of them, don't mix them up

```
origin  → https://github.com/dawnofcd/Aizen_agent.git   (PRIVATE — full source, day-to-day work)
public  → https://github.com/dawnofcd/aizen.git         (PUBLIC — redirects to rivyn-llc/aizen)
```

- The **canonical public repo is `rivyn-llc/aizen`** (an org). `dawnofcd/aizen` still resolves via
  GitHub's redirect, but **write `rivyn-llc/aizen` in all user-facing URLs, install scripts, and
  code** so nothing depends on a redirect.
- `dawnofcd/Aizen_agent` is private, so an anonymous fetch of it returns 404. That is expected — it
  is not a broken URL. **Never put it in user-facing docs**; the README used to tell people to
  `cargo install --git .../Aizen_agent`, which 404'd for everyone.
- Release binaries are published to `rivyn-llc/aizen`. `src/features/update.rs` has
  `DEFAULT_REPO = "rivyn-llc/aizen"` and `aizen update` reads releases from there — keep it aligned
  with `install.ps1` (`$Repo`) and `install.sh` (`repo=`).
- Never push to `main` on either remote without being asked. Branch, then push with `-u`.

## Layout worth knowing

| Path | What |
|---|---|
| `README.md` | the short landing page — install, why, what it does. **Keep it under ~110 lines**; details go to `docs/REFERENCE.md`, not here |
| `docs/REFERENCE.md` | the full manual: REPL surface, every command, self-hosting, MCP, browser, safety model |
| `src/features/update.rs` | self-update; `DEFAULT_REPO` lives here |
| `install.ps1` / `install.sh` | one-line installers; repo slug is hardcoded in both |
| `.github/workflows/dco.yml` | DCO sign-off check (replaced the CLA bot) |
| `dist/` | assets for the public download channel |
| `docs/` | design + audit notes |
| `bench-fixtures/` | fixtures for `aizen bench` |

Links inside `docs/REFERENCE.md` that point at repo-root paths need a `../` prefix — it lives one
level down.

## Build / verify

```bash
cargo check                                   # fast feedback
cargo build --release --bin aizen             # the shipped artifact
cargo test --bin aizen                        # must be green before pushing
cargo build --release --features dense --bin aizen   # semantic-retrieval tier (feature-gated)
cargo fmt && cargo clippy
```

Note: `cargo test` on Windows can be slow; the 120 s shell cap may kill it. Run long builds through a
background process, not a foreground shell call.

## Known distribution gaps (as of 2026-08-03)

Real numbers, not guesses — from the GitHub API:

- `rivyn-llc/aizen`: 25 stars, 4 forks, **0 watchers**, Discussions **off**.
- v0.5.5 downloads: Windows 3, Linux 0, macOS 0.
- Windows `.exe` is **unsigned** → SmartScreen warns. macOS is **not notarized**.
- Not published to winget / scoop / Homebrew / crates.io / AUR. Apache-2.0 now unblocks the OSI-only
  ones (crates.io, AUR, Homebrew).
- The landing page (`aizen-stack.vercel.app`) advertised v0.4.5 while v0.5.5 was current — it is a
  separate deploy and drifts; check it when releasing.

## Working style the maintainer expects

- Vietnamese for conversation; English for code, comments, and docs.
- Evidence over assumption: measure, cite the tool output, and say plainly when something is
  unverified. Don't claim a command passed unless you ran it.
- Say what is weak about this project honestly — no sales pitch about our own tool.
- Business/licensing decisions are the maintainer's, not the agent's. Ask before changing one.

<!-- rtk-instructions v2 -->
# RTK (Rust Token Killer) - Token-Optimized Commands

## Golden Rule

**Always prefix commands with `rtk`**. If RTK has a dedicated filter, it uses it. If not, it passes through unchanged. This means RTK is always safe to use.

**Important**: Even in command chains with `&&`, use `rtk`:
```bash
# ❌ Wrong
git add . && git commit -m "msg" && git push

# ✅ Correct
rtk git add . && rtk git commit -m "msg" && rtk git push
```

## RTK Commands by Workflow

### Build & Compile (80-90% savings)
```bash
rtk cargo build         # Cargo build output
rtk cargo check         # Cargo check output
rtk cargo clippy        # Clippy warnings grouped by file (80%)
rtk tsc                 # TypeScript errors grouped by file/code (83%)
rtk lint                # ESLint/Biome violations grouped (84%)
rtk prettier --check    # Files needing format only (70%)
rtk next build          # Next.js build with route metrics (87%)
```

### Test (60-99% savings)
```bash
rtk cargo test          # Cargo test failures only (90%)
rtk go test             # Go test failures only (90%)
rtk jest                # Jest failures only (99.5%)
rtk vitest              # Vitest failures only (99.5%)
rtk playwright test     # Playwright failures only (94%)
rtk pytest              # Python test failures only (90%)
rtk rake test           # Ruby test failures only (90%)
rtk rspec               # RSpec test failures only (60%)
rtk test <cmd>          # Generic test wrapper - failures only
```

### Git (59-80% savings)
```bash
rtk git status          # Compact status
rtk git log             # Compact log (works with all git flags)
rtk git diff            # Compact diff (80%)
rtk git show            # Compact show (80%)
rtk git add             # Ultra-compact confirmations (59%)
rtk git commit          # Ultra-compact confirmations (59%)
rtk git push            # Ultra-compact confirmations
rtk git pull            # Ultra-compact confirmations
rtk git branch          # Compact branch list
rtk git fetch           # Compact fetch
rtk git stash           # Compact stash
rtk git worktree        # Compact worktree
```

Note: Git passthrough works for ALL subcommands, even those not explicitly listed.

### GitHub (26-87% savings)
```bash
rtk gh pr view <num>    # Compact PR view (87%)
rtk gh pr checks        # Compact PR checks (79%)
rtk gh run list         # Compact workflow runs (82%)
rtk gh issue list       # Compact issue list (80%)
rtk gh api              # Compact API responses (26%)
```

### JavaScript/TypeScript Tooling (70-90% savings)
```bash
rtk pnpm list           # Compact dependency tree (70%)
rtk pnpm outdated       # Compact outdated packages (80%)
rtk pnpm install        # Compact install output (90%)
rtk npm run <script>    # Compact npm script output
rtk npx <cmd>           # Compact npx command output
rtk prisma              # Prisma without ASCII art (88%)
rtk uv run <cmd>        # Compact uv project command output
```

### Files & Search (60-75% savings)
```bash
rtk ls <path>           # Tree format, compact (65%)
rtk read <file>         # Code reading with filtering (60%)
rtk grep <pattern>      # Search grouped by file (75%). Format flags (-c, -l, -L, -o, -Z) run raw.
rtk find <pattern>      # Find grouped by directory (70%)
```

### Analysis & Debug (70-90% savings)
```bash
rtk err <cmd>           # Filter errors only from any command
rtk log <file>          # Deduplicated logs with counts
rtk json <file>         # JSON structure without values
rtk deps                # Dependency overview
rtk env                 # Environment variables compact
rtk summary <cmd>       # Smart summary of command output
rtk diff                # Ultra-compact diffs
```

### Infrastructure (85% savings)
```bash
rtk docker ps           # Compact container list
rtk docker images       # Compact image list
rtk docker logs <c>     # Deduplicated logs
rtk kubectl get         # Compact resource list
rtk kubectl logs        # Deduplicated pod logs
```

### Network (65-70% savings)
```bash
rtk curl <url>          # Compact HTTP responses (70%)
rtk wget <url>          # Compact download output (65%)
```

### Meta Commands
```bash
rtk gain                # View token savings statistics
rtk gain --history      # View command history with savings
rtk discover            # Analyze Claude Code sessions for missed RTK usage
rtk proxy <cmd>         # Run command without filtering (for debugging)
rtk init                # Add RTK instructions to CLAUDE.md
rtk init --global       # Add RTK to ~/.claude/CLAUDE.md
```

## Token Savings Overview

| Category | Commands | Typical Savings |
|----------|----------|-----------------|
| Tests | vitest, playwright, cargo test | 90-99% |
| Build | next, tsc, lint, prettier | 70-87% |
| Git | status, log, diff, add, commit | 59-80% |
| GitHub | gh pr, gh run, gh issue | 26-87% |
| Package Managers | pnpm, npm, npx | 70-90% |
| Files | ls, read, grep, find | 60-75% |
| Infrastructure | docker, kubectl | 85% |
| Network | curl, wget | 65-70% |

Overall average: **60-90% token reduction** on common development operations.
<!-- /rtk-instructions -->
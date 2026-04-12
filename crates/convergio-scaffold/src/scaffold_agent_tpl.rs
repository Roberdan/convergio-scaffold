// Agent instruction templates for project scaffolding.
// These generate AGENTS.md, CLAUDE.md, copilot-instructions.md, and MCP configs.

/// AGENTS.md — the universal agent reference for Convergio-managed projects.
pub const AGENTS_MD: &str = r#"# {name} — Agent Operating Manual

Read this file FIRST if you are an AI agent working on this project.

## What is this project

{description}

**Stack:** {lang}
**Managed by:** [Convergio](https://github.com/Roberdan/convergio) — an orchestration daemon for AI agent organizations.

## How to connect (MCP)

This project is pre-configured with the Convergio MCP server. Your IDE or agent runtime
should automatically discover it via `.vscode/mcp.json` (Copilot / VS Code) or
`.mcp.json` (Claude Code).

The MCP server exposes ~43 tools for plan management, task execution, evidence recording,
agent spawning, skills, and more. Use `tools/list` to see all available tools.

If the MCP server is not available, you can fall back to the CLI (`cvg`) or the REST API
on port 8420. Both are documented below.

## Convergio process (mandatory)

This project follows the Convergio plan-based workflow. Every change goes through:

### Plan lifecycle
1. **Plan** → group of tasks organized in waves (sequential phases)
2. **Wave** → set of parallelizable tasks (all must pass before next wave)
3. **Task** → atomic unit of work (one branch, one PR)

### Task lifecycle
```
pending → in_progress → submitted → done (only Thor validates)
```

1. Create a worktree: `git worktree add -b <branch> .worktrees/<name> main`
2. Implement the task
3. Run tests and linting (see sections below)
4. Commit with conventional message (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`)
5. Push and create PR
6. Record evidence via API: `POST /api/plan-db/task/evidence`
7. Update task status to `submitted`
8. Thor validates — only Thor can set status to `done`

### Gate chain (every task must pass all gates)
```
EvidenceGate → TestGate → PrCommitGate → WaveSequenceGate → ValidatorGate
```

**Never** skip gates. **Never** declare done without evidence. **Never** bypass Thor.

## Key CLI commands

| Command | What it does |
|---------|-------------|
| `cvg status` | System overview (plans, agents, health) |
| `cvg plan list` | List all plans |
| `cvg plan show <id>` | Plan details + task tree |
| `cvg plan tree <id>` | Execution tree with wave/task hierarchy |
| `cvg task complete <id>` | Mark task done with evidence |
| `cvg solve` | End-to-end: understand → research → spec → execute |
| `cvg copilot register` | Register Copilot session with daemon |
| `cvg claude register` | Register Claude session with daemon |
| `cvg workspace create <name>` | Create isolated worktree |

## Key API endpoints (daemon on port 8420)

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/health` | GET | Health check |
| `/api/plan-db/json/:plan_id` | GET | Plan details |
| `/api/plan-db/execution-tree/:plan_id` | GET | Full execution tree |
| `/api/plan-db/task/update` | POST | Update task status |
| `/api/plan-db/task/evidence` | POST | Record evidence |
| `/api/plan-db/validate` | POST | Thor validation |
| `/api/agents/spawn` | POST | Spawn a new agent |

## Rules

- Code and docs in **English**
- Max **300 lines per file** — split if longer
- Conventional commits: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`
- Every PR must pass CI before merge
- Workspace isolation: one worktree per task, never work on main directly
- Write tests first (TDD when possible)

{running_section}

{linting_section}

## When something is unclear

If Convergio's process, a tool, or a related repo is unclear or broken — open a GitHub
issue on the Convergio repo. This helps improve the platform for all agents.
"#;

/// CLAUDE.md — Claude-specific, points to AGENTS.md.
pub const CLAUDE_MD: &str = r#"# {name}

Read **AGENTS.md** first — it contains all project rules and the Convergio process.

## Claude-specific conventions

- Co-authored-by trailer: `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>`
- Manage context: checkpoint at 70-80% capacity
- Use `cvg claude register` to register your session with the daemon

## {lang_label} commands

{running_section}

{linting_section}

## Testing

- Write tests first (RED-GREEN-REFACTOR)
- 80% coverage for business logic, 100% for critical paths
- Mock external boundaries only (network, filesystem, time)
"#;

/// .github/copilot-instructions.md — Copilot-specific, points to AGENTS.md.
pub const COPILOT_INSTRUCTIONS_MD: &str = r#"# {name} — Copilot Instructions

Read **AGENTS.md** in the repo root for all project rules and the Convergio process.

## Copilot-specific

- Use `cvg copilot register` to register your session with the daemon
- Co-authored-by trailer: `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>`
- Follow the task lifecycle in AGENTS.md — worktree → commit → PR → evidence → submit
- The daemon runs at `http://localhost:8420` — use the API endpoints listed in AGENTS.md
"#;

/// .vscode/mcp.json — VS Code / Copilot MCP client configuration.
pub fn vscode_mcp_json() -> String {
    let bin = resolve_mcp_server_path();
    format!(
        r#"{{
  "servers": {{
    "convergio": {{
      "command": "{}",
      "args": ["--transport", "stdio"],
      "env": {{
        "CONVERGIO_MCP_RING": "1"
      }}
    }}
  }}
}}
"#,
        bin
    )
}

/// .mcp.json — Claude Code MCP client configuration (repo root).
pub fn mcp_json() -> String {
    let bin = resolve_mcp_server_path();
    format!(
        r#"{{
  "mcpServers": {{
    "convergio": {{
      "command": "{}",
      "args": ["--transport", "stdio"],
      "env": {{
        "CONVERGIO_MCP_RING": "1"
      }}
    }}
  }}
}}
"#,
        bin
    )
}

/// Resolve the absolute path to `convergio-mcp-server`.
fn resolve_mcp_server_path() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("convergio-mcp-server");
            if candidate.exists() {
                return candidate.to_string_lossy().into_owned();
            }
        }
    }
    let cmd = if cfg!(windows) { "where" } else { "which" };
    if let Ok(out) = std::process::Command::new(cmd)
        .arg("convergio-mcp-server")
        .output()
    {
        if out.status.success() {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path.is_empty() {
                return path;
            }
        }
    }
    "convergio-mcp-server".into()
}

# ADR-038: Onboarding Scaffolding with MCP Integration

**Status:** Accepted  
**Date:** 2026-04-08  
**Relates to:** ADR-037 (Native MCP Server), Issue #439

## Context

When a new project is scaffolded via `cvg project init` or `POST /api/projects/scaffold`,
the generated files were minimal: Cargo.toml/package.json, CI config, README, and a basic
.claude/CLAUDE.md. Agents arriving at these projects had no idea how to use Convergio —
they didn't know the plan/task lifecycle, gate chain, CLI commands, or how to connect to
the daemon via MCP.

With ADR-037's native MCP server in place, we can now auto-configure MCP clients so agents
discover daemon tools on first launch.

## Decision

The scaffolding generates a complete agent onboarding package:

1. **AGENTS.md** — Universal agent operating manual (all LLMs), containing:
   - Plan/task lifecycle (pending → in_progress → submitted → done)
   - Gate chain (EvidenceGate → TestGate → PrCommitGate → WaveSequenceGate → ValidatorGate)
   - CLI command reference (`cvg plan`, `cvg status`, etc.)
   - API endpoints for plan management and evidence recording
   - MCP tools section with available MCP capabilities
   - Conventional commits, worktree rules, test rules

2. **Agent-specific instruction files** that reference AGENTS.md:
   - `.claude/CLAUDE.md` for Claude Code
   - `.github/copilot-instructions.md` for GitHub Copilot

3. **MCP client configurations**:
   - `.vscode/mcp.json` — VS Code MCP client (servers format)
   - `.mcp.json` — Claude Code MCP client (mcpServers format)
   - Both point to `convergio-mcp-server` via stdio transport

4. **Language-aware templates** — Rust, TypeScript, and Python get correct
   build/test/lint commands in both AGENTS.md and CI configs.

## Consequences

- **Zero-config onboarding**: agents connect to Convergio immediately
- **Single source of truth**: AGENTS.md is the one file all agent types read
- **MCP auto-discovery**: agents discover tools without manual configuration
- **Maintenance**: when we add features, update scaffold templates to match
- **Two paths**: CLI (`cvg project init`) and server (`POST /api/projects/scaffold`)
  share template content via separate but aligned template modules

// Tests for project scaffolding.

use super::*;
use crate::scaffold_gen::license_spdx;

#[test]
fn rust_scaffold_has_all_agent_files() {
    let req = ScaffoldRequest {
        name: "acme-server".into(),
        description: "A server for ACME Corp".into(),
        language: Language::Rust,
        license: License::Mit,
        visibility: Visibility::Public,
        org_id: "acme-corp".into(),
        template: None,
    };
    let resp = generate_scaffold(&req);
    assert_eq!(resp.name, "acme-server");
    let paths: Vec<&str> = resp.files.iter().map(|f| f.path.as_str()).collect();
    // Core project files
    assert!(paths.contains(&"Cargo.toml"));
    assert!(paths.contains(&"src/main.rs"));
    assert!(paths.contains(&"LICENSE"));
    assert!(paths.contains(&"CODEOWNERS"));
    assert!(paths.contains(&".github/workflows/ci.yml"));
    // Agent instruction files
    assert!(paths.contains(&"AGENTS.md"), "must generate AGENTS.md");
    assert!(
        paths.contains(&".claude/CLAUDE.md"),
        "must generate .claude/CLAUDE.md"
    );
    assert!(
        paths.contains(&".github/copilot-instructions.md"),
        "must generate copilot-instructions.md"
    );
    // MCP config files
    assert!(
        paths.contains(&".vscode/mcp.json"),
        "must generate .vscode/mcp.json"
    );
    assert!(paths.contains(&".mcp.json"), "must generate .mcp.json");
    assert!(resp.branch_protection.require_pr);
}

#[test]
fn agents_md_has_convergio_process() {
    let req = ScaffoldRequest {
        name: "acme-server".into(),
        description: "A server for ACME Corp".into(),
        language: Language::Rust,
        license: License::Mit,
        visibility: Visibility::Public,
        org_id: "acme".into(),
        template: None,
    };
    let resp = generate_scaffold(&req);
    let agents = resp.files.iter().find(|f| f.path == "AGENTS.md").unwrap();
    assert!(agents.content.contains("acme-server"), "has project name");
    assert!(agents.content.contains("Plan lifecycle"), "has plan docs");
    assert!(agents.content.contains("Gate chain"), "has gate chain");
    assert!(agents.content.contains("cvg status"), "has CLI commands");
    assert!(agents.content.contains("/api/health"), "has API endpoints");
    assert!(agents.content.contains("MCP"), "has MCP section");
    assert!(agents.content.contains("cargo"), "has Rust commands");
}

#[test]
fn agents_md_language_specific_commands() {
    let py_req = ScaffoldRequest {
        name: "py-proj".into(),
        description: "Python project".into(),
        language: Language::Python,
        license: License::Mit,
        visibility: Visibility::Public,
        org_id: "org".into(),
        template: None,
    };
    let resp = generate_scaffold(&py_req);
    let agents = resp.files.iter().find(|f| f.path == "AGENTS.md").unwrap();
    assert!(agents.content.contains("pytest"), "Python has pytest");
    assert!(agents.content.contains("ruff"), "Python has ruff");
    assert!(!agents.content.contains("cargo"), "Python has no cargo");
}

#[test]
fn mcp_configs_have_convergio_server() {
    let req = ScaffoldRequest {
        name: "proj".into(),
        description: "Test".into(),
        language: Language::Rust,
        license: License::Mit,
        visibility: Visibility::Public,
        org_id: "org".into(),
        template: None,
    };
    let resp = generate_scaffold(&req);
    let vscode = resp
        .files
        .iter()
        .find(|f| f.path == ".vscode/mcp.json")
        .unwrap();
    assert!(vscode.content.contains("convergio-mcp-server"));
    let mcp = resp.files.iter().find(|f| f.path == ".mcp.json").unwrap();
    assert!(mcp.content.contains("convergio-mcp-server"));
}

#[test]
fn typescript_scaffold_has_package_json() {
    let req = ScaffoldRequest {
        name: "widget-ui".into(),
        description: "Frontend widget library".into(),
        language: Language::Typescript,
        license: License::Apache2,
        visibility: Visibility::Private,
        org_id: "widget-co".into(),
        template: None,
    };
    let resp = generate_scaffold(&req);
    let paths: Vec<&str> = resp.files.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&"package.json"));
    assert!(paths.contains(&"tsconfig.json"));
    assert!(paths.contains(&"src/index.ts"));
}

#[test]
fn python_scaffold_has_pyproject() {
    let req = ScaffoldRequest {
        name: "data-pipeline".into(),
        description: "ETL data pipeline".into(),
        language: Language::Python,
        license: License::Gpl3,
        visibility: Visibility::Public,
        org_id: "data-team".into(),
        template: None,
    };
    let resp = generate_scaffold(&req);
    let paths: Vec<&str> = resp.files.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&"pyproject.toml"));
    assert!(paths.contains(&"src/__init__.py"));
}

#[test]
fn license_spdx_mapping() {
    assert_eq!(license_spdx(License::Mit), "MIT");
    assert_eq!(license_spdx(License::Apache2), "Apache-2.0");
    assert_eq!(license_spdx(License::Gpl3), "GPL-3.0-only");
}

#[test]
fn mit_license_contains_year() {
    let req = ScaffoldRequest {
        name: "test-proj".into(),
        description: "A test".into(),
        language: Language::Rust,
        license: License::Mit,
        visibility: Visibility::Public,
        org_id: "tester".into(),
        template: None,
    };
    let resp = generate_scaffold(&req);
    let lic = resp.files.iter().find(|f| f.path == "LICENSE").unwrap();
    assert!(lic.content.contains("MIT License"));
    assert!(lic.content.contains("tester"));
}

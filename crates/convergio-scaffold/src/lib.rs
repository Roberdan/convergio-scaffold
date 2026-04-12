// Project scaffolding — generates file trees for new projects.
//
// POST /api/projects/scaffold: returns JSON with generated files.
// Does NOT create the repo — the CLI handles that via `gh`.

pub mod scaffold_agent_tpl;
pub mod scaffold_gen;
pub mod scaffold_templates;

use std::sync::Arc;

use axum::extract::State;
use axum::response::Json;
use axum::routing::post;
use axum::Router;
use serde::{Deserialize, Serialize};

/// Shared state for scaffold routes.
pub struct ScaffoldState {}

/// Build the scaffold API router.
pub fn scaffold_routes() -> Router {
    let state = Arc::new(ScaffoldState {});
    Router::new()
        .route("/api/projects/scaffold", post(handle_scaffold))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
pub struct ScaffoldRequest {
    pub name: String,
    pub description: String,
    pub language: Language,
    pub license: License,
    pub visibility: Visibility,
    pub org_id: String,
    #[serde(default)]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Typescript,
    Python,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum License {
    Mit,
    Apache2,
    Gpl3,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScaffoldResponse {
    pub name: String,
    pub files: Vec<ScaffoldFile>,
    pub ci_config: String,
    pub branch_protection: BranchProtection,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScaffoldFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BranchProtection {
    pub branch: String,
    pub require_pr: bool,
    pub required_checks: Vec<String>,
    pub dismiss_stale_reviews: bool,
}

async fn handle_scaffold(
    State(_state): State<Arc<ScaffoldState>>,
    Json(req): Json<ScaffoldRequest>,
) -> Json<serde_json::Value> {
    if let Err(e) = validate_request(&req) {
        return e;
    }

    let resp = generate_scaffold(&req);
    match serde_json::to_value(resp) {
        Ok(v) => Json(v),
        Err(_) => error_response("INTERNAL_ERROR", "failed to serialize response"),
    }
}

fn validate_request(req: &ScaffoldRequest) -> Result<(), Json<serde_json::Value>> {
    if req.name.is_empty() || req.name.len() > 64 {
        return Err(error_response("NAME_INVALID", "name must be 1-64 chars"));
    }
    if !req.name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(error_response(
            "NAME_INVALID",
            "alphanumeric and hyphens only",
        ));
    }
    if req.description.is_empty() || req.description.len() > 256 {
        return Err(error_response(
            "DESCRIPTION_INVALID",
            "description must be 1-256 chars",
        ));
    }
    if req.org_id.is_empty() || req.org_id.len() > 64 {
        return Err(error_response(
            "ORG_ID_INVALID",
            "org_id must be 1-64 chars",
        ));
    }
    if !req
        .org_id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(error_response(
            "ORG_ID_INVALID",
            "org_id: alphanumeric, hyphens, and underscores only",
        ));
    }
    Ok(())
}

pub(crate) fn generate_scaffold(req: &ScaffoldRequest) -> ScaffoldResponse {
    let year = chrono::Utc::now().format("%Y").to_string();
    let spdx = scaffold_gen::license_spdx(req.license);
    let owner = &req.org_id;
    let vars = scaffold_gen::Vars {
        name: &req.name,
        description: &req.description,
        license_spdx: spdx,
        year: &year,
        author: owner,
        owner,
        language: req.language,
    };

    let mut files = scaffold_gen::common_files(&vars, req.license);
    let ci = match req.language {
        Language::Rust => scaffold_gen::append_rust(&mut files, &vars),
        Language::Typescript => scaffold_gen::append_typescript(&mut files, &vars),
        Language::Python => scaffold_gen::append_python(&mut files, &vars),
    };

    ScaffoldResponse {
        name: req.name.clone(),
        files,
        ci_config: ci,
        branch_protection: BranchProtection {
            branch: "main".into(),
            require_pr: true,
            required_checks: vec!["check".into()],
            dismiss_stale_reviews: true,
        },
    }
}

fn error_response(code: &str, message: &str) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "error": { "code": code, "message": message }
    }))
}

#[cfg(test)]
#[path = "scaffold_tests.rs"]
mod tests;

use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn test_lock() -> MutexGuard<'static, ()> {
    TEST_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

struct FileGuard {
    path: std::path::PathBuf,
    original: Option<Vec<u8>>,
}

impl FileGuard {
    fn new(path: impl Into<std::path::PathBuf>) -> Self {
        let path = path.into();
        let original = std::fs::read(&path).ok();
        Self { path, original }
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        if let Some(ref content) = self.original {
            let _ = std::fs::write(&self.path, content);
        } else if self.path.exists() {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

struct DirGuard {
    path: std::path::PathBuf,
    backup: std::path::PathBuf,
    moved: bool,
}

impl DirGuard {
    fn new(path: impl Into<std::path::PathBuf>) -> Self {
        let path = path.into();
        let backup = path.with_extension(format!("smoke-backup-{}", std::process::id()));
        let moved = if path.exists() {
            let _ = std::fs::remove_dir_all(&backup);
            std::fs::rename(&path, &backup).is_ok()
        } else {
            false
        };
        Self {
            path,
            backup,
            moved,
        }
    }
}

impl Drop for DirGuard {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = std::fs::remove_dir_all(&self.path);
        }
        if self.moved && self.backup.exists() {
            let _ = std::fs::rename(&self.backup, &self.path);
        }
    }
}

fn run(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(args)
        .output()
        .expect("ctxt binary should run");
    assert!(output.status.success(), "command failed: {args:?}");
    String::from_utf8(output.stdout).expect("stdout should be UTF-8")
}

fn run_fail(args: &[&str]) -> serde_json::Value {
    let output = Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(args)
        .output()
        .expect("ctxt binary should run");
    assert!(!output.status.success(), "command should fail: {args:?}");
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    serde_json::from_str(&stderr).expect("error JSON should parse")
}

fn valid_phase_4f_proposal(id: &str) -> serde_json::Value {
    serde_json::json!({
        "schema_version": "proposal.v1",
        "id": id,
        "created_at": "2026-06-13T12:00:00Z",
        "phase": "Phase 4f",
        "title": "Proposal Artifact Contract",
        "summary": "Read-only proposal artifact contract.",
        "intent": "Prepare proposal-before-apply inspection without applying changes.",
        "allowed_files": ["src/cli.rs"],
        "forbidden_scope": ["proposal apply"],
        "changes": [
            {
                "path": "src/cli.rs",
                "action": "modify",
                "summary": "Add read-only proposal commands."
            }
        ],
        "validation": ["cargo test"],
        "network": "offline-only",
        "secrets": "no secrets read",
        "status": "draft"
    })
}

#[test]
fn help_mentions_safety_defaults() {
    let _guard = test_lock();
    let stdout = run(&["--help"]);
    assert!(stdout.contains("SAFETY DEFAULTS"));
    assert!(stdout.contains("network_default=deny"));
}

#[test]
fn doctor_is_local_and_deterministic() {
    let _guard = test_lock();
    let stdout = run(&["doctor"]);
    assert!(stdout.contains("status: ok"));
    assert!(stdout.contains("provider_default: dummy"));
}

#[test]
fn doctor_json_is_machine_readable() {
    let _guard = test_lock();
    let stdout = run(&["--json", "doctor"]);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("doctor JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "doctor");
    assert_eq!(value["status"], "ok");
    assert_eq!(value["provider_default"], "dummy");
    assert_eq!(value["network_default"], "deny");
    assert_eq!(value["auth"]["required"], false);
}

#[test]
fn providers_include_dummy_and_ollama_variants() {
    let _guard = test_lock();
    let stdout = run(&["providers", "list"]);
    assert!(stdout.contains("dummy"));
    assert!(stdout.contains("ollama-local"));
    assert!(stdout.contains("ollama-cloud-direct"));
}

#[test]
fn providers_json_lists_stable_provider_objects() {
    let _guard = test_lock();
    let stdout = run(&["--json", "providers", "list"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("providers JSON should parse");
    let providers = value["providers"]
        .as_array()
        .expect("providers should be an array");

    assert_eq!(value["ok"], true);
    assert!(providers.iter().any(|provider| {
        provider["name"] == "dummy" && provider["kind"] == "dummy" && provider["network"] == false
    }));
    assert!(providers
        .iter()
        .any(|provider| provider["name"] == "openai-compatible"));
}

#[test]
fn init_json_dry_run_reports_target_without_write() {
    let _guard = test_lock();
    let target_path = std::path::Path::new("comptext.smoke.toml");
    let _target_guard = FileGuard::new(target_path);
    if target_path.exists() {
        let _ = std::fs::remove_file(target_path);
    }

    let stdout = run(&[
        "--json",
        "init",
        "--dry-run",
        "--out",
        "comptext.smoke.toml",
    ]);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("init JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "init");
    assert_eq!(value["dry_run"], true);
    assert_eq!(value["source"], "comptext.example.toml");
    assert_eq!(value["target"], "comptext.smoke.toml");
    assert!(!target_path.exists());
}

#[test]
fn init_json_writes_explicit_local_config_without_overwrite() {
    let _guard = test_lock();
    let target_path = std::path::Path::new("comptext.smoke.toml");
    let _target_guard = FileGuard::new(target_path);
    if target_path.exists() {
        let _ = std::fs::remove_file(target_path);
    }

    let stdout = run(&["--json", "init", "--out", "comptext.smoke.toml"]);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("init JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "init");
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["target"], "comptext.smoke.toml");
    assert!(target_path.exists());

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["--json", "init", "--out", "comptext.smoke.toml"])
        .output()
        .expect("ctxt binary should run");
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    let error: serde_json::Value =
        serde_json::from_str(&stderr).expect("overwrite error should be JSON");
    assert!(error["error"]["message"]
        .as_str()
        .unwrap()
        .contains("refusing to overwrite"));
}

#[test]
fn context_inspect_json_reports_pack_shape() {
    let _guard = test_lock();
    let stdout = run(&["--json", "context", "inspect"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("context inspect JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "context inspect");
    assert_eq!(value["schema_version"], "0.1");
    assert!(value["included_file_count"].as_u64().unwrap() > 0);
    assert!(value["included_files"]
        .as_array()
        .unwrap()
        .iter()
        .any(|file| { file.as_str().unwrap().ends_with("src/cli.rs") }));
    assert_eq!(value["policy"]["secrets_redacted"], true);
}

#[test]
fn context_pack_json_writes_latest_artifact() {
    let _guard = test_lock();
    let stdout = run(&["--json", "context", "pack", "--task", "JSON smoke pack"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("context pack JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "context pack");
    assert_eq!(value["path"], ".comptext/context_pack.latest.json");
    assert_eq!(value["task"], "JSON smoke pack");
    assert!(std::path::Path::new(".comptext/context_pack.latest.json").exists());
}

#[test]
fn artifacts_json_lists_and_reads_local_evidence() {
    let _guard = test_lock();
    let artifact_path = std::path::Path::new(".comptext/context_pack.latest.json");
    let _artifact_guard = FileGuard::new(artifact_path);
    run(&["--json", "context", "pack", "--task", "Artifact smoke"]);

    let list_stdout = run(&["--json", "artifacts", "list"]);
    let list_value: serde_json::Value =
        serde_json::from_str(&list_stdout).expect("artifacts list JSON should parse");
    let artifacts = list_value["artifacts"]
        .as_array()
        .expect("artifacts should be an array");
    assert_eq!(list_value["ok"], true);
    assert!(artifacts
        .iter()
        .any(|artifact| artifact["path"] == ".comptext/context_pack.latest.json"));

    let read_stdout = run(&[
        "--json",
        "artifacts",
        "read",
        ".comptext/context_pack.latest.json",
        "--max-bytes",
        "512",
    ]);
    let read_value: serde_json::Value =
        serde_json::from_str(&read_stdout).expect("artifacts read JSON should parse");
    assert_eq!(read_value["ok"], true);
    assert_eq!(read_value["command"], "artifacts read");
    assert_eq!(read_value["kind"], "runtime");
    assert!(read_value["content"]
        .as_str()
        .unwrap()
        .contains("Artifact smoke"));
}

#[test]
fn ask_json_dry_run_reports_artifacts_without_provider_call() {
    let _guard = test_lock();
    let stdout = run(&["--json", "ask", "--dry-run", "Summarize JSON contract"]);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("ask JSON should parse");
    let artifacts = value["artifacts"]
        .as_array()
        .expect("artifacts should be an array");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "ask");
    assert_eq!(value["dry_run"], true);
    assert_eq!(value["provider"], "dummy");
    assert!(artifacts
        .iter()
        .any(|path| path == ".comptext/context_pack.latest.json"));
}

#[test]
fn json_errors_are_machine_readable() {
    let _guard = test_lock();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["--json", "unknown-command"])
        .output()
        .expect("ctxt binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    let value: serde_json::Value =
        serde_json::from_str(&stderr).expect("error JSON should parse from stderr");
    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("unsupported command"));
}

#[test]
fn ask_dummy_provider_succeeds() {
    let _guard = test_lock();
    let stdout = run(&["ask", "--provider", "dummy", "How do I test this repo?"]);
    assert!(stdout.contains("Response from dummy provider:"));
    assert!(stdout.contains("Mock LLM response from CompText Dummy Provider."));
    assert!(stdout.contains("Received prompt: \"How do I test this repo?\""));

    // Verify response file was written
    let response_path = std::path::Path::new(".comptext/model_response.latest.json");
    assert!(response_path.exists());
    let response_content = std::fs::read_to_string(response_path).unwrap();
    assert!(response_content.contains("\"provider\": \"dummy\""));
}

#[test]
fn ask_ollama_provider_respects_network_deny_policy() {
    let _guard = test_lock();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["ask", "--provider", "ollama-local", "hello"])
        .output()
        .expect("ctxt binary should run");

    assert!(
        !output.status.success(),
        "command should fail because network is denied by policy"
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    let stderr_lower = stderr.to_ascii_lowercase();
    assert!(
        stderr_lower.contains("network access denied") && stderr_lower.contains("ollama-local"),
        "unexpected stderr from network-denied ollama run: {stderr}"
    );
}

#[test]
fn propose_dummy_provider_succeeds() {
    let _guard = test_lock();
    let latest_path = std::path::Path::new("proposals/proposal.latest.json");
    let _latest_guard = FileGuard::new(latest_path);
    let task = "Smoke temporary proposal";
    let slugified_path = std::path::Path::new("proposals/proposal_smoke_temporary_proposal.json");
    let _slugified_guard = FileGuard::new(slugified_path);
    if slugified_path.exists() {
        let _ = std::fs::remove_file(slugified_path);
    }

    let stdout = run(&["propose", "--provider", "dummy", task]);
    assert!(stdout.contains("Proposal generated successfully."));
    assert!(stdout.contains("Proposal file: proposals/proposal_smoke_temporary_proposal.json"));
    assert!(stdout.contains("Latest reference: proposals/proposal.latest.json"));

    assert!(slugified_path.exists());
    assert!(latest_path.exists());

    let proposal_content = std::fs::read_to_string(latest_path).unwrap();
    assert!(proposal_content.contains("\"task\": \"Smoke temporary proposal\""));
    assert!(proposal_content.contains("\"schema_version\": \"0.1\""));
    assert!(proposal_content.contains("Mock patch generated by dummy provider:"));
}

#[test]
fn propose_json_reports_proposal_artifacts() {
    let _guard = test_lock();
    let latest_path = std::path::Path::new("proposals/proposal.latest.json");
    let _latest_guard = FileGuard::new(latest_path);
    let task = "JSON proposal smoke";
    let slugified_path = std::path::Path::new("proposals/proposal_json_proposal_smoke.json");
    let _slugified_guard = FileGuard::new(slugified_path);
    if slugified_path.exists() {
        let _ = std::fs::remove_file(slugified_path);
    }

    let stdout = run(&["--json", "propose", "--provider", "dummy", task]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("propose JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "propose");
    assert_eq!(value["provider"], "dummy");
    assert_eq!(
        value["proposal_file"],
        "proposals/proposal_json_proposal_smoke.json"
    );
    assert_eq!(value["latest_reference"], "proposals/proposal.latest.json");
    assert_eq!(value["operation_count"], 1);
    assert!(slugified_path.exists());
}

#[test]
fn proposals_list_missing_root_returns_empty() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");

    let stdout = run(&["--json", "proposals", "list"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("proposals list JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "proposals list");
    assert_eq!(value["count"], 0);
    assert!(value["proposals"].as_array().unwrap().is_empty());
}

#[test]
fn proposals_list_shows_valid_proposal() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    std::fs::create_dir_all("proposals").unwrap();
    let id = "20260613T120000Z-phase-4f-example";
    std::fs::write(
        format!("proposals/{id}.json"),
        serde_json::to_string_pretty(&valid_phase_4f_proposal(id)).unwrap(),
    )
    .unwrap();

    let stdout = run(&["--json", "proposals", "list"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("proposals list JSON should parse");
    let proposals = value["proposals"].as_array().unwrap();

    assert_eq!(value["ok"], true);
    assert_eq!(value["count"], 1);
    assert_eq!(proposals[0]["id"], id);
    assert_eq!(
        proposals[0]["path"],
        "proposals/20260613T120000Z-phase-4f-example.json"
    );
    assert_eq!(proposals[0]["valid"], true);
}

#[test]
fn proposals_inspect_latest_reads_proposal_object() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    std::fs::create_dir_all("proposals").unwrap();
    let older_id = "20260613T110000Z-phase-4f-example";
    let latest_id = "20260613T120000Z-phase-4f-example";
    std::fs::write(
        format!("proposals/{older_id}.json"),
        serde_json::to_string_pretty(&valid_phase_4f_proposal(older_id)).unwrap(),
    )
    .unwrap();
    std::fs::write(
        format!("proposals/{latest_id}.json"),
        serde_json::to_string_pretty(&valid_phase_4f_proposal(latest_id)).unwrap(),
    )
    .unwrap();

    let stdout = run(&[
        "--json",
        "proposals",
        "inspect",
        "latest",
        "--max-bytes",
        "12000",
    ]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("proposals inspect JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "proposals inspect");
    assert_eq!(value["id"], latest_id);
    assert_eq!(value["proposal"]["id"], latest_id);
    assert_eq!(value["truncated"], false);
}

#[test]
fn proposals_inspect_latest_flag_id_matches_positional() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    std::fs::create_dir_all("proposals").unwrap();
    let id = "20260613T120000Z-phase-4f-example";
    std::fs::write(
        format!("proposals/{id}.json"),
        serde_json::to_string_pretty(&valid_phase_4f_proposal(id)).unwrap(),
    )
    .unwrap();

    let positional_stdout = run(&[
        "--json",
        "proposals",
        "inspect",
        "latest",
        "--max-bytes",
        "12000",
    ]);
    let flag_stdout = run(&[
        "--json",
        "proposals",
        "inspect",
        "--id",
        "latest",
        "--max-bytes",
        "12000",
    ]);
    let positional: serde_json::Value =
        serde_json::from_str(&positional_stdout).expect("positional JSON should parse");
    let flag: serde_json::Value =
        serde_json::from_str(&flag_stdout).expect("flag JSON should parse");

    assert_eq!(positional["id"], flag["id"]);
    assert_eq!(positional["path"], flag["path"]);
}

#[test]
fn proposals_validate_latest_accepts_valid_contract() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    std::fs::create_dir_all("proposals").unwrap();
    let id = "20260613T120000Z-phase-4f-example";
    std::fs::write(
        format!("proposals/{id}.json"),
        serde_json::to_string_pretty(&valid_phase_4f_proposal(id)).unwrap(),
    )
    .unwrap();

    let stdout = run(&["--json", "proposals", "validate", "latest"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("proposals validate JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "proposals validate");
    assert_eq!(value["valid"], true);
    assert!(value["errors"].as_array().unwrap().is_empty());
}

#[test]
fn proposals_validate_latest_flag_id_accepts_valid_contract() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    std::fs::create_dir_all("proposals").unwrap();
    let id = "20260613T120000Z-phase-4f-example";
    std::fs::write(
        format!("proposals/{id}.json"),
        serde_json::to_string_pretty(&valid_phase_4f_proposal(id)).unwrap(),
    )
    .unwrap();

    let stdout = run(&["--json", "proposals", "validate", "--id", "latest"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("proposals validate JSON should parse");

    assert_eq!(value["id"], id);
    assert_eq!(value["valid"], true);
}

#[test]
fn proposals_validate_missing_required_field_returns_invalid() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    std::fs::create_dir_all("proposals").unwrap();
    let id = "20260613T120000Z-phase-4f-example";
    let mut proposal = valid_phase_4f_proposal(id);
    proposal.as_object_mut().unwrap().remove("intent");
    std::fs::write(
        format!("proposals/{id}.json"),
        serde_json::to_string_pretty(&proposal).unwrap(),
    )
    .unwrap();

    let stdout = run(&["--json", "proposals", "validate", "latest"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("proposals validate JSON should parse");

    assert_eq!(value["valid"], false);
    assert!(value["errors"]
        .as_array()
        .unwrap()
        .iter()
        .any(|error| error.as_str().unwrap().contains("intent")));
}

#[test]
fn proposals_validate_id_mismatch_returns_invalid() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    std::fs::create_dir_all("proposals").unwrap();
    let id = "20260613T120000Z-phase-4f-example";
    let mut proposal = valid_phase_4f_proposal(id);
    proposal["id"] = serde_json::json!("20260613T120000Z-other");
    std::fs::write(
        format!("proposals/{id}.json"),
        serde_json::to_string_pretty(&proposal).unwrap(),
    )
    .unwrap();

    let stdout = run(&["--json", "proposals", "validate", "latest"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("proposals validate JSON should parse");

    assert_eq!(value["valid"], false);
    assert!(value["errors"]
        .as_array()
        .unwrap()
        .iter()
        .any(|error| error.as_str().unwrap().contains("match filename stem")));
}

#[test]
fn proposals_validate_malformed_json_returns_invalid() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    std::fs::create_dir_all("proposals").unwrap();
    std::fs::write(
        "proposals/20260613T120000Z-phase-4f-example.json",
        "{not valid json",
    )
    .unwrap();

    let stdout = run(&["--json", "proposals", "validate", "latest"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("proposals validate JSON should parse");

    assert_eq!(value["valid"], false);
    assert!(value["errors"]
        .as_array()
        .unwrap()
        .iter()
        .any(|error| error.as_str().unwrap().contains("malformed")));
}

#[test]
fn proposals_reject_path_traversal_id_with_json_error() {
    let _guard = test_lock();
    let _proposal_dir_guard = DirGuard::new("proposals");
    let value = run_fail(&["--json", "proposals", "inspect", "--id", "../outside"]);

    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("invalid proposal id"));
}

#[test]
fn proposals_reject_invalid_max_bytes_with_json_error() {
    let _guard = test_lock();
    let value = run_fail(&[
        "--json",
        "proposals",
        "inspect",
        "latest",
        "--max-bytes",
        "nope",
    ]);

    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("invalid --max-bytes"));
}

#[test]
fn proposals_reject_duplicate_id_with_json_error() {
    let _guard = test_lock();
    let value = run_fail(&[
        "--json",
        "proposals",
        "validate",
        "--id",
        "latest",
        "--id",
        "latest",
    ]);

    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("duplicate --id"));
}

#[test]
fn validate_json_lists_standard_commands() {
    let _guard = test_lock();
    let stdout = run(&["--json", "validate"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("validate JSON should parse");
    let commands = value["validation_commands"]
        .as_array()
        .expect("validation_commands should be an array");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "validate");
    assert!(commands.iter().any(|cmd| cmd == "cargo test"));
    assert!(commands
        .iter()
        .any(|cmd| cmd == "cargo clippy -- -D warnings"));
}

#[test]
fn validate_run_executes_validation_commands() {
    let _guard = test_lock();
    let output = Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["--json", "validate", "--run"])
        .env("CTXT_VALIDATE_COMMANDS_FOR_TEST", "rustc --version")
        .output()
        .expect("ctxt binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("validate --run JSON should parse");
    let steps = value["steps"].as_array().expect("steps should be an array");
    let first_step = steps.first().expect("steps should not be empty");

    assert_eq!(value["command"], "validate");
    assert_eq!(value["ok"], true);
    assert_eq!(value["run"], true);
    assert!(!steps.is_empty());
    assert!(first_step.get("cmd").is_some());
    assert!(first_step["cmd"]
        .as_str()
        .expect("cmd should be a string")
        .contains("rustc --version"));
    assert_eq!(first_step["ok"], true);
    assert_eq!(first_step["exit_code"], 0);
    assert!(first_step.get("stdout_excerpt").is_some());
    assert!(first_step.get("stderr_excerpt").is_some());
}

#[test]
fn capabilities_json_reports_phase_four_b_introspection() {
    let _guard = test_lock();
    let stdout = run(&["--json", "capabilities"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("capabilities JSON should parse");
    let phases = value["phases"]
        .as_array()
        .expect("phases should be an array");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "capabilities");
    assert_eq!(value["schema_version"], "0.1");
    assert!(phases
        .iter()
        .any(|phase| { phase["phase"] == "4b" && phase["name"] == "agent-friendly CLI polish" }));
    assert_eq!(value["features"]["real_external_execution"], false);
    assert_eq!(value["features"]["network_gate"], false);
    assert_eq!(value["features"]["apply_gate"], false);
}

#[test]
fn schema_json_reports_stable_contracts() {
    let _guard = test_lock();
    let stdout = run(&["--json", "schema"]);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("schema JSON should parse");
    let contracts = value["contracts"]
        .as_array()
        .expect("contracts should be an array");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "schema");
    assert_eq!(value["schema_version"], "0.1");
    for command in [
        "capabilities",
        "runs list",
        "runs read",
        "agent discover",
        "agent run --allow-external --proposal-only",
        "validate",
    ] {
        assert!(contracts
            .iter()
            .any(|contract| contract["command"] == command));
    }
    assert_eq!(value["safety"]["read_only"], true);
    assert_eq!(value["safety"]["network_used"], false);
    assert_eq!(value["safety"]["external_agent_invoked"], false);
    assert_eq!(value["safety"]["apply_performed"], false);
}

#[test]
fn schema_unexpected_arg_fails_with_json_error() {
    let _guard = test_lock();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["--json", "schema", "unexpected"])
        .output()
        .expect("ctxt binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    let value: serde_json::Value = serde_json::from_str(&stderr).expect("error JSON should parse");
    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("unexpected argument"));
}

#[test]
fn self_report_json_reports_runtime_baseline() {
    let _guard = test_lock();
    let stdout = run(&["--json", "self", "report"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("self report JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "self report");
    assert_eq!(value["schema_version"], "0.1");
    assert_eq!(value["runtime"]["name"], "ctxt");
    assert_eq!(value["runtime"]["phase"], "4e");
    assert_eq!(value["validation"]["last_known_unit_tests"], 37);
    assert_eq!(value["validation"]["last_known_smoke_tests"], 39);
    assert_eq!(value["agent_policy"]["external_execution"], false);
    assert_eq!(value["agent_policy"]["network_default"], "deny");
    assert_eq!(value["agent_policy"]["apply_automatic"], false);
}

#[test]
fn self_without_report_fails_with_json_error() {
    let _guard = test_lock();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["--json", "self"])
        .output()
        .expect("ctxt binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    let value: serde_json::Value = serde_json::from_str(&stderr).expect("error JSON should parse");
    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("missing subcommand"));
}

#[test]
fn self_report_extra_arg_fails_with_json_error() {
    let _guard = test_lock();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["--json", "self", "report", "extra"])
        .output()
        .expect("ctxt binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    let value: serde_json::Value = serde_json::from_str(&stderr).expect("error JSON should parse");
    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("unexpected argument"));
}

#[test]
fn agent_list_json_reports_phase_one_agents() {
    let _guard = test_lock();
    let stdout = run(&["--json", "agent", "list"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent list JSON should parse");
    let agents = value["agents"]
        .as_array()
        .expect("agents should be an array");

    assert_eq!(value["command"], "agent list");
    assert_eq!(value["ok"], true);
    assert!(agents
        .iter()
        .any(|agent| agent["kind"] == "dummy" && agent["status"] == "available"));
    assert!(agents
        .iter()
        .any(|agent| agent["kind"] == "codex" && agent["status"] == "dry-run-only"));
    assert!(agents
        .iter()
        .any(|agent| agent["kind"] == "antigravity" && agent["status"] == "dry-run-only"));
}

#[test]
fn agent_discover_json_lists_supported_targets() {
    let _guard = test_lock();
    let stdout = run(&["--json", "agent", "discover"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent discover JSON should parse");
    let targets = value["targets"]
        .as_array()
        .expect("targets should be an array");

    assert_eq!(value["command"], "agent discover");
    assert_eq!(value["ok"], true);
    assert!(targets.iter().any(|target| target == "codex"));
    assert!(targets.iter().any(|target| target == "antigravity"));
    assert_eq!(value["external_agent_invoked"], false);
    assert_eq!(value["network_used"], false);
}

fn assert_agent_discovery_shape(value: &serde_json::Value, kind: &str) {
    assert_eq!(value["command"], "agent discover");
    assert_eq!(value["kind"], kind);
    assert_eq!(value["ok"], true);
    assert_eq!(value["version"], serde_json::Value::Null);
    assert_eq!(value["external_agent_invoked"], false);
    assert_eq!(value["network_used"], false);

    let discovered = value["discovered"]
        .as_bool()
        .expect("discovered should be a bool");
    let path_is_string = value["path"].is_string();
    assert_eq!(discovered, path_is_string);
    assert!(value["path"].is_null() || path_is_string);
    assert!(value["notes"].is_array());
}

#[test]
fn agent_discover_codex_json_reports_path_metadata_only() {
    let _guard = test_lock();
    let stdout = run(&["--json", "agent", "discover", "--kind", "codex"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent discover codex JSON should parse");

    assert_agent_discovery_shape(&value, "codex");
}

#[test]
fn agent_discover_antigravity_json_reports_path_metadata_only() {
    let _guard = test_lock();
    let stdout = run(&["--json", "agent", "discover", "--kind", "antigravity"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent discover antigravity JSON should parse");

    assert_agent_discovery_shape(&value, "antigravity");
}

#[test]
fn agent_discover_unknown_kind_fails_with_json_error() {
    let _guard = test_lock();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["--json", "agent", "discover", "--kind", "unknown"])
        .output()
        .expect("ctxt binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    let value: serde_json::Value = serde_json::from_str(&stderr).expect("error JSON should parse");
    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("unsupported agent discovery kind"));
}

#[test]
fn runs_list_json_reports_latest_reference() {
    let _guard = test_lock();
    let stdout = run(&["--json", "runs", "list"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("runs list JSON should parse");
    let runs = value["runs"].as_array().expect("runs should be an array");
    let latest = runs.first().expect("latest run reference should exist");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "runs list");
    assert_eq!(value["schema_version"], "0.1");
    assert_eq!(latest["id"], "latest");
    assert_eq!(latest["path"], ".comptext/runs/latest/run.json");
    assert!(latest["exists"].is_boolean());
}

#[test]
fn runs_read_latest_positional_reads_bounded_runtime_artifact() {
    let _guard = test_lock();
    let run_path = std::path::Path::new(".comptext/runs/latest/run.json");
    let _run_guard = FileGuard::new(run_path);
    run(&[
        "--json",
        "agent",
        "run",
        "--kind",
        "codex",
        "--task",
        "Prepare Codex execution plan",
        "--allow-external",
        "--proposal-only",
    ]);

    let stdout = run(&["--json", "runs", "read", "latest", "--max-bytes", "12000"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("runs read JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "runs read");
    assert_eq!(value["schema_version"], "0.1");
    assert_eq!(value["id"], "latest");
    assert_eq!(value["path"], ".comptext/runs/latest/run.json");
    assert_eq!(value["max_bytes"], 12000);
    assert!(value["content"]
        .as_str()
        .unwrap()
        .contains("execution-plan-only"));
}

#[test]
fn runs_read_latest_flag_id_reads_bounded_runtime_artifact() {
    let _guard = test_lock();
    let run_path = std::path::Path::new(".comptext/runs/latest/run.json");
    let _run_guard = FileGuard::new(run_path);
    run(&[
        "--json",
        "agent",
        "run",
        "--kind",
        "codex",
        "--task",
        "Prepare Codex execution plan",
        "--allow-external",
        "--proposal-only",
    ]);

    let stdout = run(&[
        "--json",
        "runs",
        "read",
        "--id",
        "latest",
        "--max-bytes",
        "12000",
    ]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("runs read JSON should parse");

    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "runs read");
    assert_eq!(value["schema_version"], "0.1");
    assert_eq!(value["id"], "latest");
    assert_eq!(value["path"], ".comptext/runs/latest/run.json");
}

#[test]
fn runs_read_errors_are_machine_readable() {
    let _guard = test_lock();
    let cases = vec![
        vec!["--json", "runs", "read", "unknown"],
        vec!["--json", "runs", "read", "latest", "--max-bytes", "nope"],
        vec!["--json", "runs", "read", "--id", "latest", "--id", "latest"],
        vec!["--json", "runs", "read", "latest", "extra"],
    ];
    for args in cases {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
            .args(&args)
            .output()
            .expect("ctxt binary should run");

        assert!(!output.status.success(), "command should fail: {args:?}");
        let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
        let value: serde_json::Value =
            serde_json::from_str(&stderr).expect("error JSON should parse");
        assert_eq!(value["ok"], false);
        assert!(value["error"]["message"].is_string());
    }
}

#[test]
fn agent_run_dummy_writes_run_artifact() {
    let _guard = test_lock();
    let run_path = std::path::Path::new(".comptext/runs/latest/run.json");
    let context_path = std::path::Path::new(".comptext/context_pack.latest.json");
    let _run_guard = FileGuard::new(run_path);
    let _context_guard = FileGuard::new(context_path);

    let stdout = run(&[
        "--json",
        "agent",
        "run",
        "--kind",
        "dummy",
        "--task",
        "Agent dummy smoke",
    ]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent run JSON should parse");

    assert_eq!(value["command"], "agent run");
    assert_eq!(value["kind"], "dummy");
    assert_eq!(value["task"], "Agent dummy smoke");
    assert_eq!(value["external_execution"], false);
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["ok"], true);
    assert_eq!(value["run_artifact"], ".comptext/runs/latest/run.json");
    assert!(run_path.exists());

    let artifact: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(run_path).unwrap())
            .expect("run artifact should parse");
    assert_eq!(artifact["schema_version"], "0.1");
    assert_eq!(artifact["task"], "Agent dummy smoke");
    assert_eq!(artifact["agent_kind"], "dummy");
    assert_eq!(
        artifact["context_pack"],
        ".comptext/context_pack.latest.json"
    );
    assert_eq!(artifact["network_default"], "deny");
    assert_eq!(artifact["proposal_required"], true);
    assert!(artifact["timestamp"]
        .as_str()
        .unwrap()
        .parse::<u64>()
        .is_ok());
}

#[test]
fn agent_run_codex_is_dry_run_by_default() {
    let _guard = test_lock();
    let run_path = std::path::Path::new(".comptext/runs/latest/run.json");
    let _run_guard = FileGuard::new(run_path);

    let stdout = run(&[
        "--json",
        "agent",
        "run",
        "--kind",
        "codex",
        "--task",
        "Codex dry run smoke",
    ]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent run JSON should parse");

    assert_eq!(value["kind"], "codex");
    assert_eq!(value["external_execution"], false);
    assert_eq!(value["dry_run"], true);
    assert_eq!(value["ok"], true);
    assert!(value["would_run"].as_str().unwrap().contains("codex"));
}

#[test]
fn agent_run_antigravity_is_dry_run_by_default() {
    let _guard = test_lock();
    let run_path = std::path::Path::new(".comptext/runs/latest/run.json");
    let _run_guard = FileGuard::new(run_path);

    let stdout = run(&[
        "--json",
        "agent",
        "run",
        "--kind",
        "antigravity",
        "--task",
        "Antigravity dry run smoke",
    ]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent run JSON should parse");

    assert_eq!(value["kind"], "antigravity");
    assert_eq!(value["external_execution"], false);
    assert_eq!(value["dry_run"], true);
    assert_eq!(value["ok"], true);
    assert!(value["would_run"].as_str().unwrap().contains("antigravity"));
}

#[test]
fn agent_run_codex_allow_external_proposal_only_returns_execution_plan() {
    let _guard = test_lock();
    let run_path = std::path::Path::new(".comptext/runs/latest/run.json");
    let _run_guard = FileGuard::new(run_path);

    let stdout = run(&[
        "--json",
        "agent",
        "run",
        "--kind",
        "codex",
        "--task",
        "Codex execution plan smoke",
        "--allow-external",
        "--proposal-only",
    ]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent run JSON should parse");
    assert_eq!(value["kind"], "codex");
    assert_eq!(value["external_execution"], false);
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["allow_external"], true);
    assert_eq!(value["proposal_only"], true);
    assert_eq!(value["ok"], true);
    assert_eq!(value["status"], "execution-plan-only");
    assert_eq!(value["execution_plan"]["agent_kind"], "codex");
    assert_eq!(value["execution_plan"]["mode"], "proposal-only");
    assert_eq!(value["execution_plan"]["external_process_invoked"], false);
    assert_eq!(value["execution_plan"]["network_default"], "deny");
    assert_eq!(value["execution_plan"]["writes_allowed"], false);
    assert_eq!(value["execution_plan"]["apply_allowed"], false);
    assert!(value["would_run"].as_str().unwrap().contains("codex"));
    assert_eq!(value["safety"]["external_agent_invoked"], false);

    let artifact: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(run_path).unwrap())
            .expect("run artifact should parse");
    assert_eq!(artifact["allow_external"], true);
    assert_eq!(artifact["proposal_only"], true);
    assert_eq!(artifact["status"], "execution-plan-only");
    assert_eq!(artifact["execution_plan"]["agent_kind"], "codex");
    assert_eq!(artifact["safety_flags"]["external_agent_invoked"], false);
    assert_eq!(artifact["safety_flags"]["apply_allowed"], false);
    assert_eq!(artifact["safety_flags"]["network_allowed"], false);
}

#[test]
fn agent_run_antigravity_allow_external_proposal_only_returns_execution_plan() {
    let _guard = test_lock();
    let run_path = std::path::Path::new(".comptext/runs/latest/run.json");
    let _run_guard = FileGuard::new(run_path);

    let stdout = run(&[
        "--json",
        "agent",
        "run",
        "--kind",
        "antigravity",
        "--task",
        "Antigravity execution plan smoke",
        "--allow-external",
        "--proposal-only",
    ]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("agent run JSON should parse");
    assert_eq!(value["kind"], "antigravity");
    assert_eq!(value["external_execution"], false);
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["allow_external"], true);
    assert_eq!(value["proposal_only"], true);
    assert_eq!(value["ok"], true);
    assert_eq!(value["status"], "execution-plan-only");
    assert_eq!(value["execution_plan"]["agent_kind"], "antigravity");
    assert_eq!(value["execution_plan"]["mode"], "proposal-only");
    assert_eq!(value["execution_plan"]["external_process_invoked"], false);
    assert!(value["would_run"].as_str().unwrap().contains("antigravity"));

    let artifact: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(run_path).unwrap())
            .expect("run artifact should parse");
    assert_eq!(artifact["execution_plan"]["agent_kind"], "antigravity");
    assert_eq!(artifact["safety_flags"]["external_agent_invoked"], false);
    assert_eq!(artifact["safety_flags"]["apply_allowed"], false);
    assert_eq!(artifact["safety_flags"]["network_allowed"], false);
}

#[test]
fn unknown_agent_kind_fails_with_json_error() {
    let _guard = test_lock();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args([
            "--json", "agent", "run", "--kind", "unknown", "--task", "Nope",
        ])
        .output()
        .expect("ctxt binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    let value: serde_json::Value = serde_json::from_str(&stderr).expect("error JSON should parse");
    assert_eq!(value["ok"], false);
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("unsupported agent kind"));
}

#[test]
fn apply_and_validate_succeeds() {
    let _guard = test_lock();
    let mock_file = std::path::Path::new("tests/mock_applied_patch.rs");
    let _mock_file_guard = FileGuard::new(mock_file);
    std::fs::write(mock_file, "// initial\n").unwrap();

    let mock_proposal = serde_json::json!({
        "schema_version": "0.1",
        "task": "Test apply",
        "rationale": "Verify apply and validate commands",
        "preconditions": [],
        "affected_files": ["tests/mock_applied_patch.rs"],
        "operations": [
            {
                "op": "patch",
                "path": "tests/mock_applied_patch.rs",
                "detail": "Mock test patch detail"
            }
        ],
        "validation_commands": ["cargo --version"],
        "rollback_strategy": "none",
        "risk_notes": "none"
    });

    let proposal_path = std::path::Path::new("proposals/proposal_test_apply.json");
    let _proposal_guard = FileGuard::new(proposal_path);
    std::fs::write(
        proposal_path,
        serde_json::to_string_pretty(&mock_proposal).unwrap(),
    )
    .unwrap();

    let stdout_apply = run(&["apply", "--yes", "proposals/proposal_test_apply.json"]);
    assert!(stdout_apply.contains("Applying Proposal:"));
    assert!(stdout_apply.contains("Proposal applied and validated successfully."));

    let modified_content = std::fs::read_to_string(mock_file).unwrap();
    assert!(modified_content.contains("// Mock patch applied:"));

    let stdout_validate = run(&["validate"]);
    assert!(stdout_validate.contains("Standard local validation commands:"));
    assert!(stdout_validate.contains("cargo test"));
}

#[test]
fn apply_rejects_disallowed_paths() {
    let _guard = test_lock();
    let mock_proposal = serde_json::json!({
        "schema_version": "0.1",
        "task": "Malicious task",
        "rationale": "Try to edit forbidden file",
        "preconditions": [],
        "affected_files": [".env"],
        "operations": [
            {
                "op": "patch",
                "path": ".env",
                "detail": "inject secret"
            }
        ],
        "validation_commands": [],
        "rollback_strategy": "none",
        "risk_notes": "high"
    });

    let path = std::path::Path::new("proposals/proposal_malicious.json");
    let _proposal_guard = FileGuard::new(path);
    std::fs::write(path, serde_json::to_string_pretty(&mock_proposal).unwrap()).unwrap();

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(["apply", "--yes", "proposals/proposal_malicious.json"])
        .output()
        .expect("ctxt binary should run");

    assert!(
        !output.status.success(),
        "should fail on security policy violation"
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("Security Policy Violation: Path '.env' is not an allowed write path."));
}

#[test]
fn test_antigravity_commands_skeleton() {
    let _guard = test_lock();

    let out_export = run(&["antigravity", "export"]);
    assert!(out_export.contains("Antigravity bundle export initialized."));

    let out_skills = run(&["antigravity", "skills", "validate"]);
    assert!(out_skills.contains("Validating repo-local skills..."));
    assert!(out_skills.contains("All skill paths verified."));

    let out_agents = run(&["antigravity", "agents", "export"]);
    assert!(out_agents.contains("Exporting advisory subagents metadata..."));
    assert!(out_agents.contains("advisory only"));

    let out_hooks = run(&["antigravity", "hooks", "audit"]);
    assert!(out_hooks.contains("Auditing hook permissions configuration..."));
    assert!(out_hooks.contains("No live runtime hooks detected"));

    let out_plugin = run(&["antigravity", "plugin", "package"]);
    assert!(out_plugin.contains("Packaging repo-local plugin bundle..."));
    assert!(out_plugin.contains("MCP outputs treated as untrusted input"));
}

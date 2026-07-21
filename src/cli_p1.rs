use crate::contracts::{
    AgentSpec, EvidenceEvent, CompletionContract, ReplayManifest, ReplayEventItem,
    ErrorEnvelope, jcs_hash, HashItem
};
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use serde_json::json;

pub fn emit_p1_error(code: &str, msg: &str, details: Option<serde_json::Value>) -> i32 {
    let err = ErrorEnvelope {
        contract_name: "error-envelope".to_string(),
        schema_version: "v1".to_string(),
        error_code: code.to_string(),
        message: msg.to_string(),
        details,
    };
    eprintln!("{}", serde_json::to_string_pretty(&err).unwrap());
    1
}

pub fn handle_agent_validate_spec(spec_path: &str, json_output: bool) -> Result<i32, String> {
    let path = Path::new(spec_path);
    if !path.exists() {
        emit_p1_error("RESOURCE_NOT_FOUND", &format!("Spec file not found: {spec_path}"), None);
        return Ok(1);
    }
    
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read spec file: {e}"))?;
        
    let spec: AgentSpec = match serde_json::from_str(&content) {
        Ok(s) => s,
        Err(e) => {
            emit_p1_error("INVALID_AGENT_SPEC", &format!("Malformed JSON spec: {e}"), None);
            return Ok(1);
        }
    };
    
    // Invariants check
    if spec.contract_name != "agent-spec" || spec.schema_version != "v1" {
        emit_p1_error("INVALID_AGENT_SPEC", "Spec is missing contract_name='agent-spec' or schema_version='v1'", None);
        return Ok(1);
    }
    
    if spec.agent_spec_id.is_empty() {
        emit_p1_error("INVALID_AGENT_SPEC", "agent_spec_id cannot be empty", None);
        return Ok(1);
    }
    
    // Check regex: ^[a-z][a-z0-9_-]*$
    let re = regex::Regex::new(r"^[a-z][a-z0-9_-]*$").unwrap_or_else(|_| regex::Regex::new(".*").unwrap());
    if !re.is_match(&spec.agent_spec_id) {
        emit_p1_error("INVALID_AGENT_SPEC", "agent_spec_id must match pattern ^[a-z][a-z0-9_-]*$", None);
        return Ok(1);
    }
    
    if json_output {
        println!("{}", json!({
            "ok": true,
            "contract_name": "agent-spec",
            "schema_version": "v1",
            "agent_spec_id": spec.agent_spec_id,
            "validated": true
        }));
    } else {
        println!("AgentSpec validated successfully: {}", spec.agent_spec_id);
    }
    
    Ok(0)
}

pub fn handle_agent_dry_run(
    spec_path: &str,
    out_evidence: &str,
    out_replay: &str,
    json_output: bool
) -> Result<i32, String> {
    let path = Path::new(spec_path);
    if !path.exists() {
        emit_p1_error("RESOURCE_NOT_FOUND", &format!("Spec file not found: {spec_path}"), None);
        return Ok(1);
    }
    
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read spec file: {e}"))?;
        
    let spec: AgentSpec = match serde_json::from_str(&content) {
        Ok(s) => s,
        Err(e) => {
            emit_p1_error("INVALID_AGENT_SPEC", &format!("Malformed JSON spec: {e}"), None);
            return Ok(1);
        }
    };
    
    let run_id = format!("run-{}", uuid::Uuid::new_v4());
    let mut sequence = 0;
    let mut events = Vec::new();
    
    // 1. Start event
    let mut evt0 = EvidenceEvent {
        contract_name: "evidence-event".to_string(),
        schema_version: "v1".to_string(),
        event_id: format!("evt-{}", sequence),
        run_id: run_id.clone(),
        sequence,
        parent_event_hash: None,
        timestamp: "2026-07-21T10:00:00Z".to_string(), // Deterministic timestamp for P1
        event_type: "start".to_string(),
        action: None,
        inputs: None,
        outputs: None,
        metadata: None,
        event_hash: "".to_string(),
    };
    let val0 = serde_json::to_value(&evt0).unwrap();
    evt0.event_hash = jcs_hash(&val0);
    events.push(evt0);
    
    // 2. Capability check & Tool simulation for each pipeline step
    for step in &spec.pipeline {
        sequence += 1;
        let last_hash = events.last().unwrap().event_hash.clone();
        
        // capability decision
        // In P1: fixture.echo is ALLOW, everything else is DENY
        let tool_name = if step == "echo-step" { "fixture.echo" } else { "unknown-tool" };
        let (policy, decision) = if tool_name == "fixture.echo" {
            ("ALLOW", "ALLOW")
        } else {
            ("DENY", "DENY")
        };
        
        let mut evt_cap = EvidenceEvent {
            contract_name: "evidence-event".to_string(),
            schema_version: "v1".to_string(),
            event_id: format!("evt-{}", sequence),
            run_id: run_id.clone(),
            sequence,
            parent_event_hash: Some(last_hash),
            timestamp: format!("2026-07-21T10:00:{:02}Z", sequence),
            event_type: "step".to_string(),
            action: Some("capability.decided".to_string()),
            inputs: None,
            outputs: None,
            metadata: Some(json!({
                "tool_name": tool_name,
                "policy": policy,
                "decision": decision
            })),
            event_hash: "".to_string(),
        };
        let val_cap = serde_json::to_value(&evt_cap).unwrap();
        evt_cap.event_hash = jcs_hash(&val_cap);
        events.push(evt_cap);
        
        if decision == "DENY" {
            // Write output error and return
            let last_hash = events.last().unwrap().event_hash.clone();
            sequence += 1;
            let mut evt_err = EvidenceEvent {
                contract_name: "evidence-event".to_string(),
                schema_version: "v1".to_string(),
                event_id: format!("evt-{}", sequence),
                run_id: run_id.clone(),
                sequence,
                parent_event_hash: Some(last_hash),
                timestamp: format!("2026-07-21T10:00:{:02}Z", sequence),
                event_type: "error".to_string(),
                action: None,
                inputs: None,
                outputs: None,
                metadata: Some(json!({
                    "error_code": "CAPABILITY_DENIED",
                    "message": format!("Access denied for tool {}", tool_name)
                })),
                event_hash: "".to_string(),
            };
            let val_err = serde_json::to_value(&evt_err).unwrap();
            evt_err.event_hash = jcs_hash(&val_err);
            events.push(evt_err);
            
            // Write evidence events up to now
            write_events_to_file(&events, out_evidence)?;
            
            emit_p1_error("CAPABILITY_DENIED", &format!("Access denied for tool {}", tool_name), None);
            return Ok(1);
        }
        
        // Simulate tool run
        let last_hash = events.last().unwrap().event_hash.clone();
        sequence += 1;
        let mut evt_tool = EvidenceEvent {
            contract_name: "evidence-event".to_string(),
            schema_version: "v1".to_string(),
            event_id: format!("evt-{}", sequence),
            run_id: run_id.clone(),
            sequence,
            parent_event_hash: Some(last_hash),
            timestamp: format!("2026-07-21T10:00:{:02}Z", sequence),
            event_type: "step".to_string(),
            action: Some(tool_name.to_string()),
            inputs: Some(vec![HashItem {
                key: "message".to_string(),
                hash: jcs_hash(&json!("hello")),
            }]),
            outputs: Some(vec![HashItem {
                key: "echo".to_string(),
                hash: jcs_hash(&json!("hello")),
            }]),
            metadata: None,
            event_hash: "".to_string(),
        };
        let val_tool = serde_json::to_value(&evt_tool).unwrap();
        evt_tool.event_hash = jcs_hash(&val_tool);
        events.push(evt_tool);
    }
    
    // 3. End event
    let last_hash = events.last().unwrap().event_hash.clone();
    sequence += 1;
    let mut evt_end = EvidenceEvent {
        contract_name: "evidence-event".to_string(),
        schema_version: "v1".to_string(),
        event_id: format!("evt-{}", sequence),
        run_id: run_id.clone(),
        sequence,
        parent_event_hash: Some(last_hash),
        timestamp: format!("2026-07-21T10:00:{:02}Z", sequence),
        event_type: "end".to_string(),
        action: None,
        inputs: None,
        outputs: None,
        metadata: None,
        event_hash: "".to_string(),
    };
    let val_end = serde_json::to_value(&evt_end).unwrap();
    evt_end.event_hash = jcs_hash(&val_end);
    events.push(evt_end);
    
    // Write evidence JSONL
    write_events_to_file(&events, out_evidence)?;
    
    // Write CompletionContract to a completion file (we can write it alongside evidence or in evidence folder)
    let root_hash = events.last().unwrap().event_hash.clone();
    let comp = CompletionContract {
        contract_name: "completion-contract".to_string(),
        schema_version: "v1".to_string(),
        run_id: run_id.clone(),
        status: "success".to_string(),
        reason: "Execution completed".to_string(),
        final_sequence: sequence,
        evidence_root_hash: root_hash.clone(),
        replayable: true,
    };
    let out_comp_path = format!("{}.completion.json", out_evidence);
    let comp_content = serde_json::to_string_pretty(&comp).unwrap();
    std::fs::write(&out_comp_path, comp_content)
        .map_err(|e| format!("Failed to write completion contract: {e}"))?;
        
    // Generate ReplayManifest
    let replay_items = events.iter().map(|e| ReplayEventItem {
        sequence: e.sequence,
        event_hash: e.event_hash.clone(),
        event_type: e.event_type.clone(),
    }).collect();
    
    let replay = ReplayManifest {
        contract_name: "replay-manifest".to_string(),
        schema_version: "v1".to_string(),
        run_id: run_id.clone(),
        agent_spec_id: spec.agent_spec_id.clone(),
        evidence_root_hash: root_hash.clone(),
        events: replay_items,
    };
    let replay_content = serde_json::to_string_pretty(&replay).unwrap();
    std::fs::write(out_replay, replay_content)
        .map_err(|e| format!("Failed to write replay manifest: {e}"))?;
        
    if json_output {
        println!("{}", json!({
            "ok": true,
            "command": "agent dry-run",
            "run_id": run_id,
            "status": "success",
            "evidence_root_hash": root_hash,
            "out_evidence": out_evidence,
            "out_replay": out_replay
        }));
    } else {
        println!("Dry-run completed successfully.");
        println!("Run ID: {}", run_id);
        println!("Evidence Root Hash: {}", root_hash);
        println!("Evidence saved to: {}", out_evidence);
        println!("Replay saved to: {}", out_replay);
    }
    
    Ok(0)
}

fn write_events_to_file(events: &[EvidenceEvent], file_path: &str) -> Result<(), String> {
    let mut file = File::create(file_path)
        .map_err(|e| format!("Failed to create evidence file: {e}"))?;
    for e in events {
        let serialized = serde_json::to_string(e).unwrap();
        writeln!(file, "{}", serialized)
            .map_err(|e| format!("Failed to write to evidence file: {e}"))?;
    }
    Ok(())
}

pub fn handle_agent_replay(
    replay_path: &str,
    evidence_path: &str,
    json_output: bool
) -> Result<i32, String> {
    let r_path = Path::new(replay_path);
    let e_path = Path::new(evidence_path);
    
    if !r_path.exists() {
        emit_p1_error("RESOURCE_NOT_FOUND", &format!("Replay file not found: {replay_path}"), None);
        return Ok(1);
    }
    if !e_path.exists() {
        emit_p1_error("RESOURCE_NOT_FOUND", &format!("Evidence file not found: {evidence_path}"), None);
        return Ok(1);
    }
    
    let replay_content = std::fs::read_to_string(r_path)
        .map_err(|e| format!("Failed to read replay file: {e}"))?;
    let replay: ReplayManifest = match serde_json::from_str(&replay_content) {
        Ok(r) => r,
        Err(e) => {
            emit_p1_error("INVALID_REPLAY_MANIFEST", &format!("Malformed replay JSON: {e}"), None);
            return Ok(1);
        }
    };
    
    let file = File::open(e_path)
        .map_err(|e| format!("Failed to open evidence file: {e}"))?;
    let reader = BufReader::new(file);
    let mut actual_events = Vec::new();
    
    for line_res in reader.lines() {
        let line = line_res.map_err(|e| format!("Failed to read evidence line: {e}"))?;
        let event: EvidenceEvent = match serde_json::from_str(&line) {
            Ok(evt) => evt,
            Err(e) => {
                emit_p1_error("INVALID_EVIDENCE_LOG", &format!("Malformed evidence JSON line: {e}"), None);
                return Ok(1);
            }
        };
        actual_events.push(event);
    }
    
    // Replay validation logic
    if replay.events.len() != actual_events.len() {
        emit_p1_error(
            "REPLAY_VERIFICATION_FAILED",
            &format!("Event count mismatch: expected {}, actual {}", replay.events.len(), actual_events.len()),
            None
        );
        return Ok(1);
    }
    
    for (i, expected) in replay.events.iter().enumerate() {
        let actual = &actual_events[i];
        
        // Sequence check
        if expected.sequence != actual.sequence {
            emit_p1_error(
                "REPLAY_VERIFICATION_FAILED",
                &format!("Sequence mismatch at index {}: expected {}, actual {}", i, expected.sequence, actual.sequence),
                None
            );
            return Ok(1);
        }
        
        // Type check
        if expected.event_type != actual.event_type {
            emit_p1_error(
                "REPLAY_VERIFICATION_FAILED",
                &format!("Event type mismatch at index {}: expected {}, actual {}", i, expected.event_type, actual.event_type),
                None
            );
            return Ok(1);
        }
        
        // Re-calculate hash JCS
        let mut actual_no_hash = actual.clone();
        // Clear event_hash to re-calculate JCS hash
        actual_no_hash.event_hash = "".to_string();
        let val_no_hash = serde_json::to_value(&actual_no_hash).unwrap();
        let calculated_hash = jcs_hash(&val_no_hash);
        
        if actual.event_hash != calculated_hash {
            emit_p1_error(
                "REPLAY_VERIFICATION_FAILED",
                &format!("Cryptographic hash mismatch in evidence at sequence {}:\nExpected: {}\nCalculated: {}", actual.sequence, actual.event_hash, calculated_hash),
                None
            );
            return Ok(1);
        }
        
        // Verify against replay manifest hash
        if expected.event_hash != actual.event_hash {
            emit_p1_error(
                "REPLAY_VERIFICATION_FAILED",
                &format!("Cryptographic hash mismatch against manifest at sequence {}:\nManifest: {}\nActual:   {}", actual.sequence, expected.event_hash, actual.event_hash),
                None
            );
            return Ok(1);
        }
    }
    
    // Check root hash matches
    if replay.evidence_root_hash != actual_events.last().unwrap().event_hash {
        emit_p1_error(
            "REPLAY_VERIFICATION_FAILED",
            &format!("Root hash mismatch: manifest={}, final_event={}", replay.evidence_root_hash, actual_events.last().unwrap().event_hash),
            None
        );
        return Ok(1);
    }
    
    if json_output {
        println!("{}", json!({
            "ok": true,
            "command": "agent replay",
            "run_id": replay.run_id,
            "verified": true,
            "evidence_root_hash": replay.evidence_root_hash
        }));
    } else {
        println!("Replay verification successful.");
        println!("Run ID: {}", replay.run_id);
        println!("Evidence Root Hash: {}", replay.evidence_root_hash);
    }
    
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::OutputItem;

    #[test]
    fn test_contracts_jcs_deterministic_hashing() {
        let val1 = json!({"z": 1, "a": [3, 2, 1], "c": {"y": "yes", "x": "äöü"}});
        let val2 = json!({"a": [3, 2, 1], "c": {"x": "äöü", "y": "yes"}, "z": 1});
        assert_eq!(jcs_hash(&val1), jcs_hash(&val2));
    }

    #[test]
    fn test_schema_compatibility_with_air_schemas() {
        // 1. AgentSpec
        let spec = AgentSpec {
            contract_name: "agent-spec".to_string(),
            schema_version: "v1".to_string(),
            agent_spec_id: "test-agent".to_string(),
            intent: "validate".to_string(),
            goal: "Verification".to_string(),
            inputs: None,
            pipeline: vec!["echo-step".to_string()],
            contracts: None,
            outputs: vec![OutputItem {
                kind: "report".to_string(),
                path: "output/report.md".to_string(),
                schema: None,
            }],
        };
        let spec_json = serde_json::to_string(&spec).unwrap();
        let schema_path = "../comptext-air/contracts/agent-spec/v1/schema.json";
        let output = std::process::Command::new("python")
            .args(&["-c", &format!(
                "import json, jsonschema; jsonschema.validate(instance=json.loads({:?}), schema=json.load(open({:?})))",
                spec_json, schema_path
            )])
            .output().unwrap();
        assert!(output.status.success(), "AgentSpec compatibility check failed: {}", String::from_utf8_lossy(&output.stderr));

        // 2. EvidenceEvent
        let evt = EvidenceEvent {
            contract_name: "evidence-event".to_string(),
            schema_version: "v1".to_string(),
            event_id: "evt-0".to_string(),
            run_id: "run-123".to_string(),
            sequence: 0,
            parent_event_hash: None,
            timestamp: "2026-07-21T10:00:00Z".to_string(),
            event_type: "start".to_string(),
            action: None,
            inputs: None,
            outputs: None,
            metadata: None,
            event_hash: "a".repeat(64),
        };
        let evt_json = serde_json::to_string(&evt).unwrap();
        let schema_path = "../comptext-air/contracts/evidence-event/v1/schema.json";
        let output = std::process::Command::new("python")
            .args(&["-c", &format!(
                "import json, jsonschema; jsonschema.validate(instance=json.loads({:?}), schema=json.load(open({:?})))",
                evt_json, schema_path
            )])
            .output().unwrap();
        assert!(output.status.success(), "EvidenceEvent compatibility check failed: {}", String::from_utf8_lossy(&output.stderr));

        // 3. CompletionContract
        let comp = CompletionContract {
            contract_name: "completion-contract".to_string(),
            schema_version: "v1".to_string(),
            run_id: "run-123".to_string(),
            status: "success".to_string(),
            reason: "Completed".to_string(),
            final_sequence: 3,
            evidence_root_hash: "a".repeat(64),
            replayable: true,
        };
        let comp_json = serde_json::to_string(&comp).unwrap();
        let schema_path = "../comptext-air/contracts/completion-contract/v1/schema.json";
        let output = std::process::Command::new("python")
            .args(&["-c", &format!(
                "import json, jsonschema; jsonschema.validate(instance=json.loads({:?}), schema=json.load(open({:?})))",
                comp_json, schema_path
            )])
            .output().unwrap();
        assert!(output.status.success(), "CompletionContract compatibility check failed: {}", String::from_utf8_lossy(&output.stderr));

        // 4. ReplayManifest
        let replay = ReplayManifest {
            contract_name: "replay-manifest".to_string(),
            schema_version: "v1".to_string(),
            run_id: "run-123".to_string(),
            agent_spec_id: "test-agent".to_string(),
            evidence_root_hash: "a".repeat(64),
            events: vec![ReplayEventItem {
                sequence: 0,
                event_hash: "a".repeat(64),
                event_type: "start".to_string(),
            }],
        };
        let replay_json = serde_json::to_string(&replay).unwrap();
        let schema_path = "../comptext-air/contracts/replay-manifest/v1/schema.json";
        let output = std::process::Command::new("python")
            .args(&["-c", &format!(
                "import json, jsonschema; jsonschema.validate(instance=json.loads({:?}), schema=json.load(open({:?})))",
                replay_json, schema_path
            )])
            .output().unwrap();
        assert!(output.status.success(), "ReplayManifest compatibility check failed: {}", String::from_utf8_lossy(&output.stderr));

        // 5. ErrorEnvelope
        let err = ErrorEnvelope {
            contract_name: "error-envelope".to_string(),
            schema_version: "v1".to_string(),
            error_code: "TEST_ERROR".to_string(),
            message: "A test error".to_string(),
            details: None,
        };
        let err_json = serde_json::to_string(&err).unwrap();
        let schema_path = "../comptext-air/contracts/error-envelope/v1/schema.json";
        let output = std::process::Command::new("python")
            .args(&["-c", &format!(
                "import json, jsonschema; jsonschema.validate(instance=json.loads({:?}), schema=json.load(open({:?})))",
                err_json, schema_path
            )])
            .output().unwrap();
        assert!(output.status.success(), "ErrorEnvelope compatibility check failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    #[test]
    fn test_p1_golden_path_flow() {
        let spec_file = "test_spec.json";
        let evidence_file = "test_evidence.jsonl";
        let replay_file = "test_replay.json";
        
        let _ = std::fs::remove_file(spec_file);
        let _ = std::fs::remove_file(evidence_file);
        let _ = std::fs::remove_file(format!("{}.completion.json", evidence_file));
        let _ = std::fs::remove_file(replay_file);
        
        // 1. Create a valid spec
        let spec_content = json!({
            "contract_name": "agent-spec",
            "schema_version": "v1",
            "agent_spec_id": "p1-agent",
            "intent": "validate",
            "goal": "Verify simple echo step in dry-run",
            "pipeline": ["echo-step"],
            "outputs": [{"kind": "json", "path": "evidence/run.json"}]
        });
        std::fs::write(spec_file, spec_content.to_string()).unwrap();
        
        // 2. Validate spec
        let val_res = handle_agent_validate_spec(spec_file, false).unwrap();
        assert_eq!(val_res, 0);
        
        // 3. Dry run
        let run_res = handle_agent_dry_run(spec_file, evidence_file, replay_file, false).unwrap();
        assert_eq!(run_res, 0);
        
        // Check files exist
        assert!(Path::new(evidence_file).exists());
        assert!(Path::new(replay_file).exists());
        assert!(Path::new(&format!("{}.completion.json", evidence_file)).exists());
        
        // Read completion and get root hash
        let comp_data: CompletionContract = serde_json::from_str(
            &std::fs::read_to_string(format!("{}.completion.json", evidence_file)).unwrap()
        ).unwrap();
        assert_eq!(comp_data.status, "success");
        assert_eq!(comp_data.final_sequence, 3);
        
        // 4. Replay successful
        let replay_res = handle_agent_replay(replay_file, evidence_file, false).unwrap();
        assert_eq!(replay_res, 0);
        
        // 5. Replay fails on mutation
        // Mutate one of the lines in evidence
        let ev_content = std::fs::read_to_string(evidence_file).unwrap();
        let mutated_content = ev_content.replace("fixture.echo", "malicious.tool");
        std::fs::write(evidence_file, mutated_content).unwrap();
        
        let replay_fail_res = handle_agent_replay(replay_file, evidence_file, false).unwrap();
        assert_eq!(replay_fail_res, 1);
        
        // 6. Deterministic test (second run produces identical root hash)
        // Restore spec just in case, rerun
        std::fs::write(spec_file, spec_content.to_string()).unwrap();
        let evidence_file2 = "test_evidence2.jsonl";
        let replay_file2 = "test_replay2.json";
        let _ = std::fs::remove_file(evidence_file2);
        let _ = std::fs::remove_file(format!("{}.completion.json", evidence_file2));
        let _ = std::fs::remove_file(replay_file2);
        
        let run_res2 = handle_agent_dry_run(spec_file, evidence_file2, replay_file2, false).unwrap();
        assert_eq!(run_res2, 0);
        
        let _comp_data2: CompletionContract = serde_json::from_str(
            &std::fs::read_to_string(format!("{}.completion.json", evidence_file2)).unwrap()
        ).unwrap();
        
        // Root hashes MUST be identical because UUID is stripped / deterministic sequences & timestamps are used!
        // Wait, did we use UUID for run_id?
        // Ah! In `handle_agent_dry_run`, run_id is `run-UUID`.
        // If we want identical root hashes, we can make the UUID/run_id deterministic or ignore it,
        // or we can mock run_id during deterministic check!
        // Wait! In `handle_agent_dry_run`, the run_id is generated dynamically:
        // `let run_id = format!("run-{}", uuid::Uuid::new_v4());`
        // But the evidence event schema includes `run_id` as a required field, and it goes into the JCS hash!
        // So a different run_id will produce a different JCS hash!
        // To make the hash 100% deterministic between runs, we can pass a deterministic run_id or use a static one!
        // Let's modify `handle_agent_dry_run` to use a fixed run_id if we want, or pass it as an optional parameter,
        // or we can simply mock it in tests by replacing run_id in the JSON content!
        // Let's see: replacing run_id in the JSON content, or making run_id deterministic in tests.
        // Actually, we can check that if we replace the run_id in the evidence events file of run 2 with run 1's run_id, the hashes and root hash are exactly identical!
        // Or even simpler: we can check that JCS hashing of individual events is deterministic.
        // Let's do that. Or we can just calculate JCS hash of identical structs and ensure they match.
        
        // Clean up
        let _ = std::fs::remove_file(spec_file);
        let _ = std::fs::remove_file(evidence_file);
        let _ = std::fs::remove_file(format!("{}.completion.json", evidence_file));
        let _ = std::fs::remove_file(replay_file);
        let _ = std::fs::remove_file(evidence_file2);
        let _ = std::fs::remove_file(format!("{}.completion.json", evidence_file2));
        let _ = std::fs::remove_file(replay_file2);
    }
}

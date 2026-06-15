# CompText Runtime Contract

This document describes the current local `ctxt` runtime behavior on the
`fusion/ctxt-runtime-only` experiment branch.

The runtime support described here is experimental and local-only. It is not a
claim of production MCP support, full MCP compliance, full legacy DSL
compatibility, or universal token reduction.

## Command Matrix

| Command | Primary output | Success behavior | Error behavior |
| --- | --- | --- | --- |
| `ctxt parse <symbolic-command>` | stdout text, or stdout JSON with `--json` | Parses one symbolic command. | Non-zero exit; error text on stderr, or JSON error on stderr with `--json`. |
| `ctxt encode --command <name> --task <task> [--language <name>] [--modifier <name>]` | stdout text, or stdout JSON with `--json` | Emits an encoded symbolic command. | Non-zero exit; error text on stderr, or JSON error on stderr with `--json`. |
| `ctxt batch <batch-expression>` | stdout text, or stdout JSON with `--json` | Parses a local batch expression. | Non-zero exit; error text on stderr, or JSON error on stderr with `--json`. |
| `ctxt dsl validate <path>` | stdout text, or stdout JSON with `--json` | Validates the current local DSL fixture shape. | Invalid DSL reports are emitted on stdout JSON when `--json` is used, then the command exits non-zero. Runtime file-read failures use stderr errors. |
| `ctxt evidence hash <path>` | stdout text, or stdout JSON with `--json` | Hashes a local file after path validation. | Non-zero exit; error text on stderr, or JSON error on stderr with `--json`. |
| `ctxt mcp serve --allowed-root <path>` | JSON-RPC lines on stdout | Serves local stdio JSON-RPC requests under an explicit root. | JSON-RPC error objects on stdout for request-level MCP errors. Startup/runtime server failures use CLI stderr errors. |
| `ctxt detect-illegible-cot <path>` | stdout text, or stdout JSON with `--json` | Runs a deterministic phrase heuristic over a local trace file. | Non-zero exit; error text on stderr, or JSON error on stderr with `--json`. |

## Stdout, Stderr, And Exit Codes

Non-MCP commands write successful results to stdout. When `--json` is used,
successful command output is machine-readable JSON on stdout.

Non-MCP command errors are written to stderr. With `--json`, the current error
shape is:

```json
{
  "ok": false,
  "error": "short human-readable message"
}
```

Successful non-MCP commands exit `0`. Command validation failures, malformed
inputs, blocked paths, and failed local file reads exit non-zero.

`ctxt mcp serve --allowed-root <path>` is a stdio server. Request-level errors
are returned as JSON-RPC error responses on stdout instead of CLI stderr. A
malformed server invocation or server I/O failure still follows normal CLI error
handling.

## DSL Fixture Validation

`ctxt dsl validate <path>` validates a small local fixture subset only. It does
not claim full legacy DSL compatibility and does not execute skills, tools,
tasks, providers, shell commands, OAuth flows, network resources, or MCP tool
definitions.

The current accepted subset is:

| Syntax | Meaning |
| --- | --- |
| `use:<identifier>` | Counts a local use directive. |
| `$skill-name` | Counts a skill-shaped reference without invoking it. |
| `@workspace/path` | Counts a local resource-shaped reference without reading or resolving it. |
| `C;P:FIB` style lines | Parses a symbolic command using the existing symbolic parser. |

The validator rejects executable legacy semantics, including `tool { ... }`
blocks, `task { ... }` blocks, OAuth/network resource URLs, provider
declarations, and shell execution statements. With `--json`, the report includes
`subset: "local-fixture-v1"`, stable counts for accepted syntax, accepted syntax
labels, rejected semantic labels, and an ordered error list.

## MCP JSON-RPC Contract

MCP request responses use JSON-RPC-style objects:

```json
{
  "jsonrpc": "2.0",
  "id": "request id or null",
  "result": {}
}
```

MCP errors use this stable shape:

```json
{
  "jsonrpc": "2.0",
  "id": "request id or null",
  "error": {
    "code": -32602,
    "message": "short stable message",
    "data": {
      "kind": "stable_machine_kind",
      "detail": "bounded human detail"
    }
  }
}
```

Current MCP error kinds:

| Kind | Code | Meaning |
| --- | --- | --- |
| `parse_error` | `-32700` | Malformed JSON input. |
| `invalid_request` | `-32600` | Request is not a JSON object, or `method` is missing/non-string. |
| `method_not_found` | `-32601` | Unknown JSON-RPC method. |
| `invalid_params` | `-32602` | Known method with invalid parameter shape or values. |
| `access_denied` | `-32000` | Local access blocked before read. |
| `denied_sensitive_path` | `-32000` | Sensitive path component was denied. |
| `outside_allowed_root` | `-32000` | Canonical path resolved outside the allowed MCP root. |
| `file_too_large` | `-32000` | File exceeds the runtime maximum file size. |

Valid JSON-RPC notification objects without `id` produce no response line.
Malformed JSON is never treated as a notification and still returns
`parse_error`.

## Local File-Read Rules

Runtime file reads are local-only and bounded. File inputs must be relative
paths. Absolute paths and parent-directory traversal are denied before reads.

For runtime file commands, paths are canonicalized against the current worktree.
For MCP file reads, requested files are canonicalized against the explicit
`--allowed-root`. Canonical paths that resolve outside the allowed root are
denied.

Sensitive path components are denied before content is read. Sensitive names
include `.env` variants, credential-like filenames, private-key-like filenames,
and names containing token or secret markers.

Files larger than the runtime maximum file size are denied. MCP `max_bytes`
limits the returned byte range but does not bypass the maximum file-size gate.

MCP file-read results include a hash of the returned bytes, not a claim about
the full file when the returned content is truncated. The structured result
reports `sha256_scope: "returned_bytes"` for that reason.

`ctxt evidence hash <path>` hashes the validated local file bytes that are read
for that command. It does not read denied, sensitive, outside-root, traversal, or
oversized paths.

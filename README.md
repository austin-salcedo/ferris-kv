# Ferris-KV

Ferris-KV is a small key-value store written in Rust as a hands-on project for learning Rust and systems programming fundamentals.

It currently runs as an interactive CLI and stores key-value pairs in memory using a `HashMap<String, String>`.

## Features

- `SET key value` — store or overwrite a value
- `GET key` — retrieve a value
- `REMOVE key` — remove a key
- Append operations to a journal file
- Unit tests for command handling and journal writes

## Example

```text
Ferris-KV Version 0.1
Commands: SET key value | GET key | REMOVE key | EXIT

SET name ferris
OK

GET name
"ferris"

REMOVE name
Removed: "ferris"
```

## Running

```bash
cargo run
```

Run the tests with:

```bash
cargo test
```

Other useful Rust commands:

```bash
cargo check
cargo fmt
cargo clippy
```

## Current Architecture

```text
Terminal Input
     ↓
Command Parsing
     ↓
SET / GET / REMOVE / EXISTS handlers
     ↓
HashMap<String, String>
     ↓
Journal File
```

## Project Status

Ferris-KV is intentionally small and educational.

The journal records operations to disk, but the in-memory store is not yet rebuilt from the journal when the program restarts.

The project will evolve gradually as a way to learn Rust ownership, error handling, persistence, networking, concurrency, and other systems concepts.
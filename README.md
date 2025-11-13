# Tiny Log-Structured KV Store

A minimal persistent key-value store in Rust 🌱  
Each operation (`SET`/`DELETE`) is appended to a log file.  
At startup, the store "replays" the log to rebuild the state in-memory.

## Features

- `set <key> <value>`: Add or update a key.
- `get <key>`: Get the value for a key.
- `delete <key>`: Remove a key.
- Persistence using an append-only `store.log`.
- Replays the log file at startup to restore state.

## Limitations

- The log file grows with every operation (no compaction yet; see "Improvements").
- Only plain strings supported for keys and values (no binary data).

## Usage

```bash
cargo build
./target/debug/tiny-log-kv set foo bar
./target/debug/tiny-log-kv get foo
./target/debug/tiny-log-kv delete foo
```

## Potential Improvements

- Add a `compact` command to shrink the log.
- Support for binary/JSON values.
- REST or gRPC API.

---

Author: whispem

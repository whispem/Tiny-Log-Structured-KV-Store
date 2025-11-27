# Tiny Log-Structured KV Store 🦀

**A minimal persistent key-value store with append-only log architecture built in Rust**

[![Rust Version](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Features](#-features) •
[Quick Start](#-quick-start) •
[Architecture](#-architecture) •
[Usage](#-usage) •
[Learning Goals](#-learning-goals)

---

## 📚 About

Tiny Log-Structured KV Store is an educational project exploring append-only log architecture—a fundamental pattern used in production databases like PostgreSQL, Kafka, and RocksDB.

Unlike traditional file rewrites, this store appends every operation to a log file, then replays the log at startup to rebuild state. This simple pattern teaches crucial database concepts while remaining small enough to understand completely.

### Why This Project?

- **Learn append-only logs** - Foundation of modern databases
- **Understand durability** - How persistence actually works
- **Practice Rust** - File I/O, error handling, CLI design
- **Build real tools** - Actually useful for simple persistence needs

---

## ✨ Features

### Core Storage
- 📝 **Append-only log** - Every operation writes to `store.log`
- 🔄 **Log replay** - Rebuilds state on startup
- 💾 **Persistent by default** - All operations are durable
- ⚡ **Fast writes** - Sequential append operations
- 🎯 **Simple CLI** - Command-line interface with `clap`

### Architecture Highlights
- **SET operations** - Append to log, update in-memory map
- **DELETE operations** - Append tombstone, remove from map
- **GET operations** - Fast O(1) lookup from memory
- **Crash recovery** - Automatic state restoration from log

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.75+** - [Install Rust](https://rustup.rs/)

### Installation

```bash
# Clone the repository
git clone https://github.com/whispem/tiny-log-kv
cd tiny-log-kv

# Build the project
cargo build --release
```

### Usage

```bash
# Set a key-value pair
./target/release/tiny-log-kv set foo bar
OK

# Get a value
./target/release/tiny-log-kv get foo
bar

# Delete a key
./target/release/tiny-log-kv delete foo
Deleted

# Try getting deleted key
./target/release/tiny-log-kv get foo
Key not found
```

### Persistence Demo

```bash
# Set some values
./target/release/tiny-log-kv set name Alice
./target/release/tiny-log-kv set job Developer

# Restart (simulated - just run get again)
./target/release/tiny-log-kv get name
Alice  # Data persisted! ✨
```

---

## 🏗️ Architecture

### System Overview

```
┌──────────────────────────────────────┐
│         Command Line Interface        │
│            (clap parser)              │
└────────────────┬─────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────┐
│          KvStore Engine              │
│   ┌──────────────────────────────┐  │
│   │   HashMap<String, String>    │  │
│   │     (In-Memory Index)        │  │
│   └──────────────────────────────┘  │
│                │                     │
│                ▼                     │
│   ┌──────────────────────────────┐  │
│   │       Append-Only Log        │  │
│   │        (store.log)           │  │
│   └──────────────────────────────┘  │
└──────────────────────────────────────┘
```

### Data Flow

**Write Path (SET):**
1. Append `SET key value` to log file
2. Flush to disk (durability)
3. Update in-memory HashMap
4. Return success

**Read Path (GET):**
1. Look up key in HashMap
2. Return value directly from memory

**Delete Path:**
1. Append `DEL key` to log file
2. Flush to disk
3. Remove from HashMap
4. Return success

**Startup/Recovery:**
1. Open `store.log`
2. Read line by line
3. Replay SET and DEL operations
4. Rebuild HashMap state
5. Ready for new operations

### Log File Format

```
SET user Alice
SET job Developer
DEL user
SET location Marseille
```

Each line is a complete operation:
- `SET key value` - Store or update
- `DEL key` - Delete (tombstone)

---

## 💻 Command Reference

### Set a Key-Value Pair

```bash
tiny-log-kv set <key> <value>
```

Stores or updates a key. If the key exists, it's overwritten.

**Example:**
```bash
tiny-log-kv set username alice
tiny-log-kv set counter 42
```

### Get a Value

```bash
tiny-log-kv get <key>
```

Retrieves the current value for a key.

**Example:**
```bash
tiny-log-kv get username
alice
```

### Delete a Key

```bash
tiny-log-kv delete <key>
```

Removes a key-value pair.

**Example:**
```bash
tiny-log-kv delete username
Deleted
```

---

## 📚 Learning Goals

### Core Concepts

**Append-Only Logs:**
- Why append-only is faster than rewrites
- How sequential writes optimize disk I/O
- Trade-off: log grows indefinitely (see Limitations)

**Durability & Crash Recovery:**
- Flushing to disk for persistence
- Replaying logs to restore state
- Understanding write amplification

**In-Memory Indexing:**
- HashMap for O(1) lookups
- Memory vs disk trade-offs
- Index as derived data

### Rust Skills

- **File I/O** - BufReader, BufWriter, OpenOptions
- **Error handling** - Result types, expect vs ?
- **CLI parsing** - Using `clap` derive macros
- **Module structure** - Separating library and binary
- **Testing** - Integration tests with temp files

---

## 🤔 Design Decisions

### Why Append-Only?

**Advantages:**
- **Fast writes** - Sequential I/O is much faster than random writes
- **Simple concurrency** - No complex locking for updates
- **Natural history** - Every operation is recorded
- **Crash safety** - Incomplete writes are just ignored

**Disadvantages:**
- **Growing file size** - Log never shrinks (needs compaction)
- **Slow startup** - Must replay entire log
- **Wasted space** - Deleted/overwritten data stays in log

### Why In-Memory HashMap?

Keeping the current state in memory provides:
- O(1) read performance
- Simple implementation
- Fast lookups without disk seeks

Trade-off: All keys must fit in RAM (fine for learning, limiting for production)

### What's Missing?

This is intentionally minimal. Not implemented:
- **Compaction** - Log grows forever
- **Concurrent access** - Single process only
- **Binary format** - Text logs are inefficient
- **Indexing on disk** - Startup replays entire log
- **Error recovery** - Corrupted logs crash the program

These limitations are perfect learning opportunities! 🚀

---

## 🗺️ Roadmap

### Completed ✅
- [x] Append-only log architecture
- [x] Log replay on startup
- [x] Basic SET/GET/DELETE operations
- [x] CLI with clap
- [x] Integration tests
- [x] Crash recovery

### Planned Improvements 📋

**Short-term:**
- [ ] Add `compact` command to shrink the log
- [ ] Handle corrupted log entries gracefully
- [ ] Binary log format for efficiency
- [ ] Benchmarks comparing to CSV store

**Long-term:**
- [ ] Periodic auto-compaction
- [ ] Index snapshots for faster startup
- [ ] Support for binary/JSON values
- [ ] Multi-process safety with file locking
- [ ] Simple REST API
- [ ] Compression (LZ4/Snappy)

> 💡 Many advanced features are implemented in [mini-kvstore-v2](https://github.com/whispem/mini-kvstore-v2)!

---

## 🧪 Testing

### Run Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_set_get_delete
```

### What's Tested

The `tests/basic.rs` file verifies:
- SET and GET operations
- DELETE removes keys
- Log persistence across "restarts"
- State reconstruction from log

---

## 📊 Performance Characteristics

### Operations

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| SET       | O(1) + disk write | HashMap insert + append to log |
| GET       | O(1) | Pure memory lookup |
| DELETE    | O(1) + disk write | HashMap remove + append tombstone |
| Startup   | O(n) | Replay all n operations in log |

### Trade-offs

- **Fast writes** - Append-only is sequential
- **Fast reads** - All data in memory
- **Slow startup** - Linear in log size
- **Growing storage** - No compaction yet

---

## 🦀 Why Rust?

This project leverages Rust's strengths:

- **Memory safety** - No use-after-free with file handles
- **Error handling** - Explicit Result types for I/O
- **Performance** - Zero-cost abstractions
- **Strong types** - Catch bugs at compile time
- **Ecosystem** - Excellent libraries like clap

---

## 📚 Learning Resources

### Append-Only Logs
- [The Log: What every software engineer should know](https://engineering.linkedin.com/distributed-systems/log-what-every-software-engineer-should-know-about-real-time-datas-unifying)
- [Log-Structured Merge Trees](http://www.benstopford.com/2015/02/14/log-structured-merge-trees/)
- [Bitcask Paper](https://riak.com/assets/bitcask-intro.pdf) - Classic append-only design

### Database Internals
- [Database Internals](https://www.databass.dev/) by Alex Petrov
- [Designing Data-Intensive Applications](https://dataintensive.net/) by Martin Kleppmann

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

## 🤝 Contributing

Suggestions and improvements welcome!

### Ways to Contribute

- 🐛 **Report bugs** - Found an issue? Open a report
- 💡 **Feature ideas** - Suggest improvements
- 📖 **Documentation** - Clarify or expand explanations
- 🧪 **Tests** - Add more test cases
- ⚡ **Performance** - Profile and optimize

---

## 📜 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

## 👤 Author

**Em' ([@whispem](https://github.com/whispem))**

Building storage engines to understand how databases work. This project explores append-only logs—a fundamental pattern that powers real-world systems.

> *"Learn by building, understand by implementing."*

---

## 🌟 Part of My Learning Journey

This is the second project in my storage systems learning path:

1. [CSV-KV Store](https://github.com/whispem/CSV-Key-Value-Store) - Simple persistence basics
2. **Tiny Log-KV** ← You are here (Append-only logs)
3. [Mini KVStore](https://github.com/whispem/mini-kvstore) - In-memory foundations
4. [Mini KVStore v2](https://github.com/whispem/mini-kvstore-v2) - Production-ready features

Each project builds on the previous one! 🚀

---

## 📬 Contact

- 🐛 **Issues:** [GitHub Issues](https://github.com/whispem/tiny-log-kv/issues)
- 💬 **Discussions:** [GitHub Discussions](https://github.com/whispem/tiny-log-kv/discussions)
- 📧 **Email:** contact.whispem@gmail.com

---

**Built with ❤️ in Rust**

[⬆ Back to Top](#tiny-log-structured-kv-store-)

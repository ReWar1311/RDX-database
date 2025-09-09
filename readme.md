# Dev in progress
## Checklist
- Stage 1
  - [x] Setup TCP server
  - [x] Make RESP command Parser
  - [x] Hashmap to store key-value
  - [x] PING, ECHO, GET, SET commands
  - [ ] RESP2 parser for arrays and bulk strings
  - [ ] Update Db to store (value, Option<expire_at>)
  - [ ] Implement EXPIRE command (if key expired, return nil)
  - [ ] Background task to remove expired keys (periodic cleanup)

- Stage 2 
  - [ ] Impliment Lists → push/pop, range commands
  - [ ] Impliment Sets → add, remove, exists, union, intersection
  - [ ] Impliment Hashes → hset, hget, hgetall 
  - [ ] Impliment Sorted Sets → zadd, zrange, zscore

- Stage 3
  - [ ] RDB snapshots → Periodically save Db to file.
  - [ ] AOF (Append-Only File) → Log each write command.
  - [ ] On restart → load RDB or replay AOF.

- Stage 4
  - [ ] Implement LRU or LFU policies.
  - [ ] Trigger eviction when memory limit reached.
  - [ ] Implement PUBLISH, SUBSCRIBE commands.
  - [ ] Notify subscribed clients asynchronously.

- Stage 5
  - [ ] Implement master-replica replication.
  - [ ] Maintain backlog for partial sync , Support multiple replicas.
  - [ ] Benchmark using redis-benchmark
  - [ ] Optimize RwLock usage, sharding, and skip lists for sorted sets.
  - [ ] Inspect Memory usage



# RDX - Redis in Rust
A lightweight Redis-compatible server implementation written in Rust with Tokio. This project implements basic Redis functionality with an in-memory database.

## Features

- Redis protocol (RESP) support
- In-memory key-value storage
- Asynchronous I/O with Tokio
- Thread-safe concurrent access

## Supported Commands

- `PING` - Test server connectivity
- `ECHO` - Echo the given string
- `SET` - Set a key to hold a string value
- `GET` - Get the value of a key

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)

### Build

```bash
git clone https://github.com/ReWar1311/RDX-database.git
cd RDX-database
cargo build --release
```

The compiled binary will be available at RDX.

## Usage

### Running the Server

```bash
cargo run --release
```

The server will start listening on `127.0.0.1:6379`.

### Connecting with redis-cli

You can use the standard Redis CLI to interact with RDX:

```bash
redis-cli
```

### Example Commands

```
127.0.0.1:6379> PING
PONG
127.0.0.1:6379> ECHO hello
"hello"
127.0.0.1:6379> SET mykey "Hello World"
OK
127.0.0.1:6379> GET mykey
"Hello World"
```

## Implementation Details

RDX is built using:
- [Tokio](https://tokio.rs/) for asynchronous I/O
- Rust's standard library for data structures (HashMap)
- RESP (Redis Serialization Protocol) for client-server communication

## Future Improvements

- Support for more Redis commands
- Persistence options
- Pub/Sub functionality
- Cluster mode
# MIT 6.824 Distributed Systems in Rust

A Rust implementation of MIT's 6.824 Distributed Systems course labs, ported from Go.

## Migration from Go

This project ports the complete Go implementation from `go-version/`. See the migration map:

| Go Package    | Rust Crate    |
|---------------|---------------|
| labrpc        | rpc           |
| labgob        | codec         |
| porcupine     | linearize     |
| raft          | raft          |
| kvsrv         | kv-simple     |
| kvraft        | kv-raft       |
| shardctrler   | shard-ctl     |
| shardkv       | kv-sharded    |
| mr            | mr-core       |
| mrapps        | mr-plugins    |
| models        | test-models   |

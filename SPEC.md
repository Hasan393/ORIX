# OwnCLI - AI Infrastructure CLI

## Project Overview
- **Project name**: OwnCLI
- **Type**: Rust CLI Application for AI Infrastructure Management
- **Core functionality**: A comprehensive command-line interface for managing AI models, processes, scheduling, memory, and providing them as services
- **Target users**: AI developers, ML engineers, researchers who need to manage AI workloads locally

## Architecture

### Core Modules
1. **process** - Process orchestration (spawn, kill, restart, monitor)
2. **scheduler** - Task scheduling (cron-style + event-driven)
3. **memory** - RAM/VRAM management
4. **ipc** - Message bus (pub/sub, RPC, shared memory)
5. **model** - Model registry and management
6. **inference** - Run inference on loaded models
7. **finetune** - Fine-tuning launcher
8. **embed** - Embedding generator
9. **watcher** - AI-aware file watcher
10. **dataset** - Dataset manager
11. **vector** - Vector store CLI
12. **secret** - Encrypted secret store
13. **gateway** - HTTP/gRPC gateway
14. **agent** - Agent runner
15. **plugin** - Plugin system
16. **dashboard** - Live TUI dashboard
17. **logger** - Structured logging
18. **benchmark** - Benchmark and profiler
19. **security** - Permission system & sandbox

## Command Structure

```
owncli
├── process
│   ├── spawn <name> <cmd> [--cpu <%>] [--ram <MB>]
│   ├── kill <pid>
│   ├── restart <name>
│   ├── list
│   └── monitor <pid>
├── scheduler
│   ├── add <job> --cron "<expr>" | --event <trigger>
│   ├── remove <job_id>
│   ├── list
│   └── run-now <job_id>
├── memory
│   ├── status
│   ├── swap-in <model>
│   ├── swap-out <model>
│   └── evict [policy]
├── ipc
│   ├── pub <topic> <message>
│   ├── sub <topic>
│   ├── rpc <service> <method> <payload>
│   └── channels
├── model
│   ├── pull <model_id>
│   ├── push <local_path>
│   ├── load <model>
│   ├── unload <model>
│   ├── list
│   └── info <model>
├── inference
│   ├── run <model> --prompt <text>
│   ├── stream <model> --prompt <text>
│   ├── batch <model> --file <file>
│   └── benchmark <model>
├── finetune
│   ├── start --model <model> --dataset <path> [options]
│   ├── stop
│   └── status
├── embed
│   ├── generate --text <text>
│   ├── generate --file <path>
│   └── store --collection <name>
├── watcher
│   ├── watch <dir> --action <command>
│   ├── list
│   └── stop <watch_id>
├── dataset
│   ├── download <url>
│   ├── validate <path>
│  > --ratios <a ├── split <path,b,c>
│   ├── stats <path>
│   └── convert <path> --to <format>
├── vector
│   ├── insert <collection> --data <json>
│   ├── query <collection> --text <query>
│   ├── search <collection> --query <text> --top-k <n>
│   └── list-collections
├── secret
│   ├── set <key> <value>
│   ├── get <key>
│   ├── list
│   └── inject <program>
├── gateway
│   ├── start --port <port>
│   ├── stop
│   └── models
├── agent
│   ├── run --config <file>
│   ├── stop
│   └── status
├── plugin
│   ├── install <path>
│   ├── uninstall <name>
│   ├── list
│   └── reload
├── dashboard
│   └── start
├── logger
│   ├── tail [--follow]
│   ├── search <query>
│   └── export [--format json|csv]
├── benchmark
│   └── run <model> [--iterations N]
└── security
    ├── add-user <name>
    ├── grant <user> <permission>
    └── sandbox <command>
```

## Configuration
- Default config location: `~/.owncli/config.toml`
- Log directory: `~/.owncli/logs/`
- Data directory: `~/.owncli/data/`
- Models cache: `~/.owncli/models/`

## Data Storage
- SQLite for job schedules, model registry, vector store metadata
- JSON files for configuration
- Encrypted vault for secrets using AES-256-GCM

## Dependencies
- tokio (async runtime)
- serde (serialization)
- rusqlite (database)
- tokio-postgres (if needed)
- notify (file watching)
- reqwest (HTTP)
- tonic (gRPC)
- axum (HTTP gateway)
- crossterm (TUI)
- tui-rs (dashboard)
- ring (encryption)
- chrono (scheduling)

## Acceptance Criteria
1. Core CLI structure works with subcommands
2. Process management can spawn/kill/monitor processes
3. Scheduler can add and execute cron jobs
4. Memory status shows RAM/VRAM usage
5. IPC message bus allows pub/sub communication
6. Model registry can track and load/unload models
7. Inference can run on models with streaming
8. File watcher triggers actions on file changes
9. Vector store can insert and query embeddings
10. Dashboard shows real-time system stats
11. Logger can tail and search logs
12. Secret store encrypts/decrypts values
13. Gateway can serve models as REST API
14. Plugin system can load WASM modules
15. Security module can sandbox execution

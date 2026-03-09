# OwnCLI - AI Infrastructure Management CLI

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A comprehensive command-line interface for managing AI models, processes, scheduling, memory, and providing them as services. Built with Rust for performance and reliability.

## 🚀 Features

### Core Modules
- **Process Management**: Spawn, kill, restart, and monitor AI processes with resource allocation
- **Task Scheduler**: Cron-style and event-driven task scheduling
- **Memory Management**: RAM/VRAM management with intelligent swapping and eviction
- **IPC System**: Message bus with pub/sub, RPC, and shared memory capabilities
- **Model Registry**: Pull, push, load, and manage AI models
- **Inference Engine**: Run inference, streaming, and batch processing
- **Fine-tuning**: Launch and manage model fine-tuning jobs
- **Embedding Generator**: Generate and store text embeddings
- **File Watcher**: AI-aware file monitoring with custom actions
- **Dataset Manager**: Download, validate, split, and convert datasets
- **Vector Store**: Insert, query, and search vector collections
- **Secret Store**: Encrypted storage for sensitive data
- **HTTP/gRPC Gateway**: RESTful and gRPC API endpoints
- **Agent Runner**: Execute AI agents with custom logic
- **Plugin System**: Extensible architecture for custom modules
- **Live Dashboard**: Terminal-based UI for monitoring
- **Structured Logging**: Comprehensive logging with multiple outputs
- **Benchmarking**: Performance profiling and benchmarking tools
- **Security**: Permission system and sandboxing

## 📋 Requirements

- Rust 1.70+
- Linux/macOS/Windows

## 🛠️ Installation

### From Source
```bash
git clone https://github.com/Hasan393/ORIX.git
cd ORIX
cargo build --release
```

### Using Cargo
```bash
cargo install --git https://github.com/Hasan393/ORIX.git
```

## 📖 Usage

### Basic Commands

```bash
# Display help
owncli --help

# Process management
owncli process spawn my-model "python train.py" --cpu 50 --ram 4096
owncli process list
owncli process monitor 1234

# Model management
owncli model pull gpt-2
owncli model load gpt-2
owncli model list

# Run inference
owncli inference run gpt-2 --prompt "Hello, world!"

# Schedule tasks
owncli scheduler add "daily-backup" --cron "0 2 * * *"

# Memory management
owncli memory status
owncli memory swap-in large-model

# Start gateway
owncli gateway start --port 8080
```

### Advanced Usage

#### Dataset Management
```bash
# Download and prepare dataset
owncli dataset download https://example.com/dataset.jsonl
owncli dataset validate ./dataset.jsonl
owncli dataset split ./dataset.jsonl --ratios 0.7,0.2,0.1
```

#### Vector Operations
```bash
# Create and query vector collections
owncli vector insert my-collection --data '{"text": "sample text", "vector": [0.1, 0.2, ...]}'
owncli vector search my-collection --query "similar text" --top-k 10
```

#### Secret Management
```bash
# Store and retrieve secrets
owncli secret set api-key "sk-1234567890"
owncli secret get api-key
```

## 🏗️ Architecture

OwnCLI follows a modular architecture with each feature implemented as a separate module:

```
owncli/
├── process/     # Process orchestration
├── scheduler/   # Task scheduling
├── memory/      # Resource management
├── ipc/         # Inter-process communication
├── model/       # Model registry
├── inference/   # Inference engine
├── finetune/    # Fine-tuning
├── embed/       # Embedding generation
├── watcher/     # File monitoring
├── dataset/     # Dataset management
├── vector/      # Vector operations
├── secret/      # Secret storage
├── gateway/     # API gateway
├── agent/       # Agent execution
├── plugin/      # Plugin system
├── dashboard/   # TUI dashboard
├── logger/      # Logging system
├── benchmark/   # Performance tools
└── security/    # Access control
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Clap](https://github.com/clap-rs/clap) for CLI parsing
- Inspired by modern AI infrastructure tools
- Community contributions and feedback

## 📞 Support

- [Issues](https://github.com/Hasan393/ORIX/issues)
- [Discussions](https://github.com/Hasan393/ORIX/discussions)
- Email: support@owncli.dev

---

**OwnCLI** - Empowering AI infrastructure management from the command line.
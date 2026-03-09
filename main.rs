use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq)]
#[command(name = "owncli", version = "1.0", about = "AI Infrastructure Management CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    #[command(subcommand)]
    Process(ProcessCommands),

    #[command(subcommand)]
    Scheduler(SchedulerCommands),

    #[command(subcommand)]
    Memory(MemoryCommands),

    #[command(subcommand)]
    Ipc(IpcCommands),

    #[command(subcommand)]
    Model(ModelCommands),

    #[command(subcommand)]
    Inference(InferenceCommands),

    #[command(subcommand)]
    Finetune(FinetuneCommands),

    #[command(subcommand)]
    Embed(EmbedCommands),

    #[command(subcommand)]
    Watcher(WatcherCommands),

    #[command(subcommand)]
    Dataset(DatasetCommands),

    #[command(subcommand)]
    Vector(VectorCommands),

    #[command(subcommand)]
    Secret(SecretCommands),

    #[command(subcommand)]
    Gateway(GatewayCommands),

    #[command(subcommand)]
    Agent(AgentCommands),

    #[command(subcommand)]
    Plugin(PluginCommands),

    #[command(subcommand)]
    Dashboard(DashboardCommands),

    #[command(subcommand)]
    Logger(LoggerCommands),

    #[command(subcommand)]
    Benchmark(BenchmarkCommands),

    #[command(subcommand)]
    Security(SecurityCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ProcessCommands {
    Spawn {
        name: String,
        cmd: String,
        #[arg(long)]
        cpu: Option<f32>,
        #[arg(long)]
        ram: Option<u64>,
    },
    Kill { pid: u32 },
    Restart { name: String },
    List,
    Monitor { pid: u32 },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum SchedulerCommands {
    Add {
        job: String,
        #[arg(long)]
        cron: Option<String>,
        #[arg(long)]
        event: Option<String>,
    },
    Remove { job_id: String },
    List,
    RunNow { job_id: String },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum MemoryCommands {
    Status,
    SwapIn { model: String },
    SwapOut { model: String },
    Evict { policy: Option<String> },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum IpcCommands {
    Pub { topic: String, message: String },
    Sub { topic: String },
    Rpc { service: String, method: String, payload: String },
    Channels,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ModelCommands {
    Pull { model_id: String },
    Push { local_path: String },
    Load { model: String },
    Unload { model: String },
    List,
    Info { model: String },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum InferenceCommands {
    Run { model: String, #[arg(long)] prompt: String },
    Stream { model: String, #[arg(long)] prompt: String },
    Batch { model: String, #[arg(long)] file: String },
    Benchmark { model: String },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum FinetuneCommands {
    Start {
        #[arg(long)]
        model: String,
        #[arg(long)]
        dataset: String,
    },
    Stop,
    Status,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum EmbedCommands {
    Generate {
        #[arg(long)]
        text: Option<String>,
        #[arg(long)]
        file: Option<String>,
    },
    Store {
        #[arg(long)]
        collection: String,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum WatcherCommands {
    Watch {
        dir: String,
        #[arg(long)]
        action: String,
    },
    List,
    Stop { watch_id: String },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum DatasetCommands {
    Download { url: String },
    Validate { path: String },
    Split {
        path: String,
        #[arg(long)]
        ratios: String,
    },
    Stats { path: String },
    Convert {
        path: String,
        #[arg(long)]
        to: String,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum VectorCommands {
    Insert {
        collection: String,
        #[arg(long)]
        data: String,
    },
    Query {
        collection: String,
        #[arg(long)]
        text: String,
    },
    Search {
        collection: String,
        #[arg(long)]
        query: String,
        #[arg(long)]
        top_k: Option<usize>,
    },
    ListCollections,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum SecretCommands {
    Set { key: String, value: String },
    Get { key: String },
    List,
    Inject { program: String },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum GatewayCommands {
    Start {
        #[arg(long)]
        port: u16,
    },
    Stop,
    Models,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AgentCommands {
    Run {
        #[arg(long)]
        config: String,
    },
    Stop,
    Status,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum PluginCommands {
    Install { path: String },
    Uninstall { name: String },
    List,
    Reload,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum DashboardCommands {
    Start,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum LoggerCommands {
    Tail {
        #[arg(long)]
        follow: bool,
    },
    Search { query: String },
    Export {
        #[arg(long)]
        format: Option<String>,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum BenchmarkCommands {
    Run {
        model: String,
        #[arg(long)]
        iterations: Option<u32>,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum SecurityCommands {
    AddUser { name: String },
    Grant { user: String, permission: String },
    Sandbox { command: String },
}

fn main() {
    let cli = Cli::parse();
    println!("Parsed command: {:?}", cli);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_spawn() {
        let cli = Cli::try_parse_from(&["owncli", "process", "spawn", "chatbot", "python app.py", "--cpu", "1.5", "--ram", "4096"]).unwrap();
        assert_eq!(cli.command, Commands::Process(ProcessCommands::Spawn { name: "chatbot".into(), cmd: "python app.py".into(), cpu: Some(1.5), ram: Some(4096) }));
    }
    #[test]
    fn test_process_kill() {
        let cli = Cli::try_parse_from(&["owncli", "process", "kill", "999"]).unwrap();
        assert_eq!(cli.command, Commands::Process(ProcessCommands::Kill { pid: 999 }));
    }
    #[test]
    fn test_process_restart() {
        let cli = Cli::try_parse_from(&["owncli", "process", "restart", "chatbot"]).unwrap();
        assert_eq!(cli.command, Commands::Process(ProcessCommands::Restart { name: "chatbot".into() }));
    }
    #[test]
    fn test_process_list() {
        let cli = Cli::try_parse_from(&["owncli", "process", "list"]).unwrap();
        assert_eq!(cli.command, Commands::Process(ProcessCommands::List));
    }
    #[test]
    fn test_process_monitor() {
        let cli = Cli::try_parse_from(&["owncli", "process", "monitor", "999"]).unwrap();
        assert_eq!(cli.command, Commands::Process(ProcessCommands::Monitor { pid: 999 }));
    }

    #[test]
    fn test_scheduler_add_cron() {
        let cli = Cli::try_parse_from(&["owncli", "scheduler", "add", "my_job", "--cron", "0 * * * *"]).unwrap();
        assert_eq!(cli.command, Commands::Scheduler(SchedulerCommands::Add { job: "my_job".into(), cron: Some("0 * * * *".into()), event: None }));
    }
    #[test]
    fn test_scheduler_add_event() {
        let cli = Cli::try_parse_from(&["owncli", "scheduler", "add", "my_job", "--event", "on_model_pull"]).unwrap();
        assert_eq!(cli.command, Commands::Scheduler(SchedulerCommands::Add { job: "my_job".into(), cron: None, event: Some("on_model_pull".into()) }));
    }
    #[test]
    fn test_scheduler_remove() {
        let cli = Cli::try_parse_from(&["owncli", "scheduler", "remove", "job_123"]).unwrap();
        assert_eq!(cli.command, Commands::Scheduler(SchedulerCommands::Remove { job_id: "job_123".into() }));
    }
    #[test]
    fn test_scheduler_list() {
        let cli = Cli::try_parse_from(&["owncli", "scheduler", "list"]).unwrap();
        assert_eq!(cli.command, Commands::Scheduler(SchedulerCommands::List));
    }
    #[test]
    fn test_scheduler_run_now() {
        let cli = Cli::try_parse_from(&["owncli", "scheduler", "run-now", "job_123"]).unwrap();
        assert_eq!(cli.command, Commands::Scheduler(SchedulerCommands::RunNow { job_id: "job_123".into() }));
    }

    #[test]
    fn test_memory_status() {
        let cli = Cli::try_parse_from(&["owncli", "memory", "status"]).unwrap();
        assert_eq!(cli.command, Commands::Memory(MemoryCommands::Status));
    }
    #[test]
    fn test_memory_swap_in() {
        let cli = Cli::try_parse_from(&["owncli", "memory", "swap-in", "gpt2"]).unwrap();
        assert_eq!(cli.command, Commands::Memory(MemoryCommands::SwapIn { model: "gpt2".into() }));
    }
    #[test]
    fn test_memory_swap_out() {
        let cli = Cli::try_parse_from(&["owncli", "memory", "swap-out", "gpt2"]).unwrap();
        assert_eq!(cli.command, Commands::Memory(MemoryCommands::SwapOut { model: "gpt2".into() }));
    }
    #[test]
    fn test_memory_evict() {
        let cli = Cli::try_parse_from(&["owncli", "memory", "evict", "lru"]).unwrap();
        assert_eq!(cli.command, Commands::Memory(MemoryCommands::Evict { policy: Some("lru".into()) }));
    }

    #[test]
    fn test_ipc_pub() {
        let cli = Cli::try_parse_from(&["owncli", "ipc", "pub", "events", "system_ready"]).unwrap();
        assert_eq!(cli.command, Commands::Ipc(IpcCommands::Pub { topic: "events".into(), message: "system_ready".into() }));
    }
    #[test]
    fn test_ipc_sub() {
        let cli = Cli::try_parse_from(&["owncli", "ipc", "sub", "events"]).unwrap();
        assert_eq!(cli.command, Commands::Ipc(IpcCommands::Sub { topic: "events".into() }));
    }
    #[test]
    fn test_ipc_rpc() {
        let cli = Cli::try_parse_from(&["owncli", "ipc", "rpc", "logger", "log_error", "bad error"]).unwrap();
        assert_eq!(cli.command, Commands::Ipc(IpcCommands::Rpc { service: "logger".into(), method: "log_error".into(), payload: "bad error".into() }));
    }
    #[test]
    fn test_ipc_channels() {
        let cli = Cli::try_parse_from(&["owncli", "ipc", "channels"]).unwrap();
        assert_eq!(cli.command, Commands::Ipc(IpcCommands::Channels));
    }

    #[test]
    fn test_model_pull() {
        let cli = Cli::try_parse_from(&["owncli", "model", "pull", "bert"]).unwrap();
        assert_eq!(cli.command, Commands::Model(ModelCommands::Pull { model_id: "bert".into() }));
    }
    #[test]
    fn test_model_push() {
        let cli = Cli::try_parse_from(&["owncli", "model", "push", "./local-bert"]).unwrap();
        assert_eq!(cli.command, Commands::Model(ModelCommands::Push { local_path: "./local-bert".into() }));
    }
    #[test]
    fn test_model_load() {
        let cli = Cli::try_parse_from(&["owncli", "model", "load", "bert"]).unwrap();
        assert_eq!(cli.command, Commands::Model(ModelCommands::Load { model: "bert".into() }));
    }
    #[test]
    fn test_model_unload() {
        let cli = Cli::try_parse_from(&["owncli", "model", "unload", "bert"]).unwrap();
        assert_eq!(cli.command, Commands::Model(ModelCommands::Unload { model: "bert".into() }));
    }
    #[test]
    fn test_model_list() {
        let cli = Cli::try_parse_from(&["owncli", "model", "list"]).unwrap();
        assert_eq!(cli.command, Commands::Model(ModelCommands::List));
    }
    #[test]
    fn test_model_info() {
        let cli = Cli::try_parse_from(&["owncli", "model", "info", "bert"]).unwrap();
        assert_eq!(cli.command, Commands::Model(ModelCommands::Info { model: "bert".into() }));
    }

    #[test]
    fn test_inference_run() {
        let cli = Cli::try_parse_from(&["owncli", "inference", "run", "bert", "--prompt", "hi"]).unwrap();
        assert_eq!(cli.command, Commands::Inference(InferenceCommands::Run { model: "bert".into(), prompt: "hi".into() }));
    }
    #[test]
    fn test_inference_stream() {
        let cli = Cli::try_parse_from(&["owncli", "inference", "stream", "bert", "--prompt", "hi"]).unwrap();
        assert_eq!(cli.command, Commands::Inference(InferenceCommands::Stream { model: "bert".into(), prompt: "hi".into() }));
    }
    #[test]
    fn test_inference_batch() {
        let cli = Cli::try_parse_from(&["owncli", "inference", "batch", "bert", "--file", "input.txt"]).unwrap();
        assert_eq!(cli.command, Commands::Inference(InferenceCommands::Batch { model: "bert".into(), file: "input.txt".into() }));
    }
    #[test]
    fn test_inference_benchmark() {
        let cli = Cli::try_parse_from(&["owncli", "inference", "benchmark", "bert"]).unwrap();
        assert_eq!(cli.command, Commands::Inference(InferenceCommands::Benchmark { model: "bert".into() }));
    }

    #[test]
    fn test_finetune_start() {
        let cli = Cli::try_parse_from(&["owncli", "finetune", "start", "--model", "bert", "--dataset", "data.jsonl"]).unwrap();
        assert_eq!(cli.command, Commands::Finetune(FinetuneCommands::Start { model: "bert".into(), dataset: "data.jsonl".into() }));
    }
    #[test]
    fn test_finetune_stop() {
        let cli = Cli::try_parse_from(&["owncli", "finetune", "stop"]).unwrap();
        assert_eq!(cli.command, Commands::Finetune(FinetuneCommands::Stop));
    }
    #[test]
    fn test_finetune_status() {
        let cli = Cli::try_parse_from(&["owncli", "finetune", "status"]).unwrap();
        assert_eq!(cli.command, Commands::Finetune(FinetuneCommands::Status));
    }

    #[test]
    fn test_embed_generate_text() {
        let cli = Cli::try_parse_from(&["owncli", "embed", "generate", "--text", "hello"]).unwrap();
        assert_eq!(cli.command, Commands::Embed(EmbedCommands::Generate { text: Some("hello".into()), file: None }));
    }
    #[test]
    fn test_embed_generate_file() {
        let cli = Cli::try_parse_from(&["owncli", "embed", "generate", "--file", "doc.txt"]).unwrap();
        assert_eq!(cli.command, Commands::Embed(EmbedCommands::Generate { text: None, file: Some("doc.txt".into()) }));
    }
    #[test]
    fn test_embed_store() {
        let cli = Cli::try_parse_from(&["owncli", "embed", "store", "--collection", "kb"]).unwrap();
        assert_eq!(cli.command, Commands::Embed(EmbedCommands::Store { collection: "kb".into() }));
    }

    #[test]
    fn test_watcher_watch() {
        let cli = Cli::try_parse_from(&["owncli", "watcher", "watch", ".", "--action", "echo 1"]).unwrap();
        assert_eq!(cli.command, Commands::Watcher(WatcherCommands::Watch { dir: ".".into(), action: "echo 1".into() }));
    }
    #[test]
    fn test_watcher_list() {
        let cli = Cli::try_parse_from(&["owncli", "watcher", "list"]).unwrap();
        assert_eq!(cli.command, Commands::Watcher(WatcherCommands::List));
    }
    #[test]
    fn test_watcher_stop() {
        let cli = Cli::try_parse_from(&["owncli", "watcher", "stop", "w_123"]).unwrap();
        assert_eq!(cli.command, Commands::Watcher(WatcherCommands::Stop { watch_id: "w_123".into() }));
    }

    #[test]
    fn test_dataset_download() {
        let cli = Cli::try_parse_from(&["owncli", "dataset", "download", "http://dl"]).unwrap();
        assert_eq!(cli.command, Commands::Dataset(DatasetCommands::Download { url: "http://dl".into() }));
    }
    #[test]
    fn test_dataset_validate() {
        let cli = Cli::try_parse_from(&["owncli", "dataset", "validate", "d.csv"]).unwrap();
        assert_eq!(cli.command, Commands::Dataset(DatasetCommands::Validate { path: "d.csv".into() }));
    }
    #[test]
    fn test_dataset_split() {
        let cli = Cli::try_parse_from(&["owncli", "dataset", "split", "d.csv", "--ratios", "80:20"]).unwrap();
        assert_eq!(cli.command, Commands::Dataset(DatasetCommands::Split { path: "d.csv".into(), ratios: "80:20".into() }));
    }
    #[test]
    fn test_dataset_stats() {
        let cli = Cli::try_parse_from(&["owncli", "dataset", "stats", "d.csv"]).unwrap();
        assert_eq!(cli.command, Commands::Dataset(DatasetCommands::Stats { path: "d.csv".into() }));
    }
    #[test]
    fn test_dataset_convert() {
        let cli = Cli::try_parse_from(&["owncli", "dataset", "convert", "d.csv", "--to", "jsonl"]).unwrap();
        assert_eq!(cli.command, Commands::Dataset(DatasetCommands::Convert { path: "d.csv".into(), to: "jsonl".into() }));
    }

    #[test]
    fn test_vector_insert() {
        let cli = Cli::try_parse_from(&["owncli", "vector", "insert", "db", "--data", "{v:1}"]).unwrap();
        assert_eq!(cli.command, Commands::Vector(VectorCommands::Insert { collection: "db".into(), data: "{v:1}".into() }));
    }
    #[test]
    fn test_vector_query() {
        let cli = Cli::try_parse_from(&["owncli", "vector", "query", "db", "--text", "find"]).unwrap();
        assert_eq!(cli.command, Commands::Vector(VectorCommands::Query { collection: "db".into(), text: "find".into() }));
    }
    #[test]
    fn test_vector_search() {
        let cli = Cli::try_parse_from(&["owncli", "vector", "search", "db", "--query", "q", "--top-k", "10"]).unwrap();
        assert_eq!(cli.command, Commands::Vector(VectorCommands::Search { collection: "db".into(), query: "q".into(), top_k: Some(10) }));
    }
    #[test]
    fn test_vector_list() {
        let cli = Cli::try_parse_from(&["owncli", "vector", "list-collections"]).unwrap();
        assert_eq!(cli.command, Commands::Vector(VectorCommands::ListCollections));
    }

    #[test]
    fn test_secret_set() {
        let cli = Cli::try_parse_from(&["owncli", "secret", "set", "token", "abc"]).unwrap();
        assert_eq!(cli.command, Commands::Secret(SecretCommands::Set { key: "token".into(), value: "abc".into() }));
    }
    #[test]
    fn test_secret_get() {
        let cli = Cli::try_parse_from(&["owncli", "secret", "get", "token"]).unwrap();
        assert_eq!(cli.command, Commands::Secret(SecretCommands::Get { key: "token".into() }));
    }
    #[test]
    fn test_secret_list() {
        let cli = Cli::try_parse_from(&["owncli", "secret", "list"]).unwrap();
        assert_eq!(cli.command, Commands::Secret(SecretCommands::List));
    }
    #[test]
    fn test_secret_inject() {
        let cli = Cli::try_parse_from(&["owncli", "secret", "inject", "python run.py"]).unwrap();
        assert_eq!(cli.command, Commands::Secret(SecretCommands::Inject { program: "python run.py".into() }));
    }

    #[test]
    fn test_gateway_start() {
        let cli = Cli::try_parse_from(&["owncli", "gateway", "start", "--port", "3000"]).unwrap();
        assert_eq!(cli.command, Commands::Gateway(GatewayCommands::Start { port: 3000 }));
    }
    #[test]
    fn test_gateway_stop() {
        let cli = Cli::try_parse_from(&["owncli", "gateway", "stop"]).unwrap();
        assert_eq!(cli.command, Commands::Gateway(GatewayCommands::Stop));
    }
    #[test]
    fn test_gateway_models() {
        let cli = Cli::try_parse_from(&["owncli", "gateway", "models"]).unwrap();
        assert_eq!(cli.command, Commands::Gateway(GatewayCommands::Models));
    }

    #[test]
    fn test_agent_run() {
        let cli = Cli::try_parse_from(&["owncli", "agent", "run", "--config", "ag.yml"]).unwrap();
        assert_eq!(cli.command, Commands::Agent(AgentCommands::Run { config: "ag.yml".into() }));
    }
    #[test]
    fn test_agent_stop() {
        let cli = Cli::try_parse_from(&["owncli", "agent", "stop"]).unwrap();
        assert_eq!(cli.command, Commands::Agent(AgentCommands::Stop));
    }
    #[test]
    fn test_agent_status() {
        let cli = Cli::try_parse_from(&["owncli", "agent", "status"]).unwrap();
        assert_eq!(cli.command, Commands::Agent(AgentCommands::Status));
    }

    #[test]
    fn test_plugin_install() {
        let cli = Cli::try_parse_from(&["owncli", "plugin", "install", "p.wasm"]).unwrap();
        assert_eq!(cli.command, Commands::Plugin(PluginCommands::Install { path: "p.wasm".into() }));
    }
    #[test]
    fn test_plugin_uninstall() {
        let cli = Cli::try_parse_from(&["owncli", "plugin", "uninstall", "p"]).unwrap();
        assert_eq!(cli.command, Commands::Plugin(PluginCommands::Uninstall { name: "p".into() }));
    }
    #[test]
    fn test_plugin_list() {
        let cli = Cli::try_parse_from(&["owncli", "plugin", "list"]).unwrap();
        assert_eq!(cli.command, Commands::Plugin(PluginCommands::List));
    }
    #[test]
    fn test_plugin_reload() {
        let cli = Cli::try_parse_from(&["owncli", "plugin", "reload"]).unwrap();
        assert_eq!(cli.command, Commands::Plugin(PluginCommands::Reload));
    }

    #[test]
    fn test_dashboard_start() {
        let cli = Cli::try_parse_from(&["owncli", "dashboard", "start"]).unwrap();
        assert_eq!(cli.command, Commands::Dashboard(DashboardCommands::Start));
    }

    #[test]
    fn test_logger_tail() {
        let cli = Cli::try_parse_from(&["owncli", "logger", "tail", "--follow"]).unwrap();
        assert_eq!(cli.command, Commands::Logger(LoggerCommands::Tail { follow: true }));
    }
    #[test]
    fn test_logger_search() {
        let cli = Cli::try_parse_from(&["owncli", "logger", "search", "error"]).unwrap();
        assert_eq!(cli.command, Commands::Logger(LoggerCommands::Search { query: "error".into() }));
    }
    #[test]
    fn test_logger_export() {
        let cli = Cli::try_parse_from(&["owncli", "logger", "export", "--format", "json"]).unwrap();
        assert_eq!(cli.command, Commands::Logger(LoggerCommands::Export { format: Some("json".into()) }));
    }

    #[test]
    fn test_benchmark_run() {
        let cli = Cli::try_parse_from(&["owncli", "benchmark", "run", "llama", "--iterations", "5"]).unwrap();
        assert_eq!(cli.command, Commands::Benchmark(BenchmarkCommands::Run { model: "llama".into(), iterations: Some(5) }));
    }

    #[test]
    fn test_security_add_user() {
        let cli = Cli::try_parse_from(&["owncli", "security", "add-user", "bob"]).unwrap();
        assert_eq!(cli.command, Commands::Security(SecurityCommands::AddUser { name: "bob".into() }));
    }
    #[test]
    fn test_security_grant() {
        let cli = Cli::try_parse_from(&["owncli", "security", "grant", "bob", "admin"]).unwrap();
        assert_eq!(cli.command, Commands::Security(SecurityCommands::Grant { user: "bob".into(), permission: "admin".into() }));
    }
    #[test]
    fn test_security_sandbox() {
        let cli = Cli::try_parse_from(&["owncli", "security", "sandbox", "run cmd"]).unwrap();
        assert_eq!(cli.command, Commands::Security(SecurityCommands::Sandbox { command: "run cmd".into() }));
    }
}

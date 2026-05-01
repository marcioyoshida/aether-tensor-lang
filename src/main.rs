use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aether", about = "The Aether tensor language compiler")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build an Aether source file to MLIR / native
    Build {
        input: std::path::PathBuf,
        #[arg(short, long, default_value = "out.mlir")]
        output: std::path::PathBuf,
    },
    /// Type-check and shape-infer without emitting code
    Check { input: std::path::PathBuf },
    /// Generate documentation from Aether source
    Doc {
        input: std::path::PathBuf,
        #[arg(short, long, default_value = "docs/")]
        out_dir: std::path::PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Build { input, output } => aether_tensor_lang::build(&input, &output),
        Command::Check { input } => aether_tensor_lang::check(&input),
        Command::Doc { input, out_dir } => aether_tensor_lang::doc(&input, &out_dir),
    }
}

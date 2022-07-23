use clap::{self, Parser, Subcommand};

mod build;
mod release_jsc;

use build::build;
use release_jsc::{download, release};

#[derive(Parser)]
#[clap(author, version, about = "Tyr cargo tasks", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Build,
  Release {
    #[clap(short, long)]
    target: String,
  },
  Download {
    #[clap(short, long)]
    target: String,
  },
}

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Build => {
      build();
    }
    &Commands::Release { ref target } => {
      release(target).await;
    }
    &Commands::Download { ref target } => {
      download(target).await;
    }
  }
}

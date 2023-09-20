use clap::{Parser, Subcommand};
use cnctd_bump::bump_project;
use tokio;

#[derive(Parser)]
#[command(author, version, about = "Welcome to cnctd-bump", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(short, long)]
    update: Option<String>,
}


#[derive(Subcommand)]
pub enum Commands {
    /// Major
    Major {
        
    },

    /// Minor
    Minor {

    },

    /// Patch
    Patch {

    },
    
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Major {  }) => {
            bump_project("major").await.unwrap()
        }
        Some(Commands::Minor {  }) => {
            bump_project("minor").await.unwrap()
        }
        Some(Commands::Patch {  }) => {
            bump_project("patch").await.unwrap()
        }
        None => {
            bump_project("patch").await.unwrap()
        }
    }
}


use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Kero project.
    New {
        /// The project name.
        #[arg(required = true)]
        name: String,

        /// If the project will be using Lua.
        #[arg(long)]
        lua: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { name, lua }) => {
            if lua {
                println!("creating new Lua project {name:?}...");
            } else {
                println!("creating new project {name:?}...");
            }
        }
        None => {}
    }
}

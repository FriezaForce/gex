mod error;
mod profile;
mod git;
mod ssh;
mod storage;
mod switcher;
mod tui;
mod cli;
mod utils;

use clap::{Parser, Subcommand};
use cli::handlers;

#[derive(Parser)]
#[command(name = "gex")]
#[command(about = "Git profile switcher for managing multiple GitHub accounts")]
#[command(version)]
#[command(author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new profile
    Add {
        /// Profile name
        name: String,
        /// GitHub username
        #[arg(short, long)]
        username: String,
        /// Email address
        #[arg(short, long)]
        email: String,
        /// SSH key name (e.g., id_rsa_personal)
        #[arg(short, long)]
        ssh_key: String,
    },
    /// List all profiles
    List,
    /// Switch to a profile
    Switch {
        /// Profile name to switch to
        name: String,
        /// Apply globally (default is local to current repository)
        #[arg(short, long)]
        global: bool,
    },
    /// Delete a profile
    Delete {
        /// Profile name to delete
        name: String,
    },
    /// Edit a profile
    Edit {
        /// Profile name to edit
        name: String,
    },
    /// Show current profile status
    Status,
    /// Launch interactive TUI
    Tui,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add {
            name,
            username,
            email,
            ssh_key,
        } => handlers::handle_add(name, username, email, ssh_key),
        Commands::List => handlers::handle_list(),
        Commands::Switch { name, global } => handlers::handle_switch(name, global),
        Commands::Delete { name } => handlers::handle_delete(name),
        Commands::Edit { name } => handlers::handle_edit(name),
        Commands::Status => handlers::handle_status(),
        Commands::Tui => {
            use tui::app::TuiApp;
            let mut app = TuiApp::new()?;
            app.run()?;
            Ok(())
        }
    };

    // Handle errors with user-friendly messages
    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        
        // Show suggestion if available
        if e.should_show_suggestion() {
            eprintln!("\n{}", e.with_suggestion());
        }
        
        std::process::exit(1);
    }

    Ok(())
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum SubCommands {
    /// Run jilebi as a process
    Stdio,
    /// Commands for managing plugins
    Plugins {
        #[command(subcommand)]
        subcommand: PluginSubCommands,
    },
    /// Read jilebi server logs
    Log,
}

#[derive(Debug, Clone, Subcommand)]
pub enum PluginSubCommands {
    /// Create a new TS or JS plugin with some useful defaults
    Create,
    /// Add/Install a plugin to use with Jilebi
    Add { id: String },
    /// Remove a plugin from jilebi and delete its state
    Remove { id: String },
    /// Show logs for a specific plugin
    Log { id: String },
}

/// Jilebi MCP server and command line interface for plugin management
#[derive(Debug, Clone, Parser)]
pub struct JilebiCli {
    #[command(subcommand)]
    pub subcommand: SubCommands,
}

pub fn read_log_file(path: &PathBuf) {
    let log_file_content = File::open(path)
        .unwrap_or_else(|err| panic!("Failed to open log file at {}: {}", path.display(), err));
    let reader = BufReader::new(log_file_content);
    for log_line in reader.lines() {
        match log_line {
            Ok(line) => println!("{}", line),
            Err(err) => panic!("Error reading log line: {}", err),
        }
    }
}

pub fn plugin_command_handler(subcommand: PluginSubCommands) -> Result<(), String> {
    match subcommand {
        PluginSubCommands::Create => todo!(),
        PluginSubCommands::Add { id } => todo!(),
        PluginSubCommands::Remove { id } => todo!(),
        PluginSubCommands::Log { id } => todo!(),
    }
}

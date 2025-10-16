mod config;
mod executor;
mod webhook;

use anyhow::{Context, Result};
use clap::Parser;
use config::{GlobalConfig, Hook, ProjectConfig};
use log::{error, info};

#[derive(Parser)]
#[command(name = "ciallo")]
#[command(about = "Execute commands and send notifications via webhooks")]
struct Cli {
    /// Command name to execute from ciallo.toml
    command: String,

    /// Additional arguments to pass to the command
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,

    /// Path to config file
    #[arg(short, long, default_value = "ciallo.toml")]
    config: String,
}

fn main() -> Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut cli = Cli::parse();

    // If args exist and user used --, prepend -- to preserve it for the underlying command
    if !cli.args.is_empty() && std::env::args().any(|arg| arg == "--") {
        cli.args.insert(0, "--".to_string());
    };

    // Load global config
    let global_config = GlobalConfig::load().context("Failed to load global config")?;

    // Load project config
    let project_config = ProjectConfig::from_file(&cli.config)
        .context(format!("Failed to load config from {}", cli.config))?;

    // Find command
    let cmd = project_config
        .cmd
        .get(&cli.command)
        .context(format!("Command '{}' not found in config", cli.command))?;

    let hooks = cmd
        .hooks
        .iter()
        .map(|hook| {
            global_config.hook.get(hook).ok_or(anyhow::anyhow!(
                "Hook '{}' not found in global config",
                hook
            ))
        })
        .collect::<Result<Vec<&Hook>>>()?;

    // Execute command
    info!("Executing {} with hooks: {:?}", cmd.command, cmd.hooks);
    let result = executor::execute_command(cmd, &cli.args).context("Failed to execute command")?;
    info!(
        "Command finished with status: {}",
        if result.success { "SUCCESS" } else { "FAILED" }
    );

    // Send notifications to all configured hooks
    for hook in hooks {
        match hook {
            Hook::Feishu(feishu_hook) => {
                info!("Sending notification to hook: {:?}", hook);
                match webhook::send_feishu_notification(feishu_hook, &result) {
                    Ok(_) => info!("✓ Notification sent to {:?}", hook),
                    Err(e) => error!("✗ Failed to send notification to {:?}: {}", hook, e),
                }
            }
        }
    }

    // Exit with same code as the command
    if !result.success {
        std::process::exit(result.exit_code.unwrap_or(1));
    }

    Ok(())
}

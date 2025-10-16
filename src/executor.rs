use crate::config::Command;
use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command as StdCommand, Stdio};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub duration: Duration,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub fn execute_command(cmd: &Command, extra_args: &[String]) -> Result<ExecutionResult> {
    // Parse command
    let parts: Vec<&str> = cmd.command.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Empty command");
    }

    let program = parts[0];
    let args = &parts[1..];

    // Configure stdio based on config
    let stdout_cfg = if cmd.stdout {
        Stdio::piped()
    } else {
        Stdio::null()
    };

    let stderr_cfg = if cmd.stderr {
        Stdio::piped()
    } else {
        Stdio::null()
    };

    let start_time = Instant::now();
    let mut child = StdCommand::new(program)
        .args(args)
        .args(extra_args)
        .stdout(stdout_cfg)
        .stderr(stderr_cfg)
        .spawn()
        .context("Failed to spawn command")?;

    // Capture stdout
    let mut stdout_lines = Vec::new();
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line?;
            println!("{}", line);
            stdout_lines.push(line);
        }
    }

    // Capture stderr
    let mut stderr_lines = Vec::new();
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            let line = line?;
            eprintln!("{}", line);
            stderr_lines.push(line);
        }
    }

    let status = child.wait()?;

    let duration = start_time.elapsed();

    Ok(ExecutionResult {
        success: status.success(),
        duration,
        exit_code: status.code(),
        stdout: stdout_lines.join("\n"),
        stderr: stderr_lines.join("\n"),
    })
}

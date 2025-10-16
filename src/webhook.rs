use crate::{config::FeishuHook, executor::ExecutionResult};
use anyhow::Result;
use serde::Serialize;

/// Feishu message struct
///
/// This will be serialized to JSON and sent as the body of the request to the Feishu webhook.
#[derive(Serialize)]
pub struct FeishuMessage {
    msg: String,
}

impl From<ExecutionResult> for FeishuMessage {
    fn from(result: ExecutionResult) -> Self {
        let status = if result.success {
            "✅ success"
        } else {
            "❌ failure"
        };
        Self {
            msg: format!(
                "{status}({:?})\n{}\n{}",
                result.duration, result.stdout, result.stderr
            ),
        }
    }
}

pub fn send_feishu_notification(hook: &FeishuHook, result: &ExecutionResult) -> Result<()> {
    let message = FeishuMessage::from(result.clone());

    let client = reqwest::blocking::Client::new();
    let response = client.post(&hook.webhook_url).json(&message).send()?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to send webhook: {}", response.status());
    }

    Ok(())
}

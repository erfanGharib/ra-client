use std::process::Command;
use std::str;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::websocket::index::TWsSender;

pub async fn on_command_received(
    ws_sender: &mut TWsSender, 
    raw_command: &str
) -> Result<(), ()> {
    println!("{:?}", raw_command);

    if !raw_command.starts_with("command:") {
        return Err(());
    }

    let raw_command = raw_command.replace("command:", "");
    let mut parts = raw_command.split_whitespace();
    let command = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();

    println!("{:?} -> skjdflk", command);
    println!("{:?}", args);

    if command.len() <= 0 {
        return Err(());
        // continue;
    }

    println!("{:?}", command);

    let output = Command::new(command).args(&args).output();

    return match output {
        Ok(output) => {
            // Convert the command's stdout to a string
            let stdout = str::from_utf8(&output.stdout).unwrap_or("Failed to parse stdout");
            let stderr = str::from_utf8(&output.stderr).unwrap_or("Failed to parse stderr");

            ws_sender
                .send(Message::Text(
                    format!("command_result:{stdout} {stderr}").to_string(),
                ))
                .await
                .expect("command error: {stderr}");

            Ok(())
        }
        Err(e) => {
            ws_sender
                .send(Message::Text(format!("command_result:{e}").to_string()))
                .await
                .expect("command error: {e}");

            Err(())
        }
    }
}

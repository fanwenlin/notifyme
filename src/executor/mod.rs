use libc;
use log::{error, info};
use std::error::Error;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command as TokioCommand;
use tokio::signal::unix::{signal, SignalKind};

pub struct CommandExecutor {
    cmd: String,
    args: Vec<String>,
    output: Option<String>,
}

impl CommandExecutor {
    pub fn new(cmd: String, args: Vec<String>) -> Self {
        CommandExecutor {
            cmd,
            args,
            output: None,
        }
    }

    pub async fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Executing command: {} with args: {:?}", self.cmd, self.args);

        let mut child = TokioCommand::new(&self.cmd)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()?;

        let child_id = child.id();
        let mut sigint = signal(SignalKind::interrupt())?;
        let mut stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let mut output = String::new();

        tokio::select! {
            result = async {
                let mut buffer = [0; 1024];
                while let Ok(n) = stdout.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    let chunk = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}", chunk);
                    output.push_str(&chunk);
                }

                let status = child.wait().await?;
                if status.success() {
                    self.output = Some(output);
                    info!("Command executed successfully");
                    Ok(())
                } else {
                    let mut stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
                    let mut error_msg = String::new();
                    stderr.read_to_string(&mut error_msg).await?;
                    error!("Command failed: {}", error_msg);
                    Err(format!("Command failed: {}", error_msg).into())
                }
            } => result,

            _ = sigint.recv() => {
                info!("Received Ctrl-C, forwarding to child process...");
                if let Some(pid) = child_id {
                    unsafe {
                        libc::kill(pid as i32, libc::SIGINT);
                    }
                }
                child.wait().await?;
                Ok(())
            }
        }
    }

    pub fn get_output(&self) -> Option<&String> {
        self.output.as_ref()
    }
}

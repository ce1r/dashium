use crate::Result;
use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum Command {
    #[bpaf(command("ping"))]
    Ping,
}

impl Command {
    pub async fn execute_command(cmd: Command) -> Result<String> {
        match cmd {
            Command::Ping => {
                tracing::info!("Ping!");
                Ok("1".to_string())
            }
        }
    }
}

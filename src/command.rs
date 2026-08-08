use crate::Result;
use bpaf::Bpaf;
use cornucopia::types::Role;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum Command {
    #[bpaf(command("ping"))]
    Ping,
}

impl Command {
    pub fn execute_command(cmd: Self, role: Role) -> Result<String> {
        match cmd {
            Self::Ping => Ok(Self::ping(role)),
        }
    }

    fn ping(_role: Role) -> String {
        tracing::info!("Ping!");

        "1".to_string()
    }
}

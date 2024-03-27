use crate::command_buffer::entity_creation_command::EntityCreationCommand;

pub trait CommandLog {
    fn log_before(&self);
    fn log_after(&self);
    fn log_error(&self, error: &str);
}

impl CommandLog for EntityCreationCommand {
    fn log_before(&self) {
        log::info!("Executing EntityCreationCommand with {} components", self.components.len());
    }

    fn log_after(&self) {
        log::info!("Executed EntityCreationCommand successfully");
    }

    fn log_error(&self, error: &str) {
        log::error!("Error executing EntityCreationCommand: {}", error);
    }
}

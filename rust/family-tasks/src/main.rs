use family_tasks::{HouseholdServicer, TaskServicer, UserServicer};
use reboot::prelude::*;

#[reboot::main]
async fn main() -> reboot::Result<()> {
    Application::builder("family-tasks")
        .oauth(reboot::oauth::google())
        .mcp(|mcp| mcp.expose_service::<UserServicer>())
        .services((UserServicer, HouseholdServicer, TaskServicer))
        .run()
        .await
}

use serenity::client::Context;
use serenity::http::Http;
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

use crate::util::CliResult;

mod basic;

pub async fn register_global_commands(http: &Http) -> CliResult {
    let commands = http.get_global_application_commands().await?;
    tracing::info!("current application commands: {:?}", commands);

    let commands = ApplicationCommand::set_global_application_commands(&http, |cmds| {
        cmds
            .create_application_command(|command| {
                command
                    .name("about")
                    .description("Get bot info")
            })
            .create_application_command(|command| {
                command
                    .name("invite")
                    .description("Displays a link to invite modbot.")
            })
    })
    .await?;
    tracing::info!("invite command: {:?}", commands);
    Ok(())
}

pub async fn handle_command(ctx: Context, command: ApplicationCommandInteraction) {
    let res = match command.data.name.as_str() {
        "about" => basic::about(ctx, command).await,
        "invite" => basic::invite(ctx, command).await,
        _ => unreachable!(),
    };
    if let Err(e) = res {
        tracing::error!("failed to responsed to interaction: {}", e);
    }
}

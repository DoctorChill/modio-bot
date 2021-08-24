use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteraction as Command;
use serenity::model::interactions::InteractionApplicationCommandCallbackDataFlags as DataFlags;
use serenity::model::interactions::InteractionResponseType;

pub async fn about(ctx: Context, command: Command) -> CommandResult {
    let (name, avatar) = ctx
        .cache
        .current_user_field(|u| (u.name.clone(), u.avatar_url()))
        .await;

    let guilds = ctx.cache.guild_count().await;
    command
        .create_interaction_response(ctx, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    msg.create_embed(|e| {
                        let version = if env!("GIT_SHA") == "UNKNOWN" {
                            env!("CARGO_PKG_VERSION").to_string()
                        } else {
                            format!(
                                "{} ([{}](https://github.com/nickelc/modio-bot/commit/{}))",
                                env!("CARGO_PKG_VERSION"),
                                env!("GIT_SHA_SHORT"),
                                env!("GIT_SHA"),
                            )
                        };
                        e.author(|a| {
                            let mut a = a.name(name);
                            if let Some(avatar) = avatar {
                                a = a.icon_url(avatar);
                            }
                            a
                        })
                        .footer(|f| f.text(format!("Servers: {}", guilds)))
                        .field(
                            "Invite to server",
                            "[discordbot.mod.io](https://discordbot.mod.io)",
                            true,
                        )
                        .field(
                            "mod.io Discord",
                            "[discord.mod.io](https://discord.mod.io)",
                            true,
                        )
                        .field(
                            "modbot Discord",
                            "[discord.gg/XNX9665](https://discord.gg/XNX9665)",
                            true,
                        )
                        .field(
                            "Website/Blog",
                            "[ModBot for Discord](https://mod.io/blog/modbot-for-discord)",
                            true,
                        )
                        .field(
                            "Github",
                            "[nickelc/modio-bot](https://github.com/nickelc/modio-bot)",
                            true,
                        )
                        .field("Version", version, true)
                    })
                })
        })
        .await?;

    Ok(())
}

pub async fn invite(ctx: Context, command: Command) -> CommandResult {
    command
        .create_interaction_response(ctx, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    let content = "Visit <https://discordbot.mod.io> to invite modbot to join your Discord server. Once modbot has joined, you can set the default game and subscribe to game(s) for updates using the `game` and `subscribe` commands.";
                    msg.content(content).flags(DataFlags::EPHEMERAL)
                })
        })
        .await?;

    Ok(())
}

use std::env;

use rand::prelude::random;
use serde_json::{Map, Number, Value};
use serenity::{
    async_trait,
    model::{gateway::Ready, interactions::Interaction},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let app_id = env!("DISCORD_APP_ID")
            .parse()
            .expect("Expected $DISCORD_APP_ID to be type of u64");

        let mut map = Map::new();
        map.insert("name".to_string(), Value::String("try".to_string()));
        map.insert(
            "description".to_string(),
            Value::String("Попытаться что-то сделать".to_string()),
        );
        let mut argument = Map::new();
        argument.insert("type".to_string(), Value::Number(Number::from(3)));
        argument.insert("name".to_string(), Value::String("action".to_string()));
        argument.insert(
            "description".to_string(),
            Value::String("Что ты хочешь сделать".to_string()),
        );
        argument.insert("required".to_string(), Value::Bool(true));
        map.insert(
            "options".to_string(),
            Value::Array(vec![Value::Object(argument)]),
        );

        for guild_id in env!("DISCORD_GUILD_IDS").split(',').map(|entry| {
            entry.parse().expect(
                "Expected $DISCORD_GUILD_IDS entry to be type of u64 array separated by `,`",
            )
        }) {
            ctx.http
                .create_guild_application_command(app_id, guild_id, &Value::Object(map.clone()))
                .await
                .expect("Error creating a guild command");
        }
    }

    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Some(data) = interaction.data {
            if data.name == "try" {
                if data.options.len() == 1 {
                    let option = &data.options[0];
                    if option.name == "action" {
                        if let Some(argument) = &option.value {
                            if let Value::String(action) = argument {
                                interaction
                                    .channel_id
                                    .say(
                                        &context.http,
                                        format!(
                                            "{} попытался {} [{}]",
                                            interaction.member.display_name(),
                                            action,
                                            if random() {
                                                "Успешно"
                                            } else {
                                                "Неудачно"
                                            }
                                        ),
                                    )
                                    .await
                                    .expect("Error sending message");
                            }
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Orbot");
    let mut client = Client::builder(
        &env!("DISCORD_TOKEN"),
    )
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#![feature(str_split_as_str)]
use std::env;
use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{
        gateway::{
            Ready,
            Activity,
        },
        user::OnlineStatus,
        id::GuildId,
        channel::Message,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};
use serenity::model::prelude::ChannelId;
use tokio::time::{sleep, Duration};
extern crate reqwest;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                "id" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                        options
                    {
                        format!("{}'s id is {}", user.tag(), user.id)
                    } else {
                        "Please provide a valid user".to_string()
                    }
                },
                "wonderful_command" => {
                    "Hey, I'm the the wonderful command!".to_string()
                },
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready){
        println!("{} is connected!", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("id").description("Get a user id").create_option(|option| {
                        option
                            .name("id")
                            .description("The user to lookup")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
                })
        })
        .await;

        println!("I now have the following global slash commands: {:#?}", commands);

        let guild_command = GuildId(824967769635946516)
            .create_application_command(&ctx.http, |command| {
                command.name("wonderful_command").description("An amazing command")
            })
            .await;

        println!("I created the following guild command: {:#?}", guild_command);  

        let etherscan = env::var("ETHERSCAN")
            .expect("Expected an application id in the environment");
        let link = "https://api.etherscan.io/api?module=gastracker&action=gasoracle&apikey=";
        let api_link = link.to_owned() + &etherscan;
        let mut wait_amount = 0;
    
        loop{
            let request = foo(&api_link).await.unwrap();
            let eth_gas = request.to_string();
            if wait_amount<0
            { 
                if request<50
                {
                    ChannelId(852380942676918302).say(&ctx, "<@463380179260276736> Gas is currently ".to_owned() + &eth_gas).await.unwrap();
                    wait_amount = 120;
                    println!("no message for 10 mins");
                }
                else if request > 49 && request < 90
                {
                    ChannelId(852380942676918302).say(&ctx, "Gas is currently ".to_owned() + &eth_gas).await.unwrap();
                    wait_amount = 60;
                    println!("no message for 5 mins");
                }
            }
            wait_amount = wait_amount - 1;
            let activity = Activity::playing(&eth_gas);
            let status = OnlineStatus::Online;
            ctx.set_presence(Some(activity), status).await;
            sleep(Duration::from_millis(5000)).await;
        }
    }
}

async fn foo(link: &str) -> Result<i32, reqwest::Error> {
    let text = reqwest::get(link)
        .await?
        .text()
        .await?;
    let mut lol = text.split("SafeGasPrice\":\"");
    lol.next();
    let bruh = lol.as_str().split('"');
    let mut temp = 0;
    for v in bruh {
        let parsed: String = v.parse().unwrap();
        println!("{}", parsed);
        let my_int = parsed.parse::<i32>().unwrap();
        temp = my_int;
        break
    }
    Ok(temp)
}

#[tokio::main]
async fn main() {
    // dotenv().expect(".env file not found");
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

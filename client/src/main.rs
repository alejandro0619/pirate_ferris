use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    types::Update
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot");

    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<Commands>().endpoint(answer))
        .branch(Update::filter_message().endpoint(foo));
    
    Dispatcher::builder(bot, handler)
        .build()
        .dispatch().await;
    //Commands::repl(bot, answer).await;
    
}

#[allow(dead_code)]
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Commands {
    #[command(description = "Help")]
    Help,
    #[command(description = "Top 10 users")]
    KarmaTop,
    #[command(description = "Top 10 hated users")]
    KarmaTopHate,
    #[command(description = "Check a user's karma")]
    CheckUser,
}

async fn answer(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    println!("bien"); 
    match cmd {
        Commands::Help => {
            bot.send_message(msg.chat.id, Commands::descriptions().to_string()).await?
        },
        Commands::KarmaTop => {
            bot.send_message(msg.chat.id, "This triggers top 10 users").await?
        },
        Commands::KarmaTopHate => {
            bot.send_message(msg.chat.id, "This triggers top 10 hated users").await?
        }
        Commands::CheckUser=> {
            bot.send_message(msg.chat.id, 
                format!("Here you must send a username"))
                .await?
        }
    };

    Ok(())
}
async fn foo(bot: Bot, msg: Message) -> ResponseResult<()>{
    //bot.send_message(msg.chat.id, "reached this").await?;
    
    if let Some(txt) = msg.text() {
        if txt == "+1" || txt == "-1"{
            bot.send_message(msg.chat.id, "Adding or sub").await?;
        }
    }
    Ok(())
}
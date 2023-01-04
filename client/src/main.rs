use teloxide::{prelude::*, types::Update};
mod library;
use library::{command::Commands, message::Handler};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot");

    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Commands>()
                .endpoint(Commands::answer),
        )
        .branch(Update::filter_message().endpoint(Handler::process_msg));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

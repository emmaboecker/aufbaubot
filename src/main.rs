use std::env;
use std::sync::Arc;

use dotenvy::dotenv;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::Dispatcher;
use teloxide::types::{Update, Message};
use teloxide::{Bot, dptree, respond};
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::words::Deletion;

mod words;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let deletions_path = env::var("DELETIONS_FILE").unwrap_or("./deletions.json".to_string());
    let deletions_file = tokio::fs::read_to_string(deletions_path).await?;

    let deletions: Vec<Deletion> = serde_json::from_str(&deletions_file)?;

    tracing::info!("Loaded Deletions: {:#?}", deletions);

    let deletions = Arc::new(deletions);


    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let bot = Bot::from_env();

    tracing::info!("Bot logged in as @{}", bot.get_me().await.unwrap().username());

    let deletions_handler = Update::filter_message().endpoint(
        |bot: Bot, deletions: Arc<Vec<Deletion>>, message: Message| async move {
            'outer: for deletion in deletions.clone().iter() {
                for keyword in deletion.keywords.clone() {
                    if let Some(text) = message.text() {
                        if text.to_lowercase().contains(&keyword) {
                            let mut reply = bot.send_message(message.chat.id, deletion.response.clone()).reply_to_message_id(message.id);
                            if let Some(id) = message.thread_id {
                                reply = reply.message_thread_id(id);
                            }
                            let reply_result = reply.await;
                            if let Err(err) = reply_result {
                                tracing::error!("Failed to reply to message: {:?}", err)
                            }
                            let delete_result = bot.delete_message(message.chat.id, message.id).await;
                            if let Err(err) = delete_result {
                                tracing::error!("Failed to delete message: {:?}", err)
                            }
                            break 'outer;
                        }
                    }
                }
            }

            respond(())
        },
    );

    Dispatcher::builder(bot, deletions_handler)
        .dependencies(dptree::deps![deletions])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

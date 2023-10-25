use dotenvy::dotenv;
use teloxide::Bot;
use teloxide::payloads::{SendMessageSetters};
use teloxide::requests::Requester;
use teloxide::types::Message;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod words;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let bot = Bot::from_env();

    tracing::info!("Bot logged in as @{}", bot.get_me().await.unwrap().username());

    teloxide::repl(bot, |bot: Bot, message: Message| async move {
        tracing::info!("Got message: {:?}", message.text());

        let nahost = [
            "israel", "palestina", "palestinenser"
        ];

        let dangerous = [
            "jan schiffer", "jan fischer", "köln"
        ];

        for word in nahost.iter() {
            if message.text().unwrap().to_lowercase().contains(word) {
                let mut reply = bot.send_message(message.chat.id, "🚨 NAHOST ERKANNT 🚨").reply_to_message_id(message.id);
                if let Some(id) = message.thread_id {
                    reply = reply.message_thread_id(id);
                }
                reply.await?;
                bot.delete_message(message.chat.id, message.id).await?;
                return Ok(());
            }
        }

        for word in dangerous.iter() {
            if message.text().unwrap().to_lowercase().contains(word) {
                let mut reply = bot.send_message(message.chat.id, "🚨 POTENZIELL GEFÄHRLICHER INHALT ERKANNT 🚨").reply_to_message_id(message.id);
                if let Some(id) = message.thread_id {
                    reply = reply.message_thread_id(id);
                }
                reply.await?; 

                bot.delete_message(message.chat.id, message.id).await?;
                return Ok(());
            }
        }
        Ok(())
    }).await;
}

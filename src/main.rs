use grammers_client::{Client, Config, InitParams, Update};
use grammers_client::types::*;


use grammers_session::Session;
use regex::Regex;
use std::{env, error::Error, sync::Arc, time::Duration, collections::HashMap};
use tokio::{sync::{Semaphore, Mutex}, time::{Instant, sleep}};
use lazy_static::lazy_static;
use rand::Rng;
use dotenv::dotenv;
use grammers_tl_types::enums::messages::ChatFull;
use grammers_tl_types::functions::messages::SendReaction;
use grammers_tl_types::enums::{messages, Reaction, InputPeer, InputChannel};
use grammers_tl_types::functions::channels::GetFullChannel;
use grammers_tl_types::types::{ InputPeerChat, InputPeerUser, ReactionEmoji};
use grammers_tl_types::types::{InputPeerChannel, InputChannel as RawInputChannel};
use grammers_tl_types::enums::{Chat as TlChat, Message, Peer};
use grammers_tl_types::types::{Channel, User};
use grammers_client::types::Chat;



lazy_static! {
    static ref AMOUNT_REGEX: Regex = Regex::new(r"\b(\d{4,7}[,.]?\d{0,2})\s*₽").unwrap();
    static ref CHAT_CACHE: Mutex<HashMap<i64, InputPeer>> = Mutex::new(HashMap::new());
}






#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_id: i32 = env::var("TELEGRAM_API_ID")?.parse()?;
    let api_hash = env::var("TELEGRAM_API_HASH")?;
    let phone = env::var("TELEGRAM_PHONE")?;

    let config = Config {
        api_id,
        api_hash: api_hash.clone(),
        session: Session::load_file_or_create("bot.session")?,
        params: InitParams {
            device_model: "Pixel 7 Pro".into(),
            system_version: "Android 14".into(),
            app_version: "10.2.0".into(),
            catch_up: false,
            ..InitParams::default()
        },
    };

    let main_client = Arc::new(Client::connect(config).await?);

    if !main_client.is_authorized().await? {
        let login_token = main_client.request_login_code(&phone).await?;
        println!("Enter code:");
        let mut code = String::new();
        std::io::stdin().read_line(&mut code)?;
        main_client.sign_in(&login_token, code.trim()).await?;
        main_client.session().save_to_file("bot.session")?;
    }

    let semaphore = Arc::new(Semaphore::new(10));
    println!("✅ Bot started");

    loop {
        if let Ok(Update::NewMessage(message)) = main_client.next_update().await {
           
            let text = message.text();
            if let Some(amount) = parse_amount(text) {
                if amount >= 1_000.0 {
          let delay = rand::thread_rng().gen_range(50..200);
               

            tokio::time::sleep(Duration::from_millis(delay)).await;

          
            
            let reaction = vec![Reaction::Emoji(ReactionEmoji {
                emoticon: "👍".into()
            })];
            
            main_client.send_reactions(
                message.chat(),
                message.id(),
                reaction,
            ).await?;
        }}}
    }
}



fn parse_amount(text: &str) -> Option<f64> {
    AMOUNT_REGEX.captures(text)?
        .get(1)?
        .as_str()
        .replace([' ', ','], "")
        .parse()
        .ok()
}

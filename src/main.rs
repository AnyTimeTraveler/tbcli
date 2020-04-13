#![feature(async_closure)]

use std::{env, io};
use std::io::{BufRead, Write};

use argparse::{ArgumentParser, Store, StoreTrue};
use futures::{join, StreamExt};
use telegram_bot::*;

struct Options {
    id: String,
    token: String,
    send_only: bool,
    receive_only: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut options = Options {
        id: "".to_string(),
        token: "".to_string(),
        send_only: false,
        receive_only: false,
    };

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Use telegram bot from cli.");
        ap.refer(&mut options.id).add_option(&["-i", "--id"], Store, "ID of receiving user/group/channel");
        ap.refer(&mut options.token).add_option(&["-t", "--token"], Store, "Telegram API KEY");
        ap.refer(&mut options.send_only).add_option(&["-s", "--send-only"], StoreTrue, "Send only");
        ap.refer(&mut options.receive_only).add_option(&["-r", "--receive-only"], StoreTrue, "Receive only");
        ap.parse_args_or_exit();
    }

    if options.token.is_empty() {
        options.token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    }
    if options.id.is_empty() {
        options.id = env::var("TELEGRAM_RECEIVER_ID").expect("TELEGRAM_RECEIVER_ID not set");
    }

    let api = Api::new(options.token.as_str());
    if options.send_only {
        Ok(send(api.clone(), &options).await?)
    } else if options.receive_only {
        Ok(receive(api).await?)
    } else {
        let (r_send, r_receive) = join!(
            send(api.clone(), &options),
            receive(api.clone())
        );
        if r_send.is_err() {
            r_send
        } else if r_receive.is_err() {
            r_receive
        } else {
            Ok(())
        }
    }
}

async fn send(api: Api, options: &Options) -> Result<(), Error> {
    let user_id: i64 = match options.id.parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid id!");
            return Ok(());
        }
    };
    let stdin_unlocked = io::stdin();
    let stdin = stdin_unlocked.lock();
    let user_id: UserId = user_id.into();
    for line in stdin.lines() {
        if let Ok(text) = line {
            api.send(user_id.text(text)).await?;
        }
    }
    Ok(())
}

async fn receive(api: Api) -> Result<(), Error> {
    let mut stream = api.stream();
    let stdout_unlocked = io::stdout();
    let mut stdout = stdout_unlocked.lock();

    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                let string = format!("{},{},{},{},{},{},{}\n",
                                     &message.from.username.unwrap_or("".to_string()),
                                     &message.from.first_name,
                                     &message.from.last_name.unwrap_or("".to_string()),
                                     &message.from.id,
                                     &message.chat.id(),
                                     &message.date,
                                     data);
                stdout.write(string.as_bytes()).expect("BBB");
                stdout.flush().expect("AAA")
            }
        }
    }
    Ok(())
}

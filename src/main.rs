//Power bot V0

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait] //Asynchronous event handler
impl EventHandler for Handler {
    /* Sends a message depening on msg's content

    Event handlers are dispatched via a threadpool, multiple
    events can be dispatched simultaneaously
    */
    async fn message(&self, context: Context, msg: Message) {
        //Handler for a simple ping command
        if msg.content == "~ping" {
            if let Err(e) = msg
                .channel_id
                .say(&context.http, "Pongo!!! you idiot")
                .await
            {
                println!("Error sending message: {:?}", e);
            };
        }
    }

    /*
    Handler for a "Ready" event
    Called when a shard is booted and a READY payload is sent by discord

    */
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connexted", ready.user.name);
    }
}

#[tokio::main] //enables asynchronous programming
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    //Create an instance of a Client, logging in as a bot
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    //start a shard and begin listening
    if let Err(e) = client.start().await {
        println!("Client error {:?}", e)
    }
}

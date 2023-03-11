#![allow(unused)]
use carapax::methods::SendPhoto;
use carapax::types::{KeyboardButton, InlineKeyboardButton, InputFile, Message, MessageData, TextEntity};
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text, Update},
    Api, App, Context, ExecuteError, Ref,
};
use database::Museums;
use dotenv::dotenv;
use geo::point;
use geo::prelude::*;
use std::env;
use std::fmt::format;

mod buttons;
mod database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let app = App::new(context, echo);
    LongPoll::new(api, app).run().await
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), ExecuteError> {
    if let MessageData::Location(location) = message.data {
        let mus_struct = database::database().await;
        for museum in distance(location.latitude.into(), location.longitude.into(), mus_struct) 
        {
            let mus_description = &museum.summ;
			let mut vector: Vec<&str> = mus_description.lines().collect();

            api.execute(
                SendMessage::new(chat_id.clone(), &museum.name).reply_markup(vec![vec![
                    InlineKeyboardButton::with_url("Web site",  &museum.site),
                ]]),
            )
            .await?;
            api.execute(
                SendMessage::new(chat_id.clone(), &museum.addr).reply_markup(vec![vec![
                    InlineKeyboardButton::with_url("📍Open google map",  &museum.ggle),
                ]]),
            )
            .await?;
        }
        api.execute(SendMessage::new(
            chat_id.clone(),
            "If you need us again, send the geo-location to the chat room ☺️",
        ))
        .await?;
    } else {
        api.execute(SendMessage::new(
            chat_id.clone(),
            "Hi! To find the nearest museum, please send your geo-location to the chat.",
        ))
        .await?;
    };
    Ok(())
}

fn distance(
    lat_user: f64,
    lon_user: f64,
    mut db_vec: Vec<Museums>,
) -> Vec<Museums> {
    let point_user = point!(x: lat_user, y: lon_user);
    db_vec.sort_by(|a, b| {
        let distance_a = point_user.geodesic_distance(&point!(x: a.lngt, y: a.lttd));
        let distance_b = point_user.geodesic_distance(&point!(x: b.lngt, y: b.lttd));
        distance_a
            .abs()
            .partial_cmp(&distance_b.abs())
            .expect("Failed to compare points.")
    });
    db_vec.into_iter().take(3).collect()
}


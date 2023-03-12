#![allow(unused)]
use carapax::methods::SendPhoto;
use carapax::types::{KeyboardButton, InlineKeyboardButton, InputFile, Message, MessageData, TextEntity, InlineQuery, ReplyKeyboardMarkup};
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
use std::path::Path;
use std::{env, fs};
use std::fmt::format;

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

            let mut photo_addr = format!("images/{}.jpg", museum.name);
            if !Path::new(&photo_addr).exists() {
                photo_addr = "images/EMPTY.jpg".to_string();
            }

            api.execute(
                SendPhoto::new(chat_id.clone(), InputFile::path(&photo_addr).await.unwrap())
                    .caption(&museum.name)
                    .caption_entities(&[TextEntity::bold(0..8)])
                    .expect("Failed to make caption bold."),
            )
            .await?;

            api.execute(
                SendMessage::new(chat_id.clone(), &museum.summ).reply_markup(vec![vec![
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
        let send_location = KeyboardButton::new("Send location");
        api.execute(
            SendMessage::new(chat_id.clone(), "Hi! To find the nearest museum, please send your geo-location to the chat.").reply_markup(vec![vec![
                KeyboardButton::request_location(send_location),
            ]]),
        )
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
        let distance_a = point_user.geodesic_distance(&point!(x: a.lttd, y: a.lngt));
        let distance_b = point_user.geodesic_distance(&point!(x: b.lttd, y: b.lngt));
        distance_a
            .abs()
            .partial_cmp(&distance_b.abs())
            .expect("Failed to compare points.")
    });
    db_vec.into_iter().take(2).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distance_gives_right_order() {
        let point0 = (41.6963678, 44.8199377);
        let point1 = (41.7255743, 44.746247);
        let point2 = (41.7106533, 44.7447204);
        let museums = database::database().await;
        let distance_to_point_0 = distance(point0.0, point0.1, museums.clone());
        let distance_to_point_1 = distance(point1.0, point1.1, museums.clone());
        let distance_to_point_2 = distance(point2.0, point2.1, museums);
        assert_eq!(distance_to_point_0[0].lngt, distance_to_point_1[0].lngt);
        assert_eq!(distance_to_point_1[0].lngt, distance_to_point_2[0].lngt);
        assert_eq!(distance_to_point_2[0].lngt, distance_to_point_0[0].lngt);
        dbg!(distance_to_point_0);
        dbg!(distance_to_point_1);
        dbg!(distance_to_point_2);
    }

    #[tokio::test]
    async fn test_tbilisi() {
        let point0 = (41.720802, 44.721416);
        let point1 = (41.727481, 44.793525);
        let museums = database::database().await;
        let distance_to_point_0 = distance(point0.0, point0.1, museums.clone());
        let distance_to_point_1 = distance(point1.0, point1.1, museums);
        assert_eq!(distance_to_point_0[0].lttd, distance_to_point_1[0].lttd);
        dbg!(distance_to_point_0);
        dbg!(distance_to_point_1);
    }
}
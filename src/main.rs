// #![allow(unused)]
use carapax::methods::SendPhoto;
use carapax::types::{KeyboardButton, InlineKeyboardButton, InputFile, Message, MessageData, TextEntity, ReplyKeyboardMarkup, Command};
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::ChatId,
    Api, App, Context, ExecuteError, Ref,
};
use database::Museums;
use dotenv::dotenv;
use geo::point;
use geo::prelude::*;
use std::path::Path;
use std::env;

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
            let mut photo_addr = format!("images/{}.jpg", museum.name);
            if !Path::new(&photo_addr).exists() {
                photo_addr = "images/NO_PHOTO.jpg".to_string();
            }
            let length = museum.name.len() as u32;
            api.execute(
                SendPhoto::new(chat_id.clone(), InputFile::path(&photo_addr).await.unwrap())
                    .caption(&museum.name)
                    .caption_entities(&[TextEntity::bold(0..length)])
                    .expect("Failed to make caption bold."),
            )
            .await?;
        
            api.execute(
                SendMessage::new(chat_id.clone(), &museum.summ).reply_markup(vec![vec![
                    InlineKeyboardButton::with_url("📍Open google map", &museum.ggle),
                ]]),
            )
            .await?;
        }
    // } else if Command::try_from(message).unwrap().get_name() == "/command1" {
        // api.execute(SendMessage::new(
            // chat_id.clone(),
            // "To find the nearest museum, please send your 📍 Location to the chat ☺️",
        // )).await?;
    } else {
        let send_location = KeyboardButton::request_location(KeyboardButton::new("📍 Location"));
        let key_raw = ReplyKeyboardMarkup::row(
            ReplyKeyboardMarkup::default(), vec![send_location]
        );
        let keyboard = ReplyKeyboardMarkup::resize_keyboard(key_raw, true);
        let text = "Hi! To find the nearest museum, please send your 📍 Location to the chat ☺️";
        let sendmessage = SendMessage::new(chat_id, text);
        let button_message = SendMessage::reply_markup(sendmessage, keyboard);
        api.execute(
            button_message
        ).await?;
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
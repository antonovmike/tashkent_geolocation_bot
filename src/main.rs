#![allow(unused)]
use crate::catalog::CoffeeHouse;
use carapax::methods::SendPhoto;
use carapax::types::{KeyboardButton, InlineKeyboardButton, InputFile, Message, MessageData, TextEntity};
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text, Update},
    Api, App, Context, ExecuteError, Ref,
};
use dotenv::dotenv;
use geo::point;
use geo::prelude::*;
use std::env;

mod catalog;
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
        for cafe in distance(
            location.latitude.into(),
            location.longitude.into(),
            catalog::kofe_list(),
        ) {
			let caffee_description = &cafe.description;
			let mut vector: Vec<&str> = caffee_description.lines().collect();
			let name_length: u32 = vector[1].len().try_into().unwrap();
			
            api.execute(
                SendPhoto::new(chat_id.clone(), InputFile::path(&cafe.photo).await.unwrap())
                    .caption(&cafe.description)
                    .caption_entities(&[TextEntity::bold(0..(name_length + 1))])
                    .expect("Failed to make caption bold."),
            )
            .await?;
            api.execute(
                SendMessage::new(chat_id.clone(), &cafe.address).reply_markup(vec![vec![
                    InlineKeyboardButton::with_url("📍Посмотреть на карте",  &cafe.google_maps),
                ]]),
            )
            .await?;
        }
        api.execute(SendMessage::new(
            chat_id.clone(),
            "database::database().await",
        ))
        .await?;
    } else {
        // let message_db = database::database().await;
        let mus_struct = database::database().await;
        let message_db = mus_struct[0].name.clone();
        api.execute(SendMessage::new(
            chat_id.clone(),
            message_db,
        ))
        .await?;
    };
    Ok(())
}

fn distance(
    lat_user: f64,
    lon_user: f64,
    mut list_of_coffe_houses: [CoffeeHouse; 46],
) -> Vec<CoffeeHouse> {
    let point_user = point!(x: lat_user, y: lon_user);
    list_of_coffe_houses.sort_by(|a, b| {
        let distance_a = point_user.geodesic_distance(&point!(x: a.location_x, y: a.location_y));
        let distance_b = point_user.geodesic_distance(&point!(x: b.location_x, y: b.location_y));
        distance_a
            .abs()
            .partial_cmp(&distance_b.abs())
            .expect("Failed to compare points.")
    });
    list_of_coffe_houses.into_iter().take(3).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_gives_right_order() {
        let point0 = (41.6963678, 44.8199377);
        let point1 = (41.7255743, 44.746247);
        let point2 = (41.7106533, 44.7447204);
        let list_of_coffe_houses = catalog::kofe_list();
        let distance_to_point_0 = distance(point0.0, point0.1, list_of_coffe_houses.clone());
        let distance_to_point_1 = distance(point1.0, point1.1, list_of_coffe_houses.clone());
        let distance_to_point_2 = distance(point2.0, point2.1, list_of_coffe_houses.clone());
        assert_ne!(distance_to_point_0, distance_to_point_1);
        assert_ne!(distance_to_point_1, distance_to_point_2);
        assert_ne!(distance_to_point_2, distance_to_point_0);
        dbg!(distance_to_point_0);
        dbg!(distance_to_point_1);
        dbg!(distance_to_point_2);
    }

    #[test]
    fn test_tbilisi() {
        let point0 = (41.720802, 44.721416);
        let point1 = (41.727481, 44.793525);
        let list_of_coffe_houses = catalog::kofe_list();
        let distance_to_point_0 = distance(point0.0, point0.1, list_of_coffe_houses.clone());
        let distance_to_point_1 = distance(point1.0, point1.1, list_of_coffe_houses.clone());
        assert_ne!(distance_to_point_0, distance_to_point_1);
        dbg!(distance_to_point_0);
        dbg!(distance_to_point_1);
    }
}

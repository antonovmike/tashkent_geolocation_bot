use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};
use carapax::{
    methods::SendMessage,
    types::{InlineKeyboardButton, Message, Update, UpdateKind},
    Api, UpdateHandler,
};

use crate::echo;

pub struct Handler {
    pub api: Api,
}

#[derive(Deserialize, Serialize)]
struct CallbackData {
    value: String,
}

impl CallbackData {
    fn new<S: Into<String>>(value: S) -> Self {
        Self { value: value.into() }
    }
}

async fn menu_buttons(api: &Api, update: Update) -> Option<Message> {
// async fn menu_buttons(api: Ref<Api>, update: Update) -> Option<Message> {
    match update.kind {
        UpdateKind::Message(message) => {
            let chat_id = message.get_chat_id();
            if let Some(commands) = message.get_text().and_then(|text| text.get_bot_commands()) {
                let command = &commands[0];
                if command.command == "/start" {
                    let callback_data_1 = CallbackData::new("Геолокация пока не работает");
                    let callback_data_2 = CallbackData::new("Геодезист в запое");
                    let callback_data_3 = CallbackData::new("Ишь чего!");
                    // let callback_data_4 = request_location();
                    let method = SendMessage::new(chat_id, "Добро пожаловать в меню:").reply_markup(vec![vec![
                        InlineKeyboardButton::with_callback_data_struct("Отправить геолокацию", &callback_data_1).unwrap(),
                        InlineKeyboardButton::with_callback_data_struct("Расстояние до кафе", &callback_data_2).unwrap(),
                        InlineKeyboardButton::with_callback_data_struct("Показать ещё", &callback_data_3).unwrap(),
                    ]]);
                    return Some(api.execute(method).await.unwrap());
                }
            }
        }
        UpdateKind::CallbackQuery(query) => {
            if let Some(ref message) = query.message {
                let chat_id = message.get_chat_id();
                let data = query.parse_data::<CallbackData>().unwrap().unwrap();
                let method = SendMessage::new(chat_id, data.value);
                return Some(api.execute(method).await.unwrap());
            }
        }
        _ => {}
    }
    None
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let api = self.api.clone();
        Box::pin(async move {
            log::info!("Got an update: {:?}", update);
            if let Some(msg) = menu_buttons(&api, update).await {
                log::info!("Message sent: {:?}", msg);
            }
        })
    }
}
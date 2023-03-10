// use tokio::{task, runtime::Handle};
// use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}, Row};

// pub async fn database() -> String {
//     let pool = match SqlitePool::connect(&"db.sqlite3").await {
//         Ok(it) => it,
//         Err(err) => return "err 1".to_string(),
//     };
    
//     let (id, name, summary, address, tel, price, schedule, dayoff, website): 
//     (i64, String, String, String, String, String, String, String, String) 
//     = match sqlx::query_as("SELECT * FROM catalog_museum").fetch_one(&pool).await {
//         Ok(it) => it,
//         Err(err) => return err.to_string(),
//     };

//     format!("{}\n{}", name, summary)
// }

use sqlite::State;

pub struct Museums {
    pub name: String,
    summ: String,
    sche: String,
    pric: String,
}

pub async fn database() -> Vec<Museums> {
    let connection = sqlite::open("db.sqlite3").unwrap();
    let query = "SELECT * FROM catalog_museum";
    let mut statement = connection.prepare(query).unwrap();

    let mut museums: Vec<Museums> = vec![];

    while let Ok(State::Row) = statement.next() {
        let temp_sctruct = Museums {
            name: statement.read::<String, _>("name").unwrap(),
            summ: statement.read::<String, _>("summary").unwrap(),
            sche: statement.read::<String, _>("schedule").unwrap(),
            pric: statement.read::<String, _>("price").unwrap(),
        };
        museums.push(temp_sctruct)
    }

    museums
}
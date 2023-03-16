use sqlite::State;

#[derive(Debug, Clone)]
pub struct Base {
    pub name: String,
    pub summ: String,
    pub sche: String,
    pub ggle: String,
    pub lttd: f64,
    pub lngt: f64,
}

pub async fn base_data(typ: &str) -> Vec<Base> {
    let connection = sqlite::open("db.sqlite3").unwrap();
    // "SELECT * FROM catalog_museum" or "SELECT * FROM catalog_cafe"
    let query = &format!("SELECT * FROM {}", typ)[..];
    let mut statement = connection.prepare(query).unwrap();

    let mut museums: Vec<Base> = vec![];

    while let Ok(State::Row) = statement.next() {
        let temp_sctruct = Base {
            name: statement.read::<String, _>("name").unwrap(),
            summ: statement.read::<String, _>("summary").unwrap(),
            sche: statement.read::<String, _>("schedule").unwrap(),
            ggle: statement.read::<String, _>("map_google").unwrap(),
            lttd: statement.read::<f64, _>("latitude").unwrap(),
            lngt: statement.read::<f64, _>("longitude").unwrap(),
        };
        museums.push(temp_sctruct)
    }

    museums
}
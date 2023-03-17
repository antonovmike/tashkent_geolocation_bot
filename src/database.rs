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
    // The variable typ should contain the name of the catalog from the database:
    // "catalog_museum" or "catalog_cafe"
    let query = &format!("SELECT * FROM {}", typ)[..];
    let mut statement = connection.prepare(query).unwrap();

    let mut base_filds: Vec<Base> = vec![];

    while let Ok(State::Row) = statement.next() {
        let temp_sctruct = Base {
            name: statement.read::<String, _>("name").unwrap(),
            summ: statement.read::<String, _>("summary").unwrap(),
            sche: statement.read::<String, _>("schedule").unwrap(),
            ggle: statement.read::<String, _>("map_google").unwrap(),
            lttd: statement.read::<f64, _>("latitude").unwrap(),
            lngt: statement.read::<f64, _>("longitude").unwrap(),
        };
        base_filds.push(temp_sctruct)
    }

    base_filds
}
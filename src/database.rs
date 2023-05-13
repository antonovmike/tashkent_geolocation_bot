use sqlite::State;
// name TEXT, summary TEXT, schedule TEXT, map TEXT, latitude TEXT, longitude TEXT
#[derive(Debug, Clone)]
pub struct Base {
    pub name: String,
    pub summ: String,
    pub sche: String,
    pub ggle: String,
    pub lttd: f64,
    pub lngt: f64,
}

pub async fn base_data() -> Vec<Base> {
    let connection = sqlite::open("db.sql").unwrap();
    let query = "SELECT * FROM museums";
    let mut statement = connection.prepare(query).unwrap();

    let mut base_filds: Vec<Base> = vec![];

    while let Ok(State::Row) = statement.next() {
        let temp_sctruct = Base {
            name: statement.read::<String, _>("name").unwrap(),
            summ: statement.read::<String, _>("summary").unwrap(),
            sche: statement.read::<String, _>("schedule").unwrap(),
            ggle: statement.read::<String, _>("map").unwrap(),
            lttd: statement.read::<f64, _>("latitude").unwrap(),
            lngt: statement.read::<f64, _>("longitude").unwrap(),
        };
        base_filds.push(temp_sctruct)
    }

    base_filds
}

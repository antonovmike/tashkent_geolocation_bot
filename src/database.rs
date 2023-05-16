use sqlite::{State, Error};

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

pub async fn base_data() -> Result<Vec<Base>, Error> {
    let connection = sqlite::open("db.sql")?;
    let query = "SELECT * FROM museums";
    let mut statement = connection.prepare(query)?;

    let mut base_filds: Vec<Base> = vec![];

    while let Ok(State::Row) = statement.next() {
        let temp_sctruct = Base {
            name: statement.read::<String, _>("name")?,
            summ: statement.read::<String, _>("summary")?,
            sche: statement.read::<String, _>("schedule")?,
            ggle: statement.read::<String, _>("map")?,
            lttd: statement.read::<f64, _>("latitude")?,
            lngt: statement.read::<f64, _>("longitude")?,
        };
        base_filds.push(temp_sctruct)
    }

    Ok(base_filds)
}

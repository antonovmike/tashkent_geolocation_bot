use sqlite::State;

pub struct Museums {
    pub name: String,
    pub summ: String,
    pub addr: String,
    pub tele: String,
    pub pric: String,
    pub sche: String,
    pub doff: String,
    pub site: String,
    pub gis2: String,
    pub ggle: String,
    pub lttd: f64,
    pub lngt: f64,
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
            addr: statement.read::<String, _>("address").unwrap(),
            tele: statement.read::<String, _>("telephone").unwrap(),
            pric: statement.read::<String, _>("price").unwrap(),
            sche: statement.read::<String, _>("schedule").unwrap(),
            doff: statement.read::<String, _>("dayoff").unwrap(),
            site: statement.read::<String, _>("website").unwrap(),
            gis2: statement.read::<String, _>("map_2gis").unwrap(),
            ggle: statement.read::<String, _>("map_google").unwrap(),
            lttd: statement.read::<f64, _>("latitude").unwrap(),
            lngt: statement.read::<f64, _>("longitude").unwrap(),
        };
        museums.push(temp_sctruct)
    }

    museums
}
use calamine::{open_workbook_auto, DataType, Reader};

pub fn to_base() {
    let path = "data.ods";
    let mut workbook = open_workbook_auto(path).unwrap();

    // Select the first worksheet
    let sheet_name = workbook.sheet_names()[0].to_owned();
    let range = workbook.worksheet_range(&sheet_name).unwrap();

    let mut table_content: Vec<Vec<DataType>> = vec![];

    for row in range.expect("REASON").rows() {
        table_content.push(row.to_vec());
    }

    let connection = sqlite::open("db.sql").unwrap();
    let query = "DROP TABLE IF EXISTS museums";
    connection.execute(query).unwrap();
    let query = "CREATE TABLE IF NOT EXISTS museums (name TEXT, summary TEXT, schedule TEXT, map TEXT, latitude TEXT, longitude TEXT);";
    connection.execute(query).unwrap();

    for museum in table_content {
        let name = museum[0].to_string().replace("'", "''");
        let summ = museum[1].to_string().replace("'", "''");

        let query = format!(
            "INSERT INTO museums VALUES ('{}', '{}', '{}', '{}', '{}', '{}');",
            name,
            summ,
            museum[2].to_string(),
            museum[3].to_string(),
            museum[4].to_string(),
            museum[5].to_string()
        );
        connection.execute(&query).unwrap();
    }
}

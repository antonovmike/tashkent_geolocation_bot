use calamine::{open_workbook_auto, DataType, Reader};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to open spreadsheet file: {0}")]
    TableErr(#[from] calamine::Error),

    #[error("sql error: {0}")]
    SqlError(#[from] sqlite::Error),
}

pub fn to_base() -> Result<(), Error>{
    let path = "data.ods";
    let mut workbook = open_workbook_auto(path)?;

    // Select the first worksheet
    let sheet_name = workbook.sheet_names()[0].to_owned();
    let range = workbook.worksheet_range(&sheet_name).unwrap();

    let mut table_content: Vec<Vec<DataType>> = vec![];

    for row in range.expect("REASON").rows() {
        table_content.push(row.to_vec());
    }

    let connection = sqlite::open("db.sql")?;
    let query = "DROP TABLE IF EXISTS museums";
    connection.execute(query)?;
    let query = "CREATE TABLE IF NOT EXISTS museums (name TEXT, summary TEXT, schedule TEXT, map TEXT, latitude TEXT, longitude TEXT);";
    connection.execute(query)?;

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
        connection.execute(&query)?;
    }

    Ok(())
}

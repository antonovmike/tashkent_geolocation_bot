use calamine::{open_workbook_auto, Reader};

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
    let path = "./data.ods";
    let mut workbook = open_workbook_auto(path).unwrap();
    let sheet_name = workbook.sheet_names()[0].to_owned();
    let range = workbook.worksheet_range(&sheet_name).unwrap();

    let mut base_filds: Vec<Base> = vec![];

    for row in range.expect("REASON").rows() {
        let temp_sctruct = Base {
            name: row[0].to_string(),
            summ: row[1].to_string(),
            sche: row[2].to_string(),
            ggle: row[3].to_string(),
            lttd: row[4].to_string().parse::<f64>().unwrap(),
            lngt: row[5].to_string().parse::<f64>().unwrap(),
        };
        base_filds.push(temp_sctruct)
    }

    base_filds
}
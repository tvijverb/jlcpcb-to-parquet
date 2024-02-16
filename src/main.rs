use sqlx::{sqlite::SqlitePool, Row};
use polars::prelude::*;
pub mod parse_resistance;
pub mod parse_inductance;
pub mod parse_capacitance;

// components table query: "PRAGMA table_info(components);"
// 0	lcsc	INTEGER	1
// 1	category_id	INTEGER	1
// 2	mfr	TEXT	1
// 3	package	TEXT	1
// 4	joints	INTEGER	1
// 5	manufacturer_id	INTEGER	1
// 6	basic	INTEGER	1
// 7	description	TEXT	1
// 8	datasheet	TEXT	1
// 9	stock	INTEGER	1
// 10	price	TEXT	1
// 11	last_update	INTEGER	1
// 12	extra	TEXT	0
// 13	flag	INTEGER	1
// 14	last_on_stock	INTEGER	1
// 15	preferred	INTEGER	1
// 16	resistance	INTEGER	0 // added by me
// 17	inductance	INTEGER	0 // added by me
// 18	capacitance	INTEGER	0 // added by me
// 19   dielectric  TEXT 0    // added by me
#[derive(Debug, Clone)]
struct Component {
    lcsc: i64,
    category_id: i64,
    mfr: String,
    package: String,
    joints: i64,
    manufacturer_id: i64,
    basic: i64,
    description: String,
    datasheet: String,
    stock: i64,
    price: String,
    last_update: i64,
    extra: Option<String>,
    flag: i64,
    last_on_stock: i64,
    preferred: i64,
    resistance: Option<i64>,
    inductance: Option<i64>,
    capacitance: Option<i64>,
    dielectric: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Open the SQLite database file
    let pool = SqlitePool::connect("sqlite:cache.sqlite3").await?;

    // Check if idx_components_lcsc index exists
    let row: Result<Option<(String,)>,sqlx::Error> = sqlx::query_as("DROP INDEX idx_components_lcsc")
        .fetch_optional(&pool).await;

    // if row.is_some() {
    //     sqlx::query("DROP INDEX idx_components_lcsc").execute(&pool).await?;
    sqlx::query("CREATE INDEX idx_components_lcsc ON components(lcsc)").execute(&pool).await?;

    // Check if resistance column exists, if not add resistance, inductance, capacitance and dielectric columns
    let row: Result<Option<i64>, sqlx::Error> = sqlx::query_scalar("SELECT resistance FROM components limit 1;")
        .fetch_optional(&pool).await;

    // if the query fails, the table does not exist, so create it
    if row.is_err() {
        sqlx::query("ALTER TABLE components ADD resistance INTEGER;").execute(&pool).await?;
        sqlx::query("ALTER TABLE components ADD inductance INTEGER;").execute(&pool).await?;
        sqlx::query("ALTER TABLE components ADD capacitance INTEGER;").execute(&pool).await?;
        sqlx::query("ALTER TABLE components ADD dielectric TEXT;").execute(&pool).await?;
    }

    let rows = sqlx::query("SELECT * from components where stock > 0").fetch_all(&pool).await?;

    let mut all_components: Vec<Component> = vec![];
    for row in rows {
        all_components.push(Component {
            lcsc: row.get::<i64,_>(0),
            category_id: row.get::<i64,_>(1),
            mfr: row.get::<String,_>(2),
            package: row.get::<String,_>(3),
            joints: row.get::<i64,_>(4),
            manufacturer_id: row.get::<i64,_>(5),
            basic: row.get::<i64,_>(6),
            description: row.get::<String,_>(7),
            datasheet: row.get::<String,_>(8),
            stock: row.get::<i64,_>(9),
            price: row.get::<String,_>(10),
            last_update: row.get::<i64,_>(11),
            extra: row.get::<Option<String>,_>(12),
            flag: row.get::<i64,_>(13),
            last_on_stock: row.get::<i64,_>(14),
            preferred: row.get::<i64,_>(15),
            resistance: row.get::<Option<i64>,_>(16),
            inductance: row.get::<Option<i64>,_>(17),
            capacitance: row.get::<Option<i64>,_>(18),
            dielectric: row.get::<Option<String>,_>(19),
        });
    };

    for component in all_components.iter_mut() {
        // Resistors are in subcategories 46-63
        if component.category_id >= 46 && component.category_id <= 63 {
            let resistance_value = parse_resistance::parse_resistance_description(&component.description);
            if let Some(value) = resistance_value {
                component.resistance = Some(value as i64);
            }
        }
        // Inductors are in subcategories 12-25
        if component.category_id >= 12 && component.category_id <= 25 {
            let inductance_value = parse_inductance::parse_inductance_description(&component.description);
            if let Some(value) = inductance_value {
                component.inductance = Some(value as i64);
            }
        }
        // Capacitors are in subcategories 26-45
        if component.category_id >= 26 && component.category_id <= 45 {
            let capacitance_value = parse_capacitance::parse_capacitance_description(&component.description);
            if let Some(value) = capacitance_value {
                component.capacitance = Some(value as i64);
            }
        }
        // Dielectric is only for capacitors
        if component.category_id >= 26 && component.category_id <= 45 {
            if component.description.contains("C0G") {
                component.dielectric = Some("C0G".to_string());
            } else if component.description.contains("X7R") {
                component.dielectric = Some("X7R".to_string());
            } else if component.description.contains("X5R") {
                component.dielectric = Some("X5R".to_string());
            } else if component.description.contains("Y5V") {
                component.dielectric = Some("Y5V".to_string());
            }
        }
    }

    let df_components = DataFrame::new(
        vec![
            Series::new("lcsc", all_components.iter().map(|c| c.lcsc).collect::<Vec<_>>()),
            Series::new("category_id", all_components.iter().map(|c| c.category_id).collect::<Vec<_>>()),
            Series::new("mfr", all_components.iter().map(|c| c.mfr.clone()).collect::<Vec<_>>()),
            Series::new("package", all_components.iter().map(|c| c.package.clone()).collect::<Vec<_>>()),
            Series::new("joints", all_components.iter().map(|c| c.joints).collect::<Vec<_>>()),
            Series::new("manufacturer_id", all_components.iter().map(|c| c.manufacturer_id).collect::<Vec<_>>()),
            Series::new("basic", all_components.iter().map(|c| c.basic).collect::<Vec<_>>()),
            Series::new("description", all_components.iter().map(|c| c.description.clone()).collect::<Vec<_>>()),
            Series::new("datasheet", all_components.iter().map(|c| c.datasheet.clone()).collect::<Vec<_>>()),
            Series::new("stock", all_components.iter().map(|c| c.stock).collect::<Vec<_>>()),
            Series::new("price", all_components.iter().map(|c| c.price.clone()).collect::<Vec<_>>()),
            Series::new("last_update", all_components.iter().map(|c| c.last_update).collect::<Vec<_>>()),
            // Series::new("extra", all_components.iter().map(|c| c.extra.clone()).collect::<Vec<_>>()),
            Series::new("flag", all_components.iter().map(|c| c.flag).collect::<Vec<_>>()),
            Series::new("last_on_stock", all_components.iter().map(|c| c.last_on_stock).collect::<Vec<_>>()),
            Series::new("preferred", all_components.iter().map(|c| c.preferred).collect::<Vec<_>>()),
            Series::new("resistance", all_components.iter().map(|c| c.resistance).collect::<Vec<_>>()),
            Series::new("inductance", all_components.iter().map(|c| c.inductance).collect::<Vec<_>>()),
            Series::new("capacitance", all_components.iter().map(|c| c.capacitance).collect::<Vec<_>>()),
            Series::new("dielectric", all_components.iter().map(|c| c.dielectric.clone()).collect::<Vec<_>>()),
        ]
    ).unwrap();
    let path = "components.parquet".into();
    df_components.lazy().sink_parquet(path, Default::default()).unwrap();


    Ok(())
}

use sqlx::Row;
pub mod category_models;
pub mod category_to_parquet;

use crate::categories::category_models::{CategoryTable, FilteredCategories};

pub async fn get_filtered_categories(pool: &sqlx::SqlitePool) -> Result<FilteredCategories, sqlx::Error> {
    let all_categories_sqliterow = sqlx::query("SELECT * FROM categories")
        .fetch_all(pool)
        .await?;

    let mut all_categories: Vec<CategoryTable> = Vec::with_capacity(all_categories_sqliterow.len());
    for row in all_categories_sqliterow {
        all_categories.push(CategoryTable {
            id: row.get::<i64, _>(0),
            category: row.get::<String, _>(1),
            subcategory: row.get::<String, _>(2),
        });
    }


    let mut smd_resistor_id: i64 = 0;
    let mut smd_capacitor_id: i64 = 0;
    let mut smd_inductor_id: i64 = 0;

    for category in all_categories {
        if category.subcategory == "Chip Resistor - Surface Mount" {
            smd_resistor_id = category.id;
        }
        if category.subcategory == "Multilayer Ceramic Capacitors MLCC - SMD/SMT" {
            smd_capacitor_id = category.id;
        }
        if category.category == "Inductors/Coils/Transformers" && category.subcategory == "Inductors (SMD)" {
            smd_inductor_id = category.id;
        }
    }

    // if any of the categories are not found, return an error
    if smd_resistor_id == 0 || smd_capacitor_id == 0 || smd_inductor_id == 0 {
        println!("Error: Could not find all categories. Should be non-zero.");
        println!("smd_resistor_id: {}", smd_resistor_id);
        println!("smd_capacitor_id: {}", smd_capacitor_id);
        println!("smd_inductor_id: {}", smd_inductor_id);
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(FilteredCategories {
        smd_resistor_id,
        smd_capacitor_id,
        smd_inductor_id,
    })

}
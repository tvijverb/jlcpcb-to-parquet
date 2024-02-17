use polars::prelude::*;
use crate::categories::category_models::FilteredCategories;

pub async fn category_to_parquet(
    df_components: LazyFrame,
    category_ids: FilteredCategories,
) -> Result<(), Box<dyn std::error::Error>> {
    let df_resistors = df_components
        .clone()
        .filter(col("category_id").eq(lit(category_ids.smd_resistor_id)));

    let df_capacitors = df_components
        .clone()
        .filter(col("category_id").eq(lit(category_ids.smd_capacitor_id)));

    let df_inductors = df_components
        .clone()
        .filter(col("category_id").eq(lit(category_ids.smd_inductor_id)));

    let path_resistors = "resistors.parquet".into();
    let path_capacitors = "capacitors.parquet".into();
    let path_inductors = "inductors.parquet".into();

    df_resistors.sink_parquet(path_resistors, Default::default()).unwrap();
    df_capacitors.sink_parquet(path_capacitors, Default::default()).unwrap();
    df_inductors.sink_parquet(path_inductors, Default::default()).unwrap();

    Ok(())
}
    
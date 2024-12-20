fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --8<-- [start:dataframe]
    use polars::prelude::*;
    let df = df! (
        "value" => &[Some(1), None],
    )?;

    println!("{}", df);
    // --8<-- [end:dataframe]

    // --8<-- [start:count]
    let null_count_df = df.null_count();
    println!("{}", null_count_df);
    // --8<-- [end:count]

    // --8<-- [start:isnull]
    let is_null_series = df
        .clone()
        .lazy()
        .select([col("value").is_null()])
        .collect()?;
    println!("{}", is_null_series);
    // --8<-- [end:isnull]

    // --8<-- [start:dataframe2]
    // Contribute the Rust translation of the Python example by opening a PR.
    // --8<-- [end:dataframe2]

    // --8<-- [start:fill]
    // Contribute the Rust translation of the Python example by opening a PR.
    // --8<-- [end:fill]

    // --8<-- [start:fillstrategy]
    // Contribute the Rust translation of the Python example by opening a PR.
    // --8<-- [end:fillstrategy]

    // --8<-- [start:fillexpr]
    // Contribute the Rust translation of the Python example by opening a PR.
    // --8<-- [end:fillexpr]

    // --8<-- [start:fillinterpolate]
    // Contribute the Rust translation of the Python example by opening a PR.
    // --8<-- [end:fillinterpolate]

    // --8<-- [start:nan]
    let nan_df = df!(
            "value" => [1.0, f64::NAN, f64::NAN, 3.0],
    )?;
    println!("{}", nan_df);
    // --8<-- [end:nan]

    // --8<-- [start:nan-computed]
    // Contribute the Rust translation of the Python example by opening a PR.
    // --8<-- [end:nan-computed]

    // --8<-- [start:nanfill]
    // Contribute the Rust translation of the Python example by opening a PR.
    // --8<-- [end:nanfill]
    Ok(())
}

use polars::prelude::*;

fn main() {
    // データ型の指定
    let mut schema = Schema::new();
    schema.with_column("customer_id".into(), DataType::Utf8);
    schema.with_column("gender_cd".into(), DataType::Utf8);
    schema.with_column("postal_cd".into(), DataType::Utf8);
    schema.with_column("application_store_cd".into(), DataType::Utf8);
    schema.with_column("status_cd".into(), DataType::Utf8);
    schema.with_column("category_major_cd".into(), DataType::Utf8);
    schema.with_column("category_medium_cd".into(), DataType::Utf8);
    schema.with_column("category_small_cd".into(), DataType::Utf8);
    schema.with_column("product_cd".into(), DataType::Utf8);
    schema.with_column("store_cd".into(), DataType::Utf8);
    schema.with_column("prefecture_cd".into(), DataType::Utf8);
    schema.with_column("tel_no".into(), DataType::Utf8);
    schema.with_column("postal_cd".into(), DataType::Utf8);
    schema.with_column("street".into(), DataType::Utf8);
    schema.with_column(
        "birth_day".into(),
        DataType::Date
    );
    schema.with_column(
        "application_date".into(),
        DataType::Utf8
    );

    let df_customer = LazyCsvReader::new("data/customer.csv")
        .has_header(true)
        .with_dtype_overwrite(Some(&schema.clone()))
        .finish()
        .unwrap();
    let df_category = LazyCsvReader::new("data/category.csv")
        .has_header(true)
        .with_dtype_overwrite(Some(&schema.clone()))
        .finish()
        .unwrap();
    let df_product = LazyCsvReader::new("data/product.csv")
        .has_header(true)
        .with_dtype_overwrite(Some(&schema.clone()))
        .finish()
        .unwrap();
    let df_receipt = LazyCsvReader::new("data/receipt.csv")
        .has_header(true)
        .with_dtype_overwrite(Some(&schema.clone()))
        .finish()
        .unwrap();
    let df_store = LazyCsvReader::new("data/store.csv")
        .has_header(true)
        .with_dtype_overwrite(Some(&schema.clone()))
        .finish()
        .unwrap();
    let df_geocode = LazyCsvReader::new("data/geocode.csv")
        .has_header(true)
        .with_dtype_overwrite(Some(&schema.clone()))
        .finish()
        .unwrap();

}


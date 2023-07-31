use polars::prelude::*;

pub fn test(df_product: LazyFrame) {

    println!(
        "{:?}",    
        df_product.clone()
            .fill_null(FillNullStrategy::Mean.)
            .null_count()
            .collect()
            .unwrap()
    );
}
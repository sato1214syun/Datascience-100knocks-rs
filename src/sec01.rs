use polars::prelude::*;

pub fn test(df_receipt: LazyFrame) {
    let df_ts_amount = df_receipt.clone()
    .groupby([col("sales_ymd") % lit(100)]).agg([col("amount").sum()])
    .sort(
        "sales_ymd",
        SortOptions {
            descending: false,
            nulls_last: true,
            multithreaded: true,
            maintain_order: true
        }
    );

    fn split_data(df: LazyFrame, train_size: u32, test_size: u32, slide_window: i64, start_point: i64) -> (LazyFrame, LazyFrame) {
        let train_start = start_point * slide_window;
        let test_start = train_start + train_size as i64;
        (df.clone().slice(train_start, train_size), df.clone().slice(test_start, test_size))
    }


    let (df_train_1, df_test_1) = split_data(df_ts_amount.clone(), 12, 6, 6, 0);
    let (df_train_2, df_test_2) = split_data(df_ts_amount.clone(), 12, 6, 6, 1);
    let (df_train_3, df_test_3) = split_data(df_ts_amount.clone(), 12, 6, 6, 2);

    println!(
        "df_train1:\n{:?}\ndf_test1:\n{:?}\ndf_train2:\n{:?}\ndf_test2:\n{:?}\ndf_train3:\n{:?}\ndf_test3:\n{:?}\n",    
        df_train_1.collect().unwrap(),
        df_test_1.collect().unwrap(),
        df_train_2.collect().unwrap(),
        df_test_2.collect().unwrap(),
        df_train_3.collect().unwrap(),
        df_test_3.collect().unwrap(),
    );
}
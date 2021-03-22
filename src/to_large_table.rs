

// rustc emits the following error messages:
// ```
// error[E0277]: the trait bound `(columns::id, columns::column_1, columns::column_2, columns::column_3, columns::column_4, columns::column_5, columns::column_6, columns::column_7, columns::column_8, columns::column_9, columns::column_10, columns::column_11, columns::column_12, columns::column_13, columns::column_14, columns::column_15, columns::column_16): diesel::Expression` is not satisfied
//    --> src/to_large_table.rs:3:1
//     |
// 3   | / table! {
// 4   | |     to_many_columns {
// 5   | |         id -> Integer,
// 6   | |         column_1 -> Text,
// ...   |
// 22  | |     }
// 23  | | }
//     | |_^ the trait `diesel::Expression` is not implemented for `(columns::id, columns::column_1, columns::column_2, columns::column_3, columns::column_4, columns::column_5, columns::column_6, columns::column_7, columns::column_8, columns::column_9, columns::column_10, columns::column_11, columns::column_12, columns::column_13, columns::column_14, columns::column_15, columns::column_16)`
//     |
//    ::: /home/weiznich/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.4.6/src/query_builder/mod.rs:247:23
//     |
// 247 |       type Query: Query<SqlType = Self::SqlType>;
//     |                         ----------------------- required by this bound in `diesel::query_builder::AsQuery::Query`
//     |
//     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

// error[E0277]: the trait bound `(columns::id, columns::column_1, columns::column_2, columns::column_3, columns::column_4, columns::column_5, columns::column_6, columns::column_7, columns::column_8, columns::column_9, columns::column_10, columns::column_11, columns::column_12, columns::column_13, columns::column_14, columns::column_15, columns::column_16): NonAggregate` is not satisfied
//   --> src/to_large_table.rs:3:1
//    |
// 3  | / table! {
// 4  | |     to_many_columns {
// 5  | |         id -> Integer,
// 6  | |         column_1 -> Text,
// ...  |
// 22 | |     }
// 23 | | }
//    | |_^ the trait `NonAggregate` is not implemented for `(columns::id, columns::column_1, columns::column_2, columns::column_3, columns::column_4, columns::column_5, columns::column_6, columns::column_7, columns::column_8, columns::column_9, columns::column_10, columns::column_11, columns::column_12, columns::column_13, columns::column_14, columns::column_15, columns::column_16)`
//    |
//   ::: /home/weiznich/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.4.6/src/query_source/mod.rs:62:51
//    |
// 62 |       type AllColumns: SelectableExpression<Self> + NonAggregate;
//    |                                                     ------------ required by this bound in `diesel::Table::AllColumns`
//    |
//    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

// error[E0277]: the trait bound `(columns::id, columns::column_1, columns::column_2, columns::column_3, columns::column_4, columns::column_5, columns::column_6, columns::column_7, columns::column_8, columns::column_9, columns::column_10, columns::column_11, columns::column_12, columns::column_13, columns::column_14, columns::column_15, columns::column_16): SelectableExpression<table>` is not satisfied
//   --> src/to_large_table.rs:3:1
//    |
// 3  | / table! {
// 4  | |     to_many_columns {
// 5  | |         id -> Integer,
// 6  | |         column_1 -> Text,
// ...  |
// 22 | |     }
// 23 | | }
//    | |_^ the trait `SelectableExpression<table>` is not implemented for `(columns::id, columns::column_1, columns::column_2, columns::column_3, columns::column_4, columns::column_5, columns::column_6, columns::column_7, columns::column_8, columns::column_9, columns::column_10, columns::column_11, columns::column_12, columns::column_13, columns::column_14, columns::column_15, columns::column_16)`
//    |
//   ::: /home/weiznich/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.4.6/src/query_source/mod.rs:62:22
//    |
// 62 |       type AllColumns: SelectableExpression<Self> + NonAggregate;
//    |                        -------------------------- required by this bound in `diesel::Table::AllColumns`
//    |
//    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

// error[E0277]: the trait bound `(columns::id, columns::column_1, columns::column_2, columns::column_3, columns::column_4, columns::column_5, columns::column_6, columns::column_7, columns::column_8, columns::column_9, columns::column_10, columns::column_11, columns::column_12, columns::column_13, columns::column_14, columns::column_15, columns::column_16): SelectableExpression<table>` is not satisfied
//   --> src/to_large_table.rs:3:1
//    |
// 3  | / table! {
// 4  | |     to_many_columns {
// 5  | |         id -> Integer,
// 6  | |         column_1 -> Text,
// ...  |
// 22 | |     }
// 23 | | }
//    | |_^ the trait `SelectableExpression<table>` is not implemented for `(columns::id, columns::column_1, columns::column_2, columns::column_3, columns::column_4, columns::column_5, columns::column_6, columns::column_7, columns::column_8, columns::column_9, columns::column_10, columns::column_11, columns::column_12, columns::column_13, columns::column_14, columns::column_15, columns::column_16)`
//    |
//   ::: /home/weiznich/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.4.6/src/query_source/mod.rs:35:28
//    |
// 35 |       type DefaultSelection: SelectableExpression<Self>;
//    |                              -------------------------- required by this bound in `diesel::QuerySource::DefaultSelection`
//    |
//    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
//
// error: aborting due to 4 previous errors
// ```
//
// All of them are basically irrelevant. The only relevant point here is that these impl exist
// for certain tuple sizes, but not for a tuple with 17 elements. It would be
// helpful for users to read something like "We found the required impl for tuples up to 16 elements,
// but your tuple has 17 elements"

table! {
    to_many_columns {
        id -> Integer,
        column_1 -> Text,
        column_2 -> Text,
        column_3 -> Text,
        column_4 -> Text,
        column_5 -> Text,
        column_6 -> Text,
        column_7 -> Text,
        column_8 -> Text,
        column_9 -> Text,
        column_10 -> Text,
        column_11 -> Text,
        column_12 -> Text,
        column_13 -> Text,
        column_14 -> Text,
        column_15 -> Text,
        column_16 -> Text,
    }
}

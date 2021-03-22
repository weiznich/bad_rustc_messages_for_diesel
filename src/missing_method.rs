use diesel::prelude::*;

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

// rustc emits the following error message:
// ```
// error[E0277]: the trait bound `Sqlite: SupportsReturningClause` is not satisfied
//   --> src/missing_method.rs:13:10
//    |
// 13 |         .get_result(conn)
//    |          ^^^^^^^^^^ the trait `SupportsReturningClause` is not implemented for `Sqlite`
//    |
//    = note: required because of the requirements on the impl of `QueryFragment<Sqlite>` for `diesel::query_builder::returning_clause::ReturningClause<(columns::id, columns::name)>`
//    = note: 1 redundant requirements hidden
//    = note: required because of the requirements on the impl of `QueryFragment<Sqlite>` for `InsertStatement<table, ValuesClause<ColumnInsertValue<columns::name, diesel::expression::bound::Bound<diesel::sql_types::Text, &str>>, table>, diesel::query_builder::insert_statement::Insert, diesel::query_builder::returning_clause::ReturningClause<(columns::id, columns::name)>>`
//    = note: required because of the requirements on the impl of `LoadQuery<diesel::SqliteConnection, _>` for `InsertStatement<table, ValuesClause<ColumnInsertValue<columns::name, diesel::expression::bound::Bound<diesel::sql_types::Text, &str>>, table>>`
// ```
// I should probably point out that I personally find this error message is
// already quite good as it clearly states that `Sqlite` (the backend type)
// does not support a certain sql construct.
//
// That written all of the `required because of …` lines are not really relevant here.
// They can likely be omitted for third party crates.

fn returning(conn: &SqliteConnection) {
    let _: (i32, String) = diesel::insert_into(users::table)
        .values(users::name.eq("foo"))
        .get_result(conn)
        .unwrap();
}

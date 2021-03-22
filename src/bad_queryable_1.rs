use diesel::prelude::*;


table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

#[derive(Queryable)]
struct User {
    name: String,
    id: i32,
    something_else: Vec<String>,
}


// rustc emits the following error message here:
// ```
// error[E0277]: the trait bound `(String, i32, Vec<String>): Queryable<(diesel::sql_types::Integer, diesel::sql_types::Text), Pg>` is not satisfied
//  --> src/bad_queryable_1.rs:19:18
//   |
// 19 |     users::table.load(conn).unwrap()
//    |                  ^^^^ the trait `Queryable<(diesel::sql_types::Integer, diesel::sql_types::Text), Pg>` is not implemented for `(String, i32, Vec<String>)`
//    |
//    = help: the following implementations were found:
//              <(A, B, C) as Queryable<(SA, SB, SC), __DB>>
//              <(A, B, C) as Queryable<Record<(SA, SB, SC)>, Pg>>
//    = note: required because of the requirements on the impl of `Queryable<(diesel::sql_types::Integer, diesel::sql_types::Text), Pg>` for `User`
//    = note: required because of the requirements on the impl of `LoadQuery<diesel::PgConnection, User>` for `table`
// ```
//
// For this simple example I feel that this is quite readable, but as soon as there are much more fields involved
// the error message can become really large as this includes two times a tuple of the size of the field list/select clause
//
// I cannot propose concrete suggestions how to improve this for a generic context. With diesel specific knowlegde it is quite easy to
// transform this into something much more readable like:
//
//  You've tried to map the result of a query to an incompatible struct. We've found the following missmatches:
//  * Tried to load a `Integer` value into a `String` field
//  * Tried to load a `Text` value into a `i32` field
//  * Field count missmatch between your result set (2 fields) and your rust struct (3 fields)
fn test_query(conn: &PgConnection) -> Vec<User> {
    users::table.load(conn).unwrap()
}

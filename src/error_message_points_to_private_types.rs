use diesel::prelude::*;


table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

// rustc emits the following error message here
// ```
// error[E0308]: mismatched types
//   --> src/error_message_points_to_private_types.rs:13:5
//    |
// 12 | fn private_type(conn: &PgConnection) -> () {
//    |                                         -- expected `()` because of return type
// 13 |     users::table.select(users::id)
//    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try adding a semicolon: `;`
//    |     |
//    |     expected `()`, found struct `diesel::query_builder::SelectStatement`
//    |
//    = note: expected unit type `()`
//                  found struct `diesel::query_builder::SelectStatement<table, query_builder::select_clause::SelectClause<columns::id>>`
// ```
// The problem with that error message is that `query_builder::select_clause::SelectClause` points
// to a private symbol, that cannot be used by any third party crate. Additionally
// `SelectStatement` is `#[doc(hidden)]` which means it's not part
// of diesels public API (following the reasoning of large parts
// of the rust ecosystem).
//
// A better error message would use the types provided by the actual method definitions in
// diesel (which points to type defs in `diesel::dsl` using trait associated types
// to resolve the actual "unnamable" type).

fn private_type(conn: &PgConnection) -> () {
    users::table.select(users::id)
}

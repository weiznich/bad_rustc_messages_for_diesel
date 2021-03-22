use diesel::prelude::*;
use diesel::sql_types::Text;

#[derive(QueryableByName)]
struct User {
    #[sql_type = "Text"]
    name: String,
}


// rustc emits the following error
// ```
// error[E0275]: overflow evaluating the requirement `<query_builder::sql_query::UncheckedBind<SqlQuery, &str, diesel::sql_types::Text> as LimitDsl>::Output == _`
//   --> src/bad_queryable_by_name_1.rs:12:31
//    |
// 12 |     query.bind::<Text, _>("").first::<User>(conn).unwrap();
//    |                               ^^^^^
// ```
//
// The underlying problem here is that diesel does not provide a impl of the trait
// providing the `first` method. Instead of throwing a overflow error
// rustc should just state this.
fn test(conn: &PgConnection) {
    let query = diesel::sql_query("SELECT * FROM users WHERE name = $1");
    query.bind::<Text, _>("").first::<User>(conn).unwrap();
}

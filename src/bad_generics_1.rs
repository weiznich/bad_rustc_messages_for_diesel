use diesel::associations::HasTable;
use diesel::connection::Connection;
use diesel::deserialize::QueryableByName;
use diesel::query_builder::AsQuery;
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::query_dsl::QueryDsl;
use diesel::Table;

// rustc emits the following error messages here:
// ```
// error[E0391]: cycle detected when computing the bounds for type parameter `T`
//   --> src/bad_generics_1.rs:15:24
//    |
// 15 |           T: SelectDsl<T::AllColumns>,
//    |                        ^^^^^^^^^^^^^
//    |
//    = note: ...which again requires computing the bounds for type parameter `T`, completing the cycle
// note: cycle used when computing explicit predicates of `bad_generics_1::get_all`
//   --> src/bad_generics_1.rs:15:24
//    |
// 15 |           T: SelectDsl<T::AllColumns>,
//    |                        ^^^^^^^^^^^^^
// ```
//
// Hard to say exactly what could be improved for this error message,
// For reference the corrected coded
// ```
// pub fn get_all<DB, T, I, Conn>(conn: &Conn) -> Result<Vec<I>, diesel::result::Error>
// where
//     Conn: Connection<Backend = DB>,
//     DB: diesel::backend::Backend + HasSqlType<T::SqlType>,
//     T: Table + AsQuery,
//     T::Query: QueryFragment<DB> + QueryId,
//     I: HasTable<Table = T> + Queryable<T::SqlType, DB>,
// {
//     I::table().load(conn)
// }
// ```

pub fn get_all<DB, T, I>(conn: impl Connection) -> Result<Vec<I>, diesel::result::Error>
where
    DB: diesel::backend::Backend,
    T: Table + AsQuery,
    I: HasTable<Table = T> + QueryableByName<DB>,
    T: SelectDsl<T::AllColumns>,
{
    let select = I::table().select();
    select.load(conn)
}

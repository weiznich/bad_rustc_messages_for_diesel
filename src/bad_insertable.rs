

table! {
    users {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

// Rustc emits the following error message here.
// ```
// error[E0277]: the trait bound `i32: diesel::Expression` is not satisfied
// --> src/bad_insertable.rs:11:10
//    |
// 11 | #[derive(Insertable)]
//    |          ^^^^^^^^^^ the trait `diesel::Expression` is not implemented for `i32`
//    |
//    = note: required because of the requirements on the impl of `AsExpression<diesel::sql_types::Text>` for `i32`
//    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
// ```
//
// Pointing to `Expression` is technically correct here as there is a generic
// `impl<T> AsExpression<T::SqlType> for T where T: Expression>`
//
// In the end this is not really helpful for an actual user. The underlying
// issue is that there is no `AsExpression<Text>` impl for `i32`, as this is bound
// on a unfullfilld `i32: ToSql<Text, DB>` trait impl.
//
// Basically this error message says: You cannot insert a `i32` into a database field
// expecting a `Text` value, as this are non compatible types.
//
// (You can replace the actual types here for something else that is not compatible)


#[derive(Insertable)]
#[table_name = "users"]
struct NewUser {
    name: i32,
    hair_color: Option<String>,
}

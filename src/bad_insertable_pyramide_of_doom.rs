use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;

// rustc emits the following error message:
// ```
// error[E0275]: overflow evaluating the requirement `&Vec<_>: diesel::Insertable<_>`
//   |
//   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`bad_rustc_messages_for_diesel`)
//   = note: required because of the requirements on the impl of `diesel::Insertable<_>` for `(&Vec<_>, &_, &_, &_, &_, &_, &_, &_, &_, &_, &_, &_, &_, &_, &_, &_)`
//   = note: 127 redundant requirements hidden
//   = note: required because of the requirements on the impl of `diesel::Insertable<
//           _>` for `&((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((Vec<_>, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _)`
// ```
//
// First of all that error message is a lot more readable than the last time I've hit.
// That said, it's not really helpful as it only says that does not work.
//
// I'm not sure how a better error message should look like, as the underlying
// problem is a short comming in rust itself, as it is not possible to
// reuse the lifetime `'b` to write a trait bound on an associated type.
// Basically the `I::Values: QueryFragment<Pg>` bound is missing from this
// where clause, but there is no way to write that using the current
// rust version. Given that the compiler should likely just emit
// a more helpful message that you cannot write the required bound
// or something like that.

fn generic_insertable<I, T>(i: I, t: T, conn: &PgConnection)
where
    for<'b> &'b I:Insertable<T>,
    T: Table,
    T::FromClause: QueryFragment<Pg>,
{
    diesel::insert_into(t).values(&i).execute(conn);
}

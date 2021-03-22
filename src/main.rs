#[macro_use]
extern crate diesel;

mod bad_generics_1;
mod bad_insertable;
mod bad_insertable_pyramide_of_doom;
mod bad_queryable_1;
mod bad_queryable_by_name_1;
mod error_message_points_to_private_types;
mod missing_method;
mod to_large_table;

fn main() {
    println!("Hello, world!");
}

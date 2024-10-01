use crate::models::*;
use crate::schema;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_numbers(new_numbers: Vec<Number>) {
    use self::schema::numbers::dsl::*;

    let connection = &mut establish_connection();

    diesel::insert_or_ignore_into(numbers)
        .values(new_numbers)
        .execute(connection)
        .map_err(|e| {
            println!("{:?}", e);
        })
        .expect("Error inserting numbers");
}

pub fn update_number(number: Number) {
    use self::schema::numbers::dsl::*;
    let connection = &mut establish_connection();

    println!("update_number -> {:?}", number);

    diesel::update(numbers.filter(digits.eq(number.digits)))
        .set((carrier.eq(number.carrier), email.eq(number.email)))
        .execute(connection)
        .map_err(|e| {
            println!("{:?}", e);
        })
        .expect("Error updating number");
}

pub fn get_number(digits_: String) -> Option<Number> {
    use self::schema::numbers::dsl::*;
    let connection = &mut establish_connection();

    match numbers
        .filter(digits.eq(digits_))
        .select(Number::as_select())
        .first::<Number>(connection)
    {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

pub fn delete_all_numbers() {
    use self::schema::numbers::dsl::*;
    let connection = &mut establish_connection();

    diesel::delete(numbers)
        .execute(connection)
        .map_err(|e| {
            println!("{:?}", e);
        })
        .expect("Error wiping numbers table");
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NumberFilter {
    pub digits: Option<String>,
    pub is_valid: Option<bool>,
    pub carrier: Option<String>,
}
pub fn get_numbers(filters: NumberFilter) -> Vec<Number> {
    use self::schema::numbers::dsl::*;

    let connection = &mut establish_connection();

    let mut query = numbers.into_boxed();

    if let Some(_digits) = filters.digits {
        query = query.filter(digits.eq(_digits));
    }

    if let Some(_is_valid) = filters.is_valid {
        query = query.filter(is_valid.eq(_is_valid));
    }

    if let Some(_carrier) = filters.carrier {
        query = query.filter(carrier.eq(_carrier));
    }

    query
        .order_by(id.asc())
        .select(Number::as_select())
        .load::<Number>(connection)
        .unwrap_or(vec![])
}

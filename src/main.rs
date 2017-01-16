#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate delila;

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;

use delila::models::*;
use delila::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use rocket_contrib::JSON;
use delila::schema::database::dsl::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/databases")]
fn databases() -> Result<JSON<Vec<Database>>, Error> {
    let connection = establish_connection();
    let res = database.load::<Database>(&connection);
    match res {
        Ok(databases) => Ok(JSON(databases)),
        Err(e) => Err(e)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, databases]).launch();
}

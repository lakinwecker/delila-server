// delila - a desktop version of lila.
// 
// Copyright (C) 2017 Lakin Wecker <lakin@wecker.ca>
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(custom_derive)]
#![recursion_limit = "1024"]

             extern crate app_dirs;
             extern crate chrono;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate diesel_migrations;
             extern crate hyper;
             extern crate serde;
             extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate slog;
             extern crate slog_async;
             extern crate slog_term;
             extern crate ws;

#[macro_use] extern crate error_chain;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub mod app_info;
pub mod errors;
pub mod models;
pub mod pathsettings;
pub mod schema;
pub mod scid;
pub mod tasks;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

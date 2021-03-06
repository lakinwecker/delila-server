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

//------------------------------------------------------------------------------
// The various tasks that the server support.
//------------------------------------------------------------------------------
pub mod import;
pub mod initialize;

use std::sync::Arc;

use diesel::sqlite::SqliteConnection;
use serde;
use serde_json;
use slog;
use ws::Sender;

use errors::*;
use super::pathsettings::{PathSettings};
use super::establish_connection;

#[derive(Clone)]
pub struct Request {
    pub id: u32,
    pub name: String,
    pub out: Sender,
    pub log: slog::Logger,
    pub path_settings: PathSettings
}
impl Request {
    fn send<T>(&self, method_name: String, args: &T) -> Result<()>
        where T: serde::Serialize
    {
        serde_json::to_string(&args).chain_err(
            || "Unable to serialize outoing args"
        ).and_then(|args| {
            let message = Message{name: method_name, id: self.id, args: args};
            serde_json::to_string(&message).chain_err(
                || "Unable to serialize outoing message"
            ).and_then(|outgoing|  {
                self.out.send(outgoing).chain_err(
                    || "Unable to send message"
                )
            })
        })
    }
    fn get_connection(&self) -> SqliteConnection {
        establish_connection(&self.path_settings.database_path.to_str().unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub name: String,
    pub id: u32,
    pub args: String
}

// A function that takes a string (representing the JSON arguments)
// And returns an optional thread. If a thread is returned the task has been
// backgrounded.
//
// TODO: Make this type safe like rocket.rs
pub trait RequestDispatch {
    fn dispatch(&self, request: &Request, args: String) -> Result<()>;
}

pub struct JSONDispatch<T> where T: serde::de::DeserializeOwned {
    pub handler: Arc<Fn(&Request, T) -> Result<()> + Send + Sync>
}

impl<T> RequestDispatch for JSONDispatch<T> where T: serde::de::DeserializeOwned {
    fn dispatch(&self, request: &Request, args: String) -> Result<()> {
        info!(request.log, "Marshalling arguments");
        serde_json::from_str(&args).chain_err(
            || "Unable to parse incoming json into the given argument type"
        ).and_then(|args| {
            info!(request.log, "Invoking handler");
            (self.handler)(&request, args)
        })
    }
}


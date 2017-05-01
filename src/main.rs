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

#![feature(plugin)]
#![recursion_limit = "1024"]

extern crate delila;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate diesel_codegen;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

// Not ours
extern crate ws;

use std::rc::Rc;
use std::cell::Cell;
use std::collections::HashMap;

use ws::{listen, Handler, Sender, Result as WsResult, Message, Handshake, CloseCode, Error as WsError};

use diesel::prelude::*;

// Ours
use delila::models::*;
use delila::schema::database::dsl::*;
use delila::establish_connection;
use delila::tasks::{Request, RequestHandler, RequestDispatch, JSONDispatch};
use delila::tasks::{
    importfile
};

pub mod errors;
use delila::errors::*;

#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    name: String,
    id: u32,
    args: String
}


struct Router
{
    out: Sender,
    commands: HashMap<String, Box<RequestDispatch>>
}


impl Handler for Router {

    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        // We have a new connection, so we increment the connection counter
        
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> WsResult<()> {
        match msg {
            Message::Text(txt) => {
                let incoming: IncomingMessage = serde_json::from_str(&txt).unwrap();
                let dispatcher: &Box<RequestDispatch> = self.commands.get(&incoming.name).unwrap();
                let request: Request = Request{id: incoming.id, name: incoming.name, out: self.out.clone()};
                let x = dispatcher.dispatch(request, incoming.args);
            },
            Message::Binary(b) => {
                println!("Unable to handle binary messages!");
            }
        }

        // Echo the message back
        self.out.send("Success!")
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: WsError) {
        println!("The server encountered an error: {:?}", err);
    }
}

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
} 

fn run() -> Result<()> {
    listen("127.0.0.1:3012", |out| {
        let mut commands: HashMap<String, Box<RequestDispatch>> = HashMap::new();
        commands.insert("importFile".into(),
            Box::new(
                JSONDispatch::<importfile::ImportFileArgs>{handler: Box::new(importfile::handler)}
            )
        );
        Router { out: out, commands: commands }
    }).chain_err(|| "Unable to start server")
}


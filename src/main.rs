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

//#[macro_use]
//extern crate diesel_codegen;

//#[macro_use]
//extern crate diesel;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate ws;

extern crate futures;
extern crate futures_cpupool;

// Not ours
use std::sync::Arc;
use std::collections::HashMap;

// use diesel::prelude::*;

use futures::{Async, Future};
use futures_cpupool::{CpuPool, CpuFuture};

// Ours
//use delila::models::*;
//use delila::schema::database::dsl::*;
//use delila::establish_connection;
use delila::tasks::{Message, Request, RequestDispatch, JSONDispatch};
use delila::tasks::{
    importfile
};

pub mod errors;
use delila::errors::*;


struct Router
{
    out: ws::Sender,
    commands: HashMap<String, Arc<RequestDispatch + Send + Sync>>,
    pool: CpuPool,
    futures: std::vec::Vec<CpuFuture<(), Error>>
}


impl ws::Handler for Router {

    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        // We have a new connection, so we increment the connection counter
        
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        match msg {
            ws::Message::Text(txt) => {
                for i in self.futures.len()..0 {
                    match self.futures[i].poll() {
                        Ok(Async::NotReady) => { },
                        Ok(Async::Ready(_)) => { self.futures.swap_remove(i); },
                        Err(_) => { self.futures.swap_remove(i); }
                    }
                }
                let incoming: Message = serde_json::from_str(&txt).unwrap();
                let dispatcher = self.commands.get(&incoming.name).unwrap().clone();
                let request: Request = Request{id: incoming.id, name: incoming.name, out: self.out.clone()};
                let args = incoming.args.clone();
                let future = self.pool.spawn_fn(move || {
                    dispatcher.dispatch(request, args)
                });
                self.futures.push(future);
            },
            ws::Message::Binary(b) => {
                println!("Unable to handle binary messages!");
            }
        }

        // Echo the message back
        self.out.send("Success!")
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        match code {
            ws::CloseCode::Normal => println!("The client is done with the connection."),
            ws::CloseCode::Away   => println!("The client is leaving the site."),
            ws::CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: ws::Error) {
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
    ws::listen("127.0.0.1:3012", |out| {
        let mut commands: HashMap<String, Arc<RequestDispatch + Send + Sync>> = HashMap::new();
        commands.insert("importFile".into(),
            Arc::new(
                JSONDispatch::<importfile::File>{handler: Arc::new(importfile::handler)}
            )
        );
        Router {
            out: out,
            commands: commands,
            pool: CpuPool::new_num_cpus(),
            futures: std::vec::Vec::new()
        }
    }).chain_err(|| "Unable to start server")
}


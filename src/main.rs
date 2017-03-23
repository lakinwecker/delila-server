#![feature(plugin)]

extern crate delila;

#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;

use delila::models::*;
use delila::establish_connection;
use diesel::prelude::*;
use delila::schema::database::dsl::*;

// A WebSocket echo server
extern crate ws;

use std::rc::Rc;
use std::cell::Cell;

use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

struct Server {
    out: Sender
}

impl Handler for Server {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("The message was: {:?}", msg);

        // Echo the message back
        self.out.send(msg)
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

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

}

fn main() {
    listen("127.0.0.1:3012", |out| { Server { out: out }}).unwrap()
} 

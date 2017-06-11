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

#![cfg_attr(feature="clippy", plugin(clippy))]

             extern crate app_dirs;
             extern crate chrono;
             extern crate diesel;
#[macro_use] extern crate error_chain;
             extern crate futures;
             extern crate futures_cpupool;
             extern crate hyper;
             extern crate serde;
             extern crate serde_json;
#[macro_use] extern crate slog;
             extern crate slog_async;
             extern crate slog_term;
             extern crate ws;

             extern crate delila;

// Not ours
use chrono::Local;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::sync::Arc;

use futures::{Async, Future};
use futures_cpupool::{CpuPool, CpuFuture};

use slog::Drain;

use diesel::migrations::setup_database;

// Ours
use delila::tasks::{Message, Request, RequestDispatch, JSONDispatch};
use delila::tasks::{
    initialize,
    import,
};
use delila::app_info::{DELILA_VERSION};
use delila::pathsettings::{PathSettings};
use delila::establish_connection;

pub mod errors;
use delila::errors::*;

macro_rules! today {
    () => ( Local::now().format("%Y-%m-%d") )
}


struct Server
{
    out: ws::Sender,
    commands: HashMap<String, Arc<RequestDispatch + Send + Sync>>,
    pool: CpuPool,
    futures: std::vec::Vec<CpuFuture<(), Error>>,
    path_settings: PathSettings,
    log: slog::Logger
}


impl ws::Handler for Server {

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
                        Ok(Async::Ready(_)) | Err(_) => { self.futures.swap_remove(i); }
                    }
                }
                let incoming: Message = serde_json::from_str(&txt).unwrap();
                let dispatcher = self.commands[&incoming.name].clone();
                let request: Request = Request{
                    id: incoming.id,
                    name: incoming.name.clone(),
                    out: self.out.clone(),
                    log: self.log.new(o!(
                        "name" => incoming.name,
                        "id" => incoming.id
                    )),
                    path_settings: self.path_settings.clone()
                };
                let args = incoming.args.clone();
                let future = self.pool.spawn_fn(move || {
                    dispatcher.dispatch(&request, args)
                });
                self.futures.push(future);
            },
            ws::Message::Binary(_) => {
                println!("Unable to handle binary messages!");
            }
        }

        Ok(()) 
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
    configure_directories()
    .and_then(|path_settings| {
        configure_logging(&path_settings.logging_path)
        .and_then(|log| {
            ensure_database_exists(&path_settings.database_path)
            .and_then(|_| {
                run_server(&path_settings, &log)
            })
        })
    })
}

//--------------------------------------------------------------------------------------------------
fn ensure_database_exists(db_path: &std::path::PathBuf) -> Result<()> {
    if db_path.exists() {
        return Ok(())
    }
    let conn = establish_connection(&db_path.to_str().unwrap());
    setup_database(&conn);
    Ok(())
}

//--------------------------------------------------------------------------------------------------
fn run_server(path_settings: &PathSettings, log: &slog::Logger) -> Result<()> {
    info!(log, "Starting Server");
    ws::listen("127.0.0.1:3012", |out| {
        info!(log, "Listening on 127.0.0.1:3012");

        let mut commands: HashMap<String, Arc<RequestDispatch + Send + Sync>> = HashMap::new();
        commands.insert("import::importFile".into(),
            Arc::new(JSONDispatch{handler: Arc::new(import::import_file)})
        );
        commands.insert("initialize::initialize".into(),
            Arc::new(JSONDispatch{handler: Arc::new(initialize::initialize)})
        );
        Server {
            out: out,
            commands: commands,
            pool: CpuPool::new_num_cpus(),
            futures: std::vec::Vec::new(),
            path_settings: path_settings.clone(),
            log: log.clone()
        }
    }).chain_err(|| "Unable to start server")
}

//--------------------------------------------------------------------------------------------------
fn configure_directories() -> Result<PathSettings> {
    PathSettings::new()
}

//--------------------------------------------------------------------------------------------------
fn configure_logging(logging_path: &std::path::PathBuf) -> Result<slog::Logger> {
    let mut log_directory = logging_path.clone();
    log_directory.push(format!("delila.{}.log", today!()));
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_directory.as_path())
        .and_then(|file| {
            let plain_decorator = slog_term::PlainDecorator::new(file);
            let file_drain = slog_term::FullFormat::new(plain_decorator).build().fuse();
            let async_file_drain = slog_async::Async::new(file_drain).build().fuse();

            let term_decorator = slog_term::TermDecorator::new().build();
            let console_drain = slog_term::FullFormat::new(term_decorator).build().fuse();
            let async_console_drain = slog_async::Async::new(console_drain).build().fuse();

            let drain = slog::Duplicate::new(async_console_drain, async_file_drain).fuse();
            let _log = slog::Logger::root(drain, o!("version" => DELILA_VERSION));
            Ok(_log)
        }).chain_err(|| "Unable to open log file")
}

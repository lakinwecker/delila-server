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


//--------------------------------------------------------------------------------------------------
// Requests for managing the database
//--------------------------------------------------------------------------------------------------

use diesel::prelude::*;

//use super::super::models::*;
//use super::super::schema::database::dsl::*;
//use super::super::establish_connection;

use std::path::Path;
use std::{io, fs};
use std::io::{Read, Write, BufWriter, BufReader};
use hyper::Client;

use super::{Request, Message};
use::errors::*;
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug)]
pub struct Finished {
    pub finished: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Progress {
    pub activity: String,
    pub progress: f32,
}

fn ensure_database_exists(request: Request) -> Result<()> {
    let mut state: Progress = Progress{activity: "Checking database".into(), progress: 0.0};
    request.send("initialize::updateProgress".into(), &state)?;
    let mut database_path = request.clone().path_settings.database_path;
    database_path.push("delila.db");
    info!(request.log, "Checking for database @ {:?}", database_path.to_str());
    if database_path.exists() {
        info!(request.log, "Database exists!");
        return Ok(())
    }

    info!(request.log, "Downloading database to {:?}", database_path.to_str());
    state.activity = "Download new database".into();
    request.send("initialize::updateProgress".into(), &state)?;

    let file = fs::File::create(&database_path)?;
    let mut writer = BufWriter::new(file);
    let client = Client::new();
    let response = client.get("http://delila.org/dist/0.1.0/db/delila.db").send()?;
    let mut reader = BufReader::new(response);
    let mut buf = [0; 128 * 1024];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(len) => writer.write_all(&buf[..len])?,
            Err(ref err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => return Err(err.into())
        };
    }
    Ok(())
}

pub fn initialize(request: Request, args:Version) -> Result<()> {
    let mut state: Progress = Progress{activity: "Loading ...".into(), progress: 0.0};
    request.send("initialize::updateProgress".into(), &state)?;
    ensure_database_exists(request.clone());
    let increment = 1f32;
    let tasks = vec![
        "Initializing datastores",
        "Updating datastore versions",
        "Reticulating splines",
        "Checking for updates",
        "Done",
    ];
    for activity in tasks {
        let _50ms = time::Duration::from_millis(50);
        thread::sleep(_50ms);
        state.progress += increment * 20.0f32;
        state.activity = activity.into();
        info!(request.log, "initialize::updateProgress {}", state.progress);
        request.send("initialize::updateProgress".into(), &state)?
    }
    request.send("initialize::finished".into(), &Finished{finished: true})?;

    Ok(()) 
}


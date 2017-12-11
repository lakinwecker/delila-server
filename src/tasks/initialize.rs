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

//use std::path::Path;
//use std::{io, fs};
//use std::io::{Read, Write, BufWriter, BufReader};
//use hyper::Client;

use super::Request;
use ::errors::*;
use std::{thread, time};
use app_info::DELILA_VERSION;

use diesel_migrations::run_pending_migrations;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeError {
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeFinished {
    pub finished: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionId(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionMismatchError {
    pub server_version: VersionId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub client_version: VersionId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeProgress {
    pub activity: String,
    pub progress: f32,
}

pub fn run_migrations(request: &Request) -> Result<()>
{
    run_pending_migrations(&request.get_connection())
        .chain_err(|| "Unable to run database migrations during startup")
}

pub fn initialize(request: &Request, version:Version) -> Result<()> {
    let Version {client_version: VersionId(client_version)} = version;
    if client_version != DELILA_VERSION {
        let mismatch =
        request.send(
            "VersionMismatchError".into(),
            &VersionMismatchError{server_version: VersionId(DELILA_VERSION.into())}
        );
    } else {
        let mut state: InitializeProgress = InitializeProgress{
            activity: "Running database migrations".into(),
            progress: 0.0
        };
        request.send("InitializeProgress".into(), &state)?;
        // TODO(lakin): This error should be sent to the client as well.
        run_migrations(&request)?;

        let increment = 1f32;
        let tasks = vec![
            "Reticulating splines",
            "Checking for updates",
            "Done",
        ];
        for activity in tasks {
            let _50ms = time::Duration::from_millis(50);
            thread::sleep(_50ms);
            state.progress += increment * 20.0f32;
            state.activity = activity.into();
            info!(request.log, "InitializeProgress {}", state.progress);
            request.send("InitializeProgress".into(), &state)?
        }
        request.send("InitializeFinished".into(), &InitializeFinished{finished: true})?;
    }

    Ok(())
}


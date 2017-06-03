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

pub fn initialize(request: Request, args:Version) -> Result<()> {
    let mut state: Progress = Progress{activity: "Loading ...".into(), progress: 0.0};
    request.send("initialize::updateProgress".into(), &state)?;
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


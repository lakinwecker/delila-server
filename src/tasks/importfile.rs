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
// A request handler for importing a PGN file.
//--------------------------------------------------------------------------------------------------

use diesel::prelude::*;

use super::super::models::*;
use super::super::schema::database::dsl::*;
use super::super::establish_connection;

use super::{Request, Message};
use::errors::*;
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub path: String
    //target_database: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Progress {
    pub activity: String,
    pub progress: f32,
}

pub fn handler(request: Request, args:File) -> Result<()> {
    let conn = establish_connection();
    let mut state: Progress = Progress{activity: "Loading ...".into(), progress: 0.0};
    request.send("updateProgress".into(), &state)?;
    let increment = 1f32;
    for i in 0..100 {
        let _0_5s = time::Duration::from_millis(50);
        thread::sleep(_0_5s);
        state.progress += increment;
        info!(request.log, "updateProgress {}", state.progress);
        request.send("updateProgress".into(), &state)?
    }

    Ok(()) 
}


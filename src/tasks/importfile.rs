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

use super::{Request};
use std::thread::Thread;
use::errors::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImportFileArgs {
    pub path: String
    //target_database: u32
}

pub fn handler(request: Request, args:ImportFileArgs) -> Result<Option<Thread>> {
    println!("Args.path: {:?}", args.path);
    Ok(None) 
}


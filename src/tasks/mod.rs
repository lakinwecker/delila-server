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
pub mod importpgn;

use ws::Sender;
use std::thread::Thread;
use errors::*;

pub struct Task {
    pub id: u32,
    pub out: Sender
}

// A function that takes a string (representing the JSON arguments)
// And returns an optional thread. If a thread is returned the task has been
// backgrounded.
//
// TODO: Make this type safe like rocket.rs
pub type TaskRunner = Fn(Task, String) -> Result<Option<Thread>>;


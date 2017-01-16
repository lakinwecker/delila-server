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
// Scid common datatypes
//------------------------------------------------------------------------------

// General types
pub type UByte = u8;
pub type UShort = u16;
pub type UInt = u32;
pub type SInt = i32;

// Comparison type
pub type Compare = i32;
const LESS_THAN: Compare = -1;
const EQUAL_TO: Compare = -1;
const GREATER_THAN: Compare = -1;

// Piece types
pub type Piece = i8;      // e.g ROOK or WHITE_KING
pub type Color = i8;      // WHITE or BLACK
pub type Square = i8;     // e.g. A3
pub type Direction = i8;  // e.g. UP_LEFT
pub type Rank = i8;       // Chess board rank
pub type File = i8;       // Chess board file
pub type LeftDiagonal = i8;   // Up-left diagonals
pub type RightDiagonal = i8;  // Up-right diagonals


// Game information types
pub type GameNumber = UInt;
pub type ELO = UShort;
pub type ECO = UShort;
// pub type typedef char            ecoStringT [6];   /* "A00j1" */
pub const ECO_None: ECO = 0;

// PIECE TYPES (without color; same value as a white piece)

pub const KING: Piece = 1;
pub const QUEEN: Piece = 2;
pub const ROOK: Piece = 3;
pub const BISHOP: Piece = 4;
pub const KNIGHT: Piece = 5;
pub const PAWN: Piece = 6;

// PIECES:
//   Note that color(x) == ((x & 0x8) >> 3)  and  type(x) == (x & 0x7)
//   EMPTY is deliberately nonzero, and END_OF_BOARD is zero, so that
//   a board can be used as a regular 0-terminated string, provided
//   that board[NULL_SQUARE] == END_OF_BOARD, as it always should be.

pub const EMPTY: Piece = 7;
pub const END_OF_BOARD: Piece = 0;
pub const WK: Piece =  1;
pub const WQ: Piece =  2;
pub const WR: Piece =  3;
pub const WB: Piece =  4;
pub const WN: Piece =  5;
pub const WP: Piece =  6;
pub const BK: Piece =  9;
pub const BQ: Piece = 10;
pub const BR: Piece = 11;
pub const BB: Piece = 12;
pub const BN: Piece = 13;
pub const BP: Piece = 14;

// Minor piece definitions, used for searching by material only:
pub const WM: Piece = 16;
pub const BM: Piece = 17;

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
// Material Signatures
//------------------------------------------------------------------------------

// Matsigs are 32-bit unsigned ints.  We only use 24 bits of this.

// From most significant bits down to least, the matsig layout is:
// Bits 22-33:  WQ    Bits 10-11:  BQ
// Bits 20-21:  WN    Bits 08-09:  BR
// Bits 18-19:  WB    Bits 06-07:  BB
// Bits 16-17:  WN    Bits 04-05:  BN
// Bits 12-15:  WP    Bits 00-03:  BP

// This means that pawn counts from 0 to 8 are possible, but for other
// pieces only counts up to 3 are possible.

pub type MaterialSignature = u32;

// Shifts:

pub const SHIFT_BP: u32 = 0;
pub const SHIFT_BN: u32 = 4;
pub const SHIFT_BB: u32 = 6;
pub const SHIFT_BR: u32 = 8;
pub const SHIFT_BQ: u32 = 10;
pub const SHIFT_WP: u32 = 12;
pub const SHIFT_WN: u32 = 16;
pub const SHIFT_WB: u32 = 18;
pub const SHIFT_WR: u32 = 20;
pub const SHIFT_WQ: u32 = 22;

// Masks:
                            //         28   24   20   16   12    8    4    0
pub const MASK_BP: u32 = 0x0000000F; // 0- 3: .... .... .... .... .... .... .... 1111
pub const MASK_BN: u32 = 0x00000030; // 4- 5: .... .... .... .... .... .... ..11 ....
pub const MASK_BB: u32 = 0x000000C0; // 6- 7: .... .... .... .... .... .... 11.. ....
pub const MASK_BR: u32 = 0x00000300; // 8- 9: .... .... .... .... .... ..11 .... ....
pub const MASK_BQ: u32 = 0x00000C00; //10-11: .... .... .... .... .... 11.. .... ....
pub const MASK_WP: u32 = 0x0000F000; //12-15: .... .... .... .... 1111 .... .... ....
pub const MASK_WN: u32 = 0x00030000; //16-17: .... .... .... ..11 .... .... .... ....
pub const MASK_WB: u32 = 0x000C0000; //18-19: .... .... .... 11.. .... .... .... ....
pub const MASK_WR: u32 = 0x00300000; //20-21: .... .... ..11 .... .... .... .... ....
pub const MASK_WQ: u32 = 0x00C00000; //29-31: .... .... 11.. .... .... .... .... ....

pub static MASK_BY_PIECE: &'static [MaterialSignature; 16] = &[
    0,        //  0: Empty
    0,        //  1: WK
    MASK_WQ,  //  2: WQ
    MASK_WR,  //  3: WR
    MASK_WB,  //  4: WB
    MASK_WN,  //  5: WN
    MASK_WP,  //  6: WP
    0, 0,     //  7, 8: Invalid pieces
    0,        //  9: BK
    MASK_BQ,  // 10: BQ
    MASK_BR,  // 11: BR
    MASK_BB,  // 12: BB
    MASK_BN,  // 13: BN
    MASK_BP,  // 14: BP
    0         // 15: Invalid piece
];

pub static SHIFT_BY_PIECE: &'static [u32; 16] = &[
    0, 0,      //  0: Empty,  1: WK
    SHIFT_WQ,  //  2: WQ
    SHIFT_WR,  //  3: WR
    SHIFT_WB,  //  4: WB
    SHIFT_WN,  //  5: WN
    SHIFT_WP,  //  6: WP
    0, 0, 0,   //  7, 8: Invalid pieces,  9: BK
    SHIFT_BQ,  // 10: BQ
    SHIFT_BR,  // 11: BR
    SHIFT_BB,  // 12: BB
    SHIFT_BN,  // 13: BN
    SHIFT_BP,  // 14: BP
    0          // 15: Invalid piece
];

// Quick way to flip colors: just switch the upper/lower 12 bits
pub fn flip_color(x: MaterialSignature) -> MaterialSignature {
    ((x) >> 12u32) | (((x) & 0x00000FFF) << 12)
}

// Quick tests for non-zero counts of a piece type:

pub fn has_wq(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_WQ) }
pub fn has_bq(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_BQ) }
pub fn has_wr(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_WR) }
pub fn has_br(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_BR) }
pub fn has_wb(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_WB) }
pub fn has_bb(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_BB) }
pub fn has_wn(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_WN) }
pub fn has_bn(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_BN) }
pub fn has_wp(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_WP) }
pub fn has_bp(x: MaterialSignature) -> MaterialSignature { ((x) & MASK_BP) }

pub fn has_queens(x: MaterialSignature) -> MaterialSignature  { ((x) & (MASK_WQ | MASK_BQ)) }
pub fn has_rooks(x: MaterialSignature) -> MaterialSignature   { ((x) & (MASK_WR | MASK_BR)) }
pub fn has_bishops(x: MaterialSignature) -> MaterialSignature { ((x) & (MASK_WB | MASK_BB)) }
pub fn has_knights(x: MaterialSignature) -> MaterialSignature { ((x) & (MASK_WN | MASK_BN)) }
pub fn has_pawns(x: MaterialSignature) -> MaterialSignature   { ((x) & (MASK_WP | MASK_BP)) }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flipcolor() {
		let x = 0b0000_0000_1111_0000_1111_0000_1111_0000;
		let y = 0b0000_0000_0000_1111_0000_1111_0000_1111;
        assert_eq!(y, flip_color(x));

		let x = 0b0000_0000_1111_0000_1111_0100_1011_0100;
		let y = 0b0000_0000_0100_1011_0100_1111_0000_1111;
        assert_eq!(y, flip_color(x));
    }

    #[test]
    fn test_has_wq() {
		let none = 0b0000_0000_0011_1111_1111_1111_1111_1111;
        assert!(has_wq(none) == 0);
		let some = 0b0000_0000_1100_0000_0000_0000_0000_0000;
        assert!(has_wq(some) > 0);
    }

    #[test]
    fn test_has_bq() {
		let none = 0b0000_0000_1111_1111_1111_0011_1111_1111;
        assert!(has_bq(none) == 0);
		let some = 0b0000_0000_0000_0000_0000_1100_0000_0000;
        assert!(has_bq(some) > 0);
    }

    #[test]
    fn test_has_wr() {
		let none = 0b0000_0000_1100_1111_1111_1111_1111_1111;
        assert!(has_wr(none) == 0);
		let some = 0b0000_0000_0011_0000_0000_0000_0000_0000;
        assert!(has_wr(some) > 0);
    }

    #[test]
    fn test_has_br() {
		let none = 0b0000_0000_1111_1111_1111_1100_1111_1111;
        assert!(has_br(none) == 0);
		let some = 0b0000_0000_0000_0000_0000_0011_0000_0000;
        assert!(has_br(some) > 0);
    }

    #[test]
    fn test_has_wb() {
		let none = 0b0000_0000_1111_0011_1111_1111_1111_1111;
        assert!(has_wb(none) == 0);
		let some = 0b0000_0000_0000_1100_0000_0000_0000_0000;
        assert!(has_wb(some) > 0);
    }

    #[test]
    fn test_has_bb() {
		let none = 0b0000_0000_1111_1111_1111_1111_0011_1111;
        assert!(has_bb(none) == 0);
		let some = 0b0000_0000_0000_0000_0000_0000_1100_0000;
        assert!(has_bb(some) > 0);
    }

    #[test]
    fn test_has_wn() {
		let none = 0b0000_0000_1111_1100_1111_1111_1111_1111;
        assert!(has_wn(none) == 0);
		let some = 0b0000_0000_0000_0011_0000_0000_0000_0000;
        assert!(has_wn(some) > 0);
    }

    #[test]
    fn test_has_bn() {
		let none = 0b0000_0000_1111_1111_1111_1111_1100_1111;
        assert!(has_bn(none) == 0);
		let some = 0b0000_0000_0000_0000_0000_0000_0011_0000;
        assert!(has_bn(some) > 0);
    }

    #[test]
    fn test_has_wp() {
		let none = 0b0000_0000_1111_1111_0000_1111_1111_1111;
        assert!(has_wp(none) == 0);
		let some = 0b0000_0000_0000_0000_1111_0000_0000_0000;
        assert!(has_wp(some) > 0);
    }

    #[test]
    fn test_has_bp() {
		let none = 0b0000_0000_1111_1111_1111_1111_1111_0000;
        assert!(has_bp(none) == 0);
		let some = 0b0000_0000_0000_0000_0000_0000_0000_1111;
        assert!(has_bp(some) > 0);
    }
}


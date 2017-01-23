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
// Bits 22-33:  WQ   Bits 10-11:  BQ
// Bits 20-21:  WR    Bits 08-09:  BR
// Bits 18-19:  WB  Bits 06-07:  BB
// Bits 16-17:  WN  Bits 04-05:  BN
// Bits 12-15:  WP     Bits 00-03:  BP

// This means that pawn counts from 0 to 8 are possible, but for other
// pieces only counts up to 3 are possible.

use super::common::*;

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
pub fn has_pawn(x: MaterialSignature) -> MaterialSignature   { ((x) & (MASK_WP | MASK_BP)) }

// Macros to extract a particular count:

pub fn count_wq(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_WQ) >> SHIFT_WQ) }
pub fn count_bq(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_BQ) >> SHIFT_BQ) }
pub fn count_wr(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_WR) >> SHIFT_WR) }
pub fn count_br(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_BR) >> SHIFT_BR) }
pub fn count_wb(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_WB) >> SHIFT_WB) }
pub fn count_bb(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_BB) >> SHIFT_BB) }
pub fn count_wn(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_WN) >> SHIFT_WN) }
pub fn count_bn(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_BN) >> SHIFT_BN) }
pub fn count_wp(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_WP) >> SHIFT_WP) }
pub fn count_bp(x: MaterialSignature) -> MaterialSignature { (((x) & MASK_BP) >> SHIFT_BP) }


//------------------------------------------------------------------------------
// get_count():
//      Inline routine to extract a count of a certain piece type.
//
pub fn get_count (m: MaterialSignature, p: Piece) -> MaterialSignature {
    return (m & MASK_BY_PIECE[p as usize]) >> SHIFT_BY_PIECE[p as usize];
}

//------------------------------------------------------------------------------
// set_count():
//      Inline routine to set a particular count.
//
pub fn set_count (m: MaterialSignature, p: Piece, count: UInt) -> MaterialSignature
{
    // First we clear the old mask for this piece:
    let mut m = m & !(MASK_BY_PIECE[p as usize]);
    let mut count = count;
    // Avoid overflow.
    if (p != WP) && (p != BP) && count > 3 {
        count = 3;
    }
    // Now we OR to add the new value in:
    m = m | (count as UInt) << SHIFT_BY_PIECE[p as usize];
    m
}

// Common constant matsigs:

const EMPTY: MaterialSignature = 0;
const STD_START: MaterialSignature =
   ((1 << SHIFT_WQ) | (1 << SHIFT_BQ) |
    (2 << SHIFT_WR) | (2 << SHIFT_BR) |
    (2 << SHIFT_WB) | (2 << SHIFT_BB) |
    (2 << SHIFT_WN) | (2 << SHIFT_BN) |
    (8 << SHIFT_WP) | (8 << SHIFT_BP));


// matsig_makeString: sets s to be a string representation of the sig,

pub fn make_string (m: MaterialSignature) -> String {
    let mut s: String = String::with_capacity(32);
    for _ in 0..count_wq(m) { s.push('Q'); }
    for _ in 0..count_wr(m) { s.push('R'); }
    for _ in 0..count_wb(m) { s.push('B'); }
    for _ in 0..count_wn(m) { s.push('N'); }
    let wp = count_wp(m);
    if wp > 0 {
        for c in wp.to_string().chars() {
            s.push(c);
        }
        s.push('0');
    }
    s.push(':');
    
    for _ in 0..count_bq(m) { s.push('Q'); }
    for _ in 0..count_br(m) { s.push('R'); }
    for _ in 0..count_bb(m) { s.push('B'); }
    for _ in 0..count_bn(m) { s.push('N'); }
    let bp = count_bp(m);
    if bp > 0 {
        for c in bp.to_string().chars() {
            s.push(c);
        }
        s.push('0');
    }
    s
}

//------------------------------------------------------------------------------
// is_reachable: returns true if a game currently
//     at a position with the signature <start>, could possibly reach
//     the signature <target>. This is useful for quick tests for material
//     searches. For example, if we store the final matsig of every game,
//     we can speedup a material search by ONLY searching the games that
//     have matsig_isReachable(searchsig, finalsig) = 1, or have promotions.
//     Example: if searchsig requires neither side to have queens, but
//     finalsig for a game shows a WQ (and no promotions), the game could
//     not possibly match.
//     If promos is true, only the pawn counts are checked, since other
//     material could reappear on the board due to a promotion.
//     If upromo is true, there are underpromotions (to R, B or N) but
//     if only promos is true, all promotions are to Queens only.
pub fn is_reachable (mStart: MaterialSignature, mTarget: MaterialSignature, promos: bool, upromo: bool) -> bool {
    if count_wp(mStart) < count_wp(mTarget)  { return false; }
    if count_bp(mStart) < count_bp(mTarget)  { return false; }

    // If there are underpromotions, we can only check pawn counts:
    if upromo { return true; }

    // No underpromotions, so check non-queen piece counts:
    if count_wr(mStart) < count_wr(mTarget)  { return false; }
    if count_br(mStart) < count_br(mTarget)  { return false; }
    if count_wb(mStart) < count_wb(mTarget)  { return false; }
    if count_bb(mStart) < count_bb(mTarget)  { return false; }
    if count_wn(mStart) < count_wn(mTarget)  { return false; }
    if count_bn(mStart) < count_bn(mTarget)  { return false; }

    // If there were promotions we cannot check queen counts:
    if promos { return true; }

    // Check queen counts:
    if count_wq(mStart) < count_wq(mTarget)  { return false; }
    if count_bq(mStart) < count_bq(mTarget)  { return false; }

    return true;
}


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
    fn test_has_wq_count_wq() {
        let none = 0b0000_0000_0011_1111_1111_1111_1111_1111;
        assert!(has_wq(none) == 0);
        assert_eq!(count_wq(none), 0);
        let some = 0b0000_0000_1100_0000_0000_0000_0000_0000;
        assert!(has_wq(some) > 0);
        assert_eq!(count_wq(some), 3);
    }

    #[test]
    fn test_has_bq_count_bq() {
        let none = 0b0000_0000_1111_1111_1111_0011_1111_1111;
        assert!(has_bq(none) == 0);
        assert_eq!(count_bq(none), 0);
        let some = 0b0000_0000_0000_0000_0000_1100_0000_0000;
        assert!(has_bq(some) > 0);
        assert_eq!(count_bq(some), 3);
    }

    #[test]
    fn test_has_wr_count_wr() {
        let none = 0b0000_0000_1100_1111_1111_1111_1111_1111;
        assert!(has_wr(none) == 0);
        assert_eq!(count_wr(none), 0);
        let some = 0b0000_0000_0011_0000_0000_0000_0000_0000;
        assert!(has_wr(some) > 0);
        assert_eq!(count_wr(some), 3);
    }

    #[test]
    fn test_has_br_count_br() {
        let none = 0b0000_0000_1111_1111_1111_1100_1111_1111;
        assert!(has_br(none) == 0);
        assert_eq!(count_br(none), 0);
        let some = 0b0000_0000_0000_0000_0000_0011_0000_0000;
        assert!(has_br(some) > 0);
        assert_eq!(count_br(some), 3);
    }

    #[test]
    fn test_has_wb_count_wb() {
        let none = 0b0000_0000_1111_0011_1111_1111_1111_1111;
        assert!(has_wb(none) == 0);
        assert_eq!(count_wb(none), 0);
        let some = 0b0000_0000_0000_1100_0000_0000_0000_0000;
        assert!(has_wb(some) > 0);
        assert_eq!(count_wb(some), 3);
    }

    #[test]
    fn test_has_bb_count_bb() {
        let none = 0b0000_0000_1111_1111_1111_1111_0011_1111;
        assert!(has_bb(none) == 0);
        assert_eq!(count_bb(none), 0);
        let some = 0b0000_0000_0000_0000_0000_0000_1100_0000;
        assert!(has_bb(some) > 0);
        assert_eq!(count_bb(some), 3);
    }

    #[test]
    fn test_has_wn_count_wn() {
        let none = 0b0000_0000_1111_1100_1111_1111_1111_1111;
        assert!(has_wn(none) == 0);
        assert_eq!(count_wn(none), 0);
        let some = 0b0000_0000_0000_0011_0000_0000_0000_0000;
        assert!(has_wn(some) > 0);
        assert_eq!(count_wn(some), 3);
    }

    #[test]
    fn test_has_bn_count_wn() {
        let none = 0b0000_0000_1111_1111_1111_1111_1100_1111;
        assert!(has_bn(none) == 0);
        assert_eq!(count_bn(none), 0);
        let some = 0b0000_0000_0000_0000_0000_0000_0011_0000;
        assert!(has_bn(some) > 0);
        assert_eq!(count_bn(some), 3);
    }

    #[test]
    fn test_has_wp_count_wp() {
        let none = 0b0000_0000_1111_1111_0000_1111_1111_1111;
        assert!(has_wp(none) == 0);
        assert_eq!(count_wp(none), 0);
        let some = 0b0000_0000_0000_0000_1111_0000_0000_0000;
        assert!(has_wp(some) > 0);
        assert_eq!(count_wp(some), 15);
    }

    #[test]
    fn test_has_bp_count_bp() {
        let none = 0b0000_0000_1111_1111_1111_1111_1111_0000;
        assert!(has_bp(none) == 0);
        assert_eq!(count_bp(none), 0);
        let some = 0b0000_0000_0000_0000_0000_0000_0000_1111;
        assert!(has_bp(some) > 0);
        assert_eq!(count_bp(some), 15);
    }

    #[test]
    fn test_get_count() {
        let none = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        assert!(get_count(none, WP) == 0);
        assert!(get_count(none, WN) == 0);
        assert!(get_count(none, WB) == 0);
        assert!(get_count(none, WR) == 0);
        assert!(get_count(none, WQ) == 0);
        assert!(get_count(none, BP) == 0);
        assert!(get_count(none, BN) == 0);
        assert!(get_count(none, BB) == 0);
        assert!(get_count(none, BR) == 0);
        assert!(get_count(none, BQ) == 0);

        let some = 0b0000_0000_0000_0000_1111_0000_0000_0000;
        assert_eq!(get_count(some, WP), 15);
        let some = 0b0000_0000_0000_0011_0000_0000_0000_0000;
        assert_eq!(get_count(some, WN), 3);
        let some = 0b0000_0000_0000_1100_0000_0000_0000_0000;
        assert_eq!(get_count(some, WB), 3);
        let some = 0b0000_0000_0011_0000_0000_0000_0000_0000;
        assert_eq!(get_count(some, WR), 3);
        let some = 0b0000_0000_1100_0000_0000_0000_0000_0000;
        assert_eq!(get_count(some, WQ), 3);

        let some = 0b0000_0000_0000_0000_0000_0000_0000_1111;
        assert_eq!(get_count(some, BP), 15);
        let some = 0b0000_0000_0000_0000_0000_0000_0011_0000;
        assert_eq!(get_count(some, BN), 3);
        let some = 0b0000_0000_0000_0000_0000_0000_1100_0000;
        assert_eq!(get_count(some, BB), 3);
        let some = 0b0000_0000_0000_0000_0000_0011_0000_0000;
        assert_eq!(get_count(some, BR), 3);
        let some = 0b0000_0000_0000_0000_0000_1100_0000_0000;
        assert_eq!(get_count(some, BQ), 3);
    }

    #[test]
    fn test_set_count() {
        for p in [WP, BP].into_iter() {
            let none = 0b0000_0000_0000_0000_0000_0000_0000_0000;
            let none = set_count(none, *p, 1);
            assert_eq!(get_count(none, *p), 1);
            let none = set_count(none, *p, 8);
            assert_eq!(get_count(none, *p), 8);
        }

        for p in [WN, WB, WR, WQ,
                  BN, BB, BR, BQ].into_iter() {
            let none = 0b0000_0000_0000_0000_0000_0000_0000_0000;
            let none = set_count(none, *p, 1);
            assert_eq!(get_count(none, *p), 1);
            let none = set_count(none, *p, 3);
            assert_eq!(get_count(none, *p), 3);
            let none = set_count(none, *p, 15);
            assert_eq!(get_count(none, *p), 3);
        }
    }

    #[test]
    fn test_make_string() {
        let none = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        assert_eq!(":", make_string(none));
        let none = 0b0000_0000_1111_1111_1111_1111_1111_1111;
        assert_eq!("QQQRRRBBBNNN150:QQQRRRBBBNNN150", make_string(none));
        let none = 0b0000_0000_0101_0101_0101_0101_0101_0101;
        assert_eq!("QRBN50:QRBN50", make_string(none));
    }
}


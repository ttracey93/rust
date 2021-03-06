// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that when we compile the static `XXX` into MIR, we do not
// generate `StorageStart` or `StorageEnd` statements.

// ignore-tidy-linelength

static XXX: &'static Foo = &Foo {
    tup: "hi",
    data: &[
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
        (0, 1), (0, 2), (0, 3),
    ]
};

#[derive(Debug)]
struct Foo {
    tup: &'static str,
    data: &'static [(u32, u32)]
}

fn main() {
    println!("{:?}", XXX);
}

// END RUST SOURCE
// START rustc.XXX.mir_map.0.mir
//    let mut _0: &'static Foo;
//    let mut _1: &'static Foo;
//    let _2: Foo;
//    let mut _3: &'static [(u32, u32)];
//    let mut _4: &'static [(u32, u32); 42];
//    let mut _5: &'static [(u32, u32); 42];
//    let _6: [(u32, u32); 42];
//    let mut _7: (u32, u32);
//    let mut _8: (u32, u32);
//    let mut _9: (u32, u32);
//    let mut _10: (u32, u32);
//    let mut _11: (u32, u32);
//    let mut _12: (u32, u32);
//    let mut _13: (u32, u32);
//    let mut _14: (u32, u32);
//    let mut _15: (u32, u32);
//    let mut _16: (u32, u32);
//    let mut _17: (u32, u32);
//    let mut _18: (u32, u32);
//    let mut _19: (u32, u32);
//    let mut _20: (u32, u32);
//    let mut _21: (u32, u32);
//    let mut _22: (u32, u32);
//    let mut _23: (u32, u32);
//    let mut _24: (u32, u32);
//    let mut _25: (u32, u32);
//    let mut _26: (u32, u32);
//    let mut _27: (u32, u32);
//    let mut _28: (u32, u32);
//    let mut _29: (u32, u32);
//    let mut _30: (u32, u32);
//    let mut _31: (u32, u32);
//    let mut _32: (u32, u32);
//    let mut _33: (u32, u32);
//    let mut _34: (u32, u32);
//    let mut _35: (u32, u32);
//    let mut _36: (u32, u32);
//    let mut _37: (u32, u32);
//    let mut _38: (u32, u32);
//    let mut _39: (u32, u32);
//    let mut _40: (u32, u32);
//    let mut _41: (u32, u32);
//    let mut _42: (u32, u32);
//    let mut _43: (u32, u32);
//    let mut _44: (u32, u32);
//    let mut _45: (u32, u32);
//    let mut _46: (u32, u32);
//    let mut _47: (u32, u32);
//    let mut _48: (u32, u32);
//    bb0: {
//        StorageLive(_1);
//        StorageLive(_2);
//        StorageLive(_3);
//        StorageLive(_4);
//        StorageLive(_5);
//        StorageLive(_6);
//        StorageLive(_7);
//        _7 = (const 0u32, const 1u32);
//        StorageLive(_8);
//        _8 = (const 0u32, const 2u32);
//        StorageLive(_9);
//        _9 = (const 0u32, const 3u32);
//        StorageLive(_10);
//        _10 = (const 0u32, const 1u32);
//        StorageLive(_11);
//        _11 = (const 0u32, const 2u32);
//        StorageLive(_12);
//        _12 = (const 0u32, const 3u32);
//        StorageLive(_13);
//        _13 = (const 0u32, const 1u32);
//        StorageLive(_14);
//        _14 = (const 0u32, const 2u32);
//        StorageLive(_15);
//        _15 = (const 0u32, const 3u32);
//        StorageLive(_16);
//        _16 = (const 0u32, const 1u32);
//        StorageLive(_17);
//        _17 = (const 0u32, const 2u32);
//        StorageLive(_18);
//        _18 = (const 0u32, const 3u32);
//        StorageLive(_19);
//        _19 = (const 0u32, const 1u32);
//        StorageLive(_20);
//        _20 = (const 0u32, const 2u32);
//        StorageLive(_21);
//        _21 = (const 0u32, const 3u32);
//        StorageLive(_22);
//        _22 = (const 0u32, const 1u32);
//        StorageLive(_23);
//        _23 = (const 0u32, const 2u32);
//        StorageLive(_24);
//        _24 = (const 0u32, const 3u32);
//        StorageLive(_25);
//        _25 = (const 0u32, const 1u32);
//        StorageLive(_26);
//        _26 = (const 0u32, const 2u32);
//        StorageLive(_27);
//        _27 = (const 0u32, const 3u32);
//        StorageLive(_28);
//        _28 = (const 0u32, const 1u32);
//        StorageLive(_29);
//        _29 = (const 0u32, const 2u32);
//        StorageLive(_30);
//        _30 = (const 0u32, const 3u32);
//        StorageLive(_31);
//        _31 = (const 0u32, const 1u32);
//        StorageLive(_32);
//        _32 = (const 0u32, const 2u32);
//        StorageLive(_33);
//        _33 = (const 0u32, const 3u32);
//        StorageLive(_34);
//        _34 = (const 0u32, const 1u32);
//        StorageLive(_35);
//        _35 = (const 0u32, const 2u32);
//        StorageLive(_36);
//        _36 = (const 0u32, const 3u32);
//        StorageLive(_37);
//        _37 = (const 0u32, const 1u32);
//        StorageLive(_38);
//        _38 = (const 0u32, const 2u32);
//        StorageLive(_39);
//        _39 = (const 0u32, const 3u32);
//        StorageLive(_40);
//        _40 = (const 0u32, const 1u32);
//        StorageLive(_41);
//        _41 = (const 0u32, const 2u32);
//        StorageLive(_42);
//        _42 = (const 0u32, const 3u32);
//        StorageLive(_43);
//        _43 = (const 0u32, const 1u32);
//        StorageLive(_44);
//        _44 = (const 0u32, const 2u32);
//        StorageLive(_45);
//        _45 = (const 0u32, const 3u32);
//        StorageLive(_46);
//        _46 = (const 0u32, const 1u32);
//        StorageLive(_47);
//        _47 = (const 0u32, const 2u32);
//        StorageLive(_48);
//        _48 = (const 0u32, const 3u32);
//        _6 = [move _7, move _8, move _9, move _10, move _11, move _12, move _13, move _14, move _15, move _16, move _17, move _18, move _19, move _20, move _21, move _22, move _23, move _24, move _25, move _26, move _27, move _28, move _29, move _30, move _31, move _32, move _33, move _34, move _35, move _36, move _37, move _38, move _39, move _40, move _41, move _42, move _43, move _44, move _45, move _46, move _47, move _48];
//        _5 = &_6;
//        _4 = &(*_5);
//        _3 = move _4 as &'static [(u32, u32)] (Unsize);
//        _2 = Foo { tup: const "hi", data: move _3 };
//        _1 = &_2;
//        _0 = &(*_1);
//        StorageDead(_1);
//        StorageDead(_5);
//        return;
//    }
//}
// END rustc.XXX.mir_map.0.mir

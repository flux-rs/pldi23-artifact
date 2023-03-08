/// Tests that parser handles spans correctly.

use prusti_contracts::*;

#[requires(  12345 ==> true)]
pub fn test_1(x: i32) {}

#[requires(  true      ==>  12345 )]
pub fn test_2(x: i32) {}

#[requires(  true  ==>  (true && 12345 && true))]
pub fn test_3(x: i32) {}

#[requires(  true  ==>  (true && 12345 ==> true))]
pub fn test_4(x: i32) {}

#[requires(  true  ==>  (true && 12345 ==> true && 12345))]
pub fn test_5(x: i32) {}

fn main() {}


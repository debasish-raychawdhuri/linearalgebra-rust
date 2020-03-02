/*
 * Copyright  2020 Debasish Ray Chawdhuri
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */


use crate::Ring;
#[derive(Clone, PartialEq, Debug)]
pub struct I32Ring;

#[derive(Clone, PartialEq, Debug)]
pub struct I64Ring;

impl Ring for I32Ring{
    type RingMember = i32;

    fn zero() -> <Self as Ring>::RingMember { 0i32 }
    fn neg(v: &i32) -> i32 { 0-v }
    fn mul(x: &i32, y: &i32) -> i32 { x*y }
    fn add(x: &i32, y: &i32) -> i32 { x+y }
    fn one() -> i32 { 1i32 }
}

impl Ring for I64Ring{
    type RingMember = i64;

    fn zero() ->i64 { 0i64 }
    fn neg(v: &i64) -> i64 { 0-v }
    fn mul(x: &i64, y: &i64) -> i64 { x*y }
    fn add(x: &i64, y: &i64) -> i64 { x+y }
    fn one() -> i64 { 1i64 }
}

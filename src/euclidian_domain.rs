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
use super::error::Error;
use super::error::ErrorKind;
use super::Ring;
pub struct DivisionAlgorithmResult<R> {
    pub quotient: R,
    pub remainder: R,
}

pub struct ExtendedEuclidResult<R> {
    pub x: R,
    pub y: R,
    pub gcd: R,
}

pub trait EuclidianDomain: Ring {
    fn division_algorithm(
        &self,
        value: &Self::RingMember,
        divisor: &Self::RingMember,
    ) -> Result<DivisionAlgorithmResult<Self::RingMember>, Error>;

    fn extended_euclid(
        &self,
        a: &Self::RingMember,
        b: &Self::RingMember,
    ) -> Result<ExtendedEuclidResult<Self::RingMember>, Error> {
        let mut cur = (self.one(), self.zero());
        let mut prev = (self.zero(), self.one());
        let mut cur_divisor = self.add(a, &self.zero());
        let mut cur_dividend = self.add(b, &self.zero());
        loop {
            let div_result = self.division_algorithm(&cur_dividend, &cur_divisor);
            match div_result {
                Ok(div_result) => {
                    if div_result.remainder == self.zero() {
                        return Ok(ExtendedEuclidResult {
                            x: cur.0,
                            y: cur.1,
                            gcd: cur_divisor,
                        });
                    }
                    cur_dividend = cur_divisor;
                    cur_divisor = div_result.remainder;
                    let temp = prev;
                    prev = cur;
                    cur = (
                        self.add(&temp.0, &self.mul(&prev.0, &self.neg(&div_result.quotient))),
                        self.add(&temp.1, &self.mul(&prev.1, &self.neg(&div_result.quotient))),
                    );
                }
                Err(e) => {
                    return Err(Error::new(ErrorKind::DivisionByZero));
                }
            }
        }
    }
}

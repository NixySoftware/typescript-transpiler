use ts_std::*;
#[allow(clippy::all)]
fn main() {
    console.assert(1 + 2 == 3);
    console.assert(4 - 1 == 3);
    console.assert(1 * 3 == 3);
    console.assert(9 / 3 == 3);
    console.assert(4 >> 1 == 2);
    console.assert(2 << 4 == 32);
    console.assert((14 & 3) == 2);
    console.assert((1 | 4) == 5);
    console.assert((2 ^ 2) == 0);
}

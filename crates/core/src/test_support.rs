//! What the tests of several modules share.

/// A small generator of numbers, the same at every run, without a dependency.
pub struct Random(pub u64);

impl Random {
    /// The next number.
    pub fn next(&mut self) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        usize::try_from(self.0 >> 33).unwrap()
    }

    /// One of `pieces`, which must not be empty.
    pub fn pick<'a>(&mut self, pieces: &[&'a str]) -> &'a str {
        pieces[self.next() % pieces.len()]
    }
}

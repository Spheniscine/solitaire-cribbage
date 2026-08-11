#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitMask(pub u16);
impl BitMask {
    pub fn single(x: usize) -> Self {
        Self(1 << x)
    }

    pub fn contains(self, x: usize) -> bool {
        self.0 >> x & 1 == 1
    }

    pub fn flip(self, x: usize) -> Self {
        Self(self.0 ^ (1 << x))
    }
}
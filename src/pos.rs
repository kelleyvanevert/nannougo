#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn neighbors(&self, size: usize) -> Vec<Pos> {
        vec![
            Pos(self.0 - 1, self.1),
            Pos(self.0 + 1, self.1),
            Pos(self.0, self.1 - 1),
            Pos(self.0, self.1 + 1),
        ]
        .iter()
        .filter(|&p| p.0 >= 0 && p.0 < (size as i32) && p.1 >= 0 && p.1 < (size as i32))
        .map(|p| *p)
        .collect()
    }
}

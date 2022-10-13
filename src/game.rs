use crate::pos::Pos;

type Board = Vec<Option<bool>>;

pub struct Game {
    pub size: usize,
    pub turn: bool,
    pub board: Board,
    history: Vec<Board>,
}

impl Game {
    pub fn new(size: usize) -> Game {
        Self {
            size,
            turn: false,
            board: vec![None; size * size],
            history: vec![],
        }
    }

    /// Checks whether the structure around position `p` is surrounded, and if so, returns the the whole structure.
    pub fn is_surrounded(&self, p: Pos) -> Option<Vec<Pos>> {
        let mut found: Vec<Pos> = vec![];

        if let Some(color) = self.stone_at(p) {
            let mut todo: Vec<Pos> = vec![p];

            while let Some(p) = todo.pop() {
                found.push(p);

                for np in p.neighbors(self.size) {
                    if !self.has_stone_at(np) {
                        return None;
                    } else if let Some(neighbor_color) = self.stone_at(np) {
                        if neighbor_color == color && !found.contains(&np) {
                            todo.push(np);
                        }
                    }
                }
            }
        }

        Some(found)
    }

    fn remove_if_surrounded(&mut self, p: Pos) {
        if let Some(structure) = self.is_surrounded(p) {
            for p in structure {
                self.board[(p.1 as usize) * self.size + (p.0 as usize)] = None;
            }
        }
    }

    pub fn try_place_stone(&mut self, p: Pos) {
        if self.has_stone_at(p) {
            return;
        }

        self.history.push(self.board.clone());

        self.board[(p.1 as usize) * self.size + (p.0 as usize)] = Some(self.turn);
        for np in p.neighbors(self.size) {
            if self.stone_at(np) == Some(!self.turn) {
                self.remove_if_surrounded(np);
            }
        }
        self.remove_if_surrounded(p);

        // ko rule
        let len = self.history.len();
        if len >= 2 && self.history.get(len - 2) == Some(&self.board) {
            self.board = self.history.pop().unwrap();
            return;
        }

        self.turn = !self.turn;
    }

    fn stone_at(&self, p: Pos) -> Option<bool> {
        self.board[(p.1 as usize) * self.size + (p.0 as usize)]
    }

    pub fn has_stone_at(&self, p: Pos) -> bool {
        None != self.stone_at(p)
    }
}

use enum_map::{Enum, EnumMap};
use std::ops::Neg;

use crate::pos::Pos;

#[derive(Clone, PartialEq, Copy, Enum)]
pub enum Stone {
    White,
    Black,
}

impl Neg for Stone {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

type Board = Vec<Option<Stone>>;

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub board: Board,
    pub captures: EnumMap<Stone, usize>,
}

impl GameState {
    pub fn new(size: usize) -> Self {
        Self {
            board: vec![None; size * size],
            captures: enum_map! {
                Stone::White => 0,
                Stone::Black => 0,
            },
        }
    }
}

pub struct Game {
    pub size: usize,
    pub turn: Stone,
    pub state: GameState,
    history: Vec<GameState>,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            turn: Stone::White,
            state: GameState::new(size),
            history: vec![],
        }
    }

    /// Checks whether the structure around position `p` is surrounded, and if so, returns the the whole structure.
    pub fn is_surrounded(&self, p: Pos) -> Option<(Stone, Vec<Pos>)> {
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

            return Some((color, found));
        }

        // should not happen
        None
    }

    fn remove_if_surrounded(&mut self, p: Pos) {
        if let Some((color, structure)) = self.is_surrounded(p) {
            let num_captures = structure.len();
            for p in structure {
                self.state.board[(p.1 as usize) * self.size + (p.0 as usize)] = None;
            }
            self.state.captures[color] += num_captures;
        }
    }

    pub fn try_place_stone(&mut self, p: Pos) {
        if self.has_stone_at(p) {
            return;
        }

        self.history.push(self.state.clone());

        self.state.board[(p.1 as usize) * self.size + (p.0 as usize)] = Some(self.turn);
        for np in p.neighbors(self.size) {
            if self.stone_at(np) == Some(-self.turn) {
                self.remove_if_surrounded(np);
            }
        }
        self.remove_if_surrounded(p);

        // ko rule
        let len = self.history.len();
        if len >= 2
            && self.history.get(len - 2).map(|s| s.board.clone()) == Some(self.state.board.clone())
        {
            self.state = self.history.pop().unwrap();
            return;
        }

        self.turn = -self.turn;
    }

    pub fn stone_at(&self, p: Pos) -> Option<Stone> {
        self.state.board[(p.1 as usize) * self.size + (p.0 as usize)]
    }

    pub fn has_stone_at(&self, p: Pos) -> bool {
        None != self.stone_at(p)
    }
}

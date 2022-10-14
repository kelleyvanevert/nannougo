#[macro_use]
extern crate enum_map;

use crate::game::Game;
use crate::pos::Pos;
use game::Stone;
use nannou::{color, event::ElementState, prelude::*, winit::event::DeviceEvent};

mod game;
mod pos;

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        // .fullscreen()
        .run();
}

trait MakeSquare {
    fn make_inset_square(&self) -> Self;
}

impl MakeSquare for Rect {
    fn make_inset_square(&self) -> Self {
        let (x, y, w, h) = self.x_y_w_h();
        if w > h {
            Rect::from_x_y_w_h(x, y, h, h)
        } else {
            Rect::from_x_y_w_h(x, y, w, w)
        }
    }
}

struct Measurements {
    board_rect: Rect,
    stone_size: f32,
}

struct ViewModel {
    game: Game,
    rect: Rect,
}

impl ViewModel {
    fn new(size: usize, rect: Rect) -> Self {
        Self {
            game: Game::new(size),
            rect,
        }
    }

    fn calculate_measurements(&self) -> Measurements {
        let board_rect = self
            .rect
            .pad_top(32.0 + 24.0 + 16.0)
            .pad_left(32.0)
            .pad_right(32.0)
            .pad_bottom(32.0)
            .make_inset_square();

        let stone_size = board_rect.w() / (self.game.size as f32);

        Measurements {
            board_rect,
            stone_size,
        }
    }

    fn stone_project(&self, p: Pos) -> Vec2 {
        let m = self.calculate_measurements();
        let bl = m.board_rect.bottom_left();

        vec2(
            bl.x + (p.0 as f32 + 0.5) * m.stone_size,
            bl.y + (p.1 as f32 + 0.5) * m.stone_size,
        )
    }

    fn stone_unproject(&self, v: Vec2) -> Option<Pos> {
        let m = self.calculate_measurements();
        let bl = m.board_rect.bottom_left();

        let x: i32 = ((v.x - bl.x) / m.stone_size - 0.5).round() as i32;
        let y: i32 = ((v.y - bl.y) / m.stone_size - 0.5).round() as i32;
        if x >= 0 && x < (self.game.size as i32) && y >= 0 && y < (self.game.size as i32) {
            Some(Pos(x, y))
        } else {
            None
        }
    }
}

fn model(app: &App) -> ViewModel {
    app.set_loop_mode(LoopMode::Wait);

    ViewModel::new(13, app.window_rect())
}

fn event(app: &App, model: &mut ViewModel, event: Event) {
    model.rect = app.window_rect();

    match event {
        Event::DeviceEvent(
            _,
            DeviceEvent::Button {
                state: ElementState::Pressed,
                ..
            },
        ) => {
            let mouseat = model.stone_unproject(vec2(app.mouse.x, app.mouse.y));
            if let Some(p) = mouseat {
                model.game.try_place_stone(p);
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &ViewModel, frame: Frame) {
    let m = model.calculate_measurements();

    let draw = app.draw();
    draw.background().color(WHITE);

    let mouseat = model.stone_unproject(vec2(app.mouse.x, app.mouse.y));

    for x in 0..model.game.size {
        for y in 0..model.game.size {
            let pos = Pos(x as i32, y as i32);

            let stone = draw
                .ellipse()
                .xy(model.stone_project(pos))
                .w_h(m.stone_size - 4.0, m.stone_size - 4.0);

            match model.game.stone_at(pos) {
                None => {
                    stone
                        .color(color::rgba(0.0, 0.0, 0.0, 0.0))
                        .stroke(color::rgba(0.0, 0.0, 0.0, 0.4))
                        .stroke_weight(1.0);
                }
                Some(color) => {
                    stone
                        .color(match color {
                            Stone::Black => BLACK,
                            Stone::White => WHITE,
                        })
                        .stroke(BLACK)
                        .stroke_weight(2.5);
                }
            }
        }
    }

    if let Some(p) = mouseat {
        if !model.game.has_stone_at(p) {
            draw.ellipse()
                .xy(model.stone_project(p))
                .w_h(m.stone_size + 8.0, m.stone_size + 8.0)
                .color(match model.game.turn {
                    Stone::Black => BLACK,
                    Stone::White => WHITE,
                })
                .stroke(BLACK)
                .stroke_weight(3.0);
        }
    }

    let white_captures = model.game.state.captures[Stone::White];
    let black_captures = model.game.state.captures[Stone::Black];
    let mut pieces: Vec<String> = vec![];

    if white_captures + black_captures == 0 {
        pieces.push("No captures".to_string());
    } else if white_captures > 0 && black_captures > 0 {
        pieces.push(format!(
            "Captures: {} white, {} black",
            white_captures, black_captures
        ));
    } else if white_captures > 0 {
        pieces.push(format!("Captures: {} white", white_captures));
    } else if black_captures > 0 {
        pieces.push(format!("Captures: {} black", black_captures));
    }

    pieces.push(match model.game.turn {
        Stone::White => "White to play".to_string(),
        Stone::Black => "Black to play".to_string(),
    });

    draw.text(&pieces.join(" — "))
        .no_line_wrap()
        .x(0.0)
        .y(model.rect.top() - 32.0)
        .color(BLACK)
        .align_text_middle_y()
        .font_size(24);

    draw.to_frame(app, &frame).unwrap();
}

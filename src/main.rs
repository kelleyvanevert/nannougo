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

struct Measurements {
    padding: f32,
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
        let padding = 24.0;
        let board_rect = self.rect.pad(padding);
        let (w, h) = board_rect.w_h();
        let board_size = if w < h { w } else { h };
        let stone_size = board_size / (self.game.size as f32);

        Measurements {
            padding,
            stone_size,
        }
    }

    fn stone_project(&self, p: Pos) -> Vec2 {
        let m = self.calculate_measurements();

        let d = (self.game.size as f32 - 1.0) / 2.0;
        vec2(
            (p.0 as f32 - d) * m.stone_size,
            (p.1 as f32 - d) * m.stone_size,
        )
    }

    fn stone_unproject(&self, v: Vec2) -> Option<Pos> {
        let m = self.calculate_measurements();

        let d = (self.game.size as f32 - 1.0) / 2.0;
        let x: i32 = (v.x / m.stone_size + d).round() as i32;
        let y: i32 = (v.y / m.stone_size + d).round() as i32;
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

    let s = model.game.state.captures[Stone::Black];
    draw.text(format!("{}", s).as_str())
        .x(model.rect.left() + m.padding)
        .y(model.rect.top() - m.padding)
        .color(BLACK)
        .align_text_bottom()
        .font_size(m.stone_size as u32);

    let s = model.game.state.captures[Stone::White];
    draw.text(format!("{}", s).as_str())
        .x(model.rect.left() + m.padding)
        .y(model.rect.top() - m.padding - m.stone_size)
        .color(BLACK)
        .align_text_bottom()
        .font_size(m.stone_size as u32);

    draw.to_frame(app, &frame).unwrap();
}

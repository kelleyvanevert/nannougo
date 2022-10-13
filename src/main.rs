use crate::game::Game;
use crate::pos::Pos;
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

struct Model {
    game: Game,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);

    Model {
        game: Game::new(13),
    }
}

fn scale(
    size: usize,
    rect: Rect<f32>,
) -> (
    Box<dyn Fn(Pos) -> Vec2>,
    Box<dyn Fn(f32, f32) -> Option<Pos>>,
) {
    let board = rect.pad(16.0);
    let (w, h) = board.w_h();
    let board_size = if w < h { w } else { h };
    let stone_size = board_size / (size as f32);

    let half = ((size as f32) - 1.0) / 2.0;

    (
        Box::new(move |p: Pos| {
            vec2(
                ((p.0 as f32) - half) * stone_size,
                ((p.1 as f32) - half) * stone_size,
            )
        }),
        Box::new(move |fx: f32, fy: f32| {
            let x: i32 = (fx / stone_size + half).round() as i32;
            let y: i32 = (fy / stone_size + half).round() as i32;
            if x >= 0 && x < (size as i32) && y >= 0 && y < (size as i32) {
                Some(Pos(x, y))
            } else {
                None
            }
        }),
    )
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::DeviceEvent(
            _,
            DeviceEvent::Button {
                state: ElementState::Pressed,
                ..
            },
        ) => {
            let (_place, unplace) = scale(model.game.size, app.window_rect());
            let mouseat = unplace(app.mouse.x, app.mouse.y);
            if let Some(p) = mouseat {
                model.game.try_place_stone(p);
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let rect = app.window_rect().pad(16.0);
    let (w, h) = rect.w_h();
    let board_size = if w < h { w } else { h };
    let stone_size = board_size / (model.game.size as f32);

    let (place, unplace) = scale(model.game.size, app.window_rect());

    let draw = app.draw();
    draw.background().color(WHITE);

    let mouseat = unplace(app.mouse.x, app.mouse.y);

    for x in 0..model.game.size {
        for y in 0..model.game.size {
            let stone = draw
                .ellipse()
                .xy(place(Pos(x as i32, y as i32)))
                .w_h(stone_size - 4.0, stone_size - 4.0);

            match model.game.board[y * model.game.size + x] {
                None => {
                    stone
                        .color(color::rgba(0.0, 0.0, 0.0, 0.0))
                        .stroke(color::rgba(0.0, 0.0, 0.0, 0.4))
                        .stroke_weight(1.0);
                }
                Some(color) => {
                    stone
                        .color(if color { BLACK } else { WHITE })
                        .stroke(BLACK)
                        .stroke_weight(2.5);
                }
            }
        }
    }

    if let Some(p) = mouseat {
        if !model.game.has_stone_at(p) {
            draw.ellipse()
                .xy(place(p))
                .w_h(stone_size + 8.0, stone_size + 8.0)
                .color(if model.game.turn { BLACK } else { WHITE })
                .stroke(BLACK)
                .stroke_weight(3.0);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

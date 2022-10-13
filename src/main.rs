use nannou::{color, event::ElementState, prelude::*, winit::event::DeviceEvent};

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct Game {
    turn: bool,
    board: [Option<bool>; 19 * 19],
}

impl Game {
    fn new() -> Game {
        Self {
            turn: false,
            board: [None; 19 * 19],
        }
    }

    fn place_stone(&mut self, x: i32, y: i32) {
        self.board[(y as usize) * 19 + (x as usize)] = Some(self.turn);
        self.turn = !self.turn;
    }

    fn stone_at(&self, x: i32, y: i32) -> Option<bool> {
        self.board[(y as usize) * 19 + (x as usize)]
    }

    fn has_stone_at(&self, x: i32, y: i32) -> bool {
        None != self.stone_at(x, y)
    }
}

struct Model {
    game: Game,
}

fn model(_app: &App) -> Model {
    Model { game: Game::new() }
}

fn scale(
    rect: Rect<f32>,
) -> (
    Box<dyn Fn(i32, i32) -> Vec2>,
    Box<dyn Fn(f32, f32) -> Option<(i32, i32)>>,
) {
    let board = rect.pad(16.0);
    let (w, h) = board.w_h();
    let board_size = if w < h { w } else { h };
    let stone_size = board_size / 19.0;

    (
        Box::new(move |x: i32, y: i32| {
            vec2(
                ((x as f32) - 9.0) * stone_size,
                ((y as f32) - 9.0) * stone_size,
            )
        }),
        Box::new(move |fx: f32, fy: f32| {
            let x: i32 = (fx / stone_size + 9.0).round() as i32;
            let y: i32 = (fy / stone_size + 9.0).round() as i32;
            if x >= 0 && x < 19 && y >= 0 && y < 19 {
                Some((x, y))
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
            let (_place, unplace) = scale(app.window_rect());
            let mouseat = unplace(app.mouse.x, app.mouse.y);
            if let Some((x, y)) = mouseat {
                if !model.game.has_stone_at(x, y) {
                    model.game.place_stone(x, y);
                }
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let rect = app.window_rect().pad(16.0);
    let (w, h) = rect.w_h();
    let board_size = if w < h { w } else { h };
    let stone_size = board_size / 19.0;

    let (place, unplace) = scale(app.window_rect());

    let draw = app.draw();
    draw.background().color(WHITE);

    let mouseat = unplace(app.mouse.x, app.mouse.y);

    for x in 0..19_usize {
        for y in 0..19_usize {
            let stone = draw
                .ellipse()
                .xy(place(x as i32, y as i32))
                .w_h(stone_size - 4.0, stone_size - 4.0);

            match model.game.board[y * 19 + x] {
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

    if let Some((x, y)) = mouseat {
        if !model.game.has_stone_at(x, y) {
            draw.ellipse()
                .xy(place(x, y))
                .w_h(stone_size + 8.0, stone_size + 8.0)
                .color(if model.game.turn { BLACK } else { WHITE })
                .stroke(BLACK)
                .stroke_weight(3.0);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

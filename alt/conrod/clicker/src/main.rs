#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use conrod::{Labelable, Positionable, Sizeable, Colorable, Theme, Ui, Widget};
use conrod::color;
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};

struct GameState {
    coins: u64,
    miners: u64,
    multiplier: u64,
    llvm_optimizations: u64
}

impl GameState {
    pub fn new() -> GameState {
        return GameState {
            coins: 0,
            miners: 0,
            multiplier: 1,
            llvm_optimizations: 1
        };
    }

    pub fn add_coins(&mut self, amount: u64) {
        self.coins += amount;
    }

    pub fn manual_mine(&mut self) {
        let mult = self.multiplier;
        let llvm = self.llvm_optimizations;

        let total_amount = mult.pow(2) + llvm.pow(4);
        self.add_coins(total_amount);
    }

    pub fn tick_second(&mut self) {
        let miners = self.miners;
        let mult = self.multiplier;
        self.add_coins(miners * mult);
    }

    pub fn miner_price(&self) -> u64 {
        return self.miners.pow(3);
    }

    pub fn multiplier_price(&self) -> u64 {
        return 7u64.pow(self.multiplier as u32);
    }

    pub fn llvm_price(&self) -> u64 {
        return 10u64.pow(self.llvm_optimizations as u32);
    }
}

fn main() {
    let window: PistonWindow = WindowSettings::new("Rust Clicker", [400, 270]).build().unwrap();

    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    let mut game = GameState::new();

    let mut button_color = color::random();
    let mut bg_hue = 0.0f32;
    let mut colorful_mode = false;

    let mut deltatime_total: f64 = 0.0;

    for event in window.ups(30) {
        ui.handle_event(&event);

        event.update(|args| ui.set_widgets(|ui| {
            widget_ids!(MINE_BUTTON,
                        MINER_BUTTON,
                        MULTIPLIER_BUTTON,
                        LLVM_BUTTON,
                        COLORFULMODE_BUTTON,
                        COINS_TEXT);

            let x_zero = -(ui.win_w/2.0);
            let y_zero = ui.win_h/2.0;

            deltatime_total += args.dt;
            if deltatime_total > 0.10 {
                game.tick_second();
                deltatime_total = deltatime_total - 0.10;
            }

            if colorful_mode {
                bg_hue += args.dt as f32 / 2.0;
                bg_hue = bg_hue % 6.28;
                conrod::Background::new()
                    .hsl(bg_hue, 0.35, 0.5)
                    .set(ui);
            }else {
                conrod::Background::new()
                    .set(ui);
            }

            conrod::Button::new()
                .x(x_zero + 50.0)
                .y(y_zero - 50.0)
                .dimensions(80.0, 80.0)
                .label("Mine!")
                .color(button_color)
                .react(|| {
                    game.manual_mine();
                    button_color = color::random();
                })
                .set(MINE_BUTTON, ui);

            conrod::Button::new()
                .right(20.0)
                .dimensions(270.0, 45.0)
                .label(format!("Buy auto-miner (${})", game.miner_price()).trim())
                .enabled(game.coins > game.miner_price())
                .react(|| {
                    game.coins -= game.miner_price();
                    game.miners += 1;
                })
                .set(MINER_BUTTON, ui);

            conrod::Button::new()
                .down(10.0)
                .dimensions(270.0, 45.0)
                .label(format!("Buy multiplier (${})", game.multiplier_price()).trim())
                .enabled(game.coins > game.multiplier_price())
                .react(|| {
                    game.coins -= game.multiplier_price();
                    game.multiplier += 1;
                })
                .set(MULTIPLIER_BUTTON, ui);

            conrod::Button::new()
                .down(10.0)
                .dimensions(270.0, 45.0)
                .label(format!("Optimize LLVM (${})", game.llvm_price()).trim())
                .enabled(game.coins > game.llvm_price())
                .react(|| {
                    game.coins -= game.llvm_price();
                    game.llvm_optimizations += 1;
                })
                .set(LLVM_BUTTON, ui);

                conrod::Button::new()
                    .down(10.0)
                    .dimensions(270.0, 45.0)
                    .label("Toggle Rainbows ($100,000)")
                    .enabled(game.coins > 100_000)
                    .react(|| {
                        colorful_mode = !colorful_mode;
                        game.coins -= 100_000;
                    })
                    .set(COLORFULMODE_BUTTON, ui);

            conrod::Text::new(&format!("{} RustCoins", game.coins))
                .down(20.0)
                .align_text_left()
                .rgb(255.0, 255.0, 255.0)
                .set(COINS_TEXT, ui);
        }));

        event.draw_2d(|c, g| ui.draw_if_changed(c, g));
    }
}fn main() {
    println!("Hello, world!");
}

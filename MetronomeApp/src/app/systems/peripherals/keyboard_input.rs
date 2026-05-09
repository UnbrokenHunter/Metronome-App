// keyboard.rs
#![allow(dead_code)]
use eframe::egui::{Context, Key, Modifiers};
use once_cell::sync::OnceCell;

static GLOBAL_CTX: OnceCell<Context> = OnceCell::new();

/// Call this early (e.g., first line of your `update`), once.
/// Safe to call repeatedly; only the first call takes effect.
pub fn init_keyboard_ctx(ctx: &Context) {
    let _ = GLOBAL_CTX.set(ctx.clone()); // `Context` is cheap to clone
}

fn ctx() -> &'static Context {
    GLOBAL_CTX
        .get()
        .expect("Keyboard ctx not initialized. Call `init_keyboard_ctx(ctx)` first.")
}

pub struct Keyboard;

impl Keyboard {
    pub fn pressed(key: Key) -> bool {
        ctx().input(|i| i.key_pressed(key))
    }

    pub fn down(key: Key) -> bool {
        ctx().input(|i| i.key_down(key))
    }

    pub fn released(key: Key) -> bool {
        ctx().input(|i| i.key_released(key))
    }

    pub fn modifiers() -> Modifiers {
        ctx().input(|i| i.modifiers)
    }

    pub fn ctrl() -> bool {
        Self::modifiers().ctrl
    }

    pub fn shift() -> bool {
        Self::modifiers().shift
    }

    pub fn alt() -> bool {
        Self::modifiers().alt
    }

    pub fn command() -> bool {
        Self::modifiers().command
    }

    pub fn vertical() -> i8 {
        let mut val = 0;
        if Self::pressed(Key::ArrowUp) || Self::pressed(Key::S) {
            val -= 1;
        }

        if Self::pressed(Key::ArrowUp) || Self::pressed(Key::W) {
            val += 1;
        }

        val
    }

    pub fn horizontal() -> i8 {
        let mut val = 0;
        if Self::pressed(Key::ArrowRight) || Self::pressed(Key::D) {
            val += 1;
        }

        if Self::pressed(Key::ArrowLeft) || Self::pressed(Key::A) {
            val -= 1;
        }

        val
    }

    pub fn vertical_continous() -> i8 {
        let mut val = 0;
        if Self::down(Key::ArrowUp) || Self::down(Key::S) {
            val -= 1;
        }

        if Self::down(Key::ArrowUp) || Self::down(Key::W) {
            val += 1;
        }

        val
    }

    pub fn horizontal_continous() -> i8 {
        let mut val = 0;
        if Self::down(Key::ArrowRight) || Self::down(Key::D) {
            val += 1;
        }

        if Self::down(Key::ArrowLeft) || Self::down(Key::A) {
            val -= 1;
        }

        val
    }
}

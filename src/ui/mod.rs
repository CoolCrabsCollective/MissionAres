use bevy::prelude::Val;

pub(crate) mod battery_ui;
pub(crate) mod control_ui;
pub(crate) mod interactive_button;
pub(crate) mod win_screen;

pub fn Px_dynamic(i: f32) -> Val {
    Val::Vw(i / 1920.0 * 100.0)
}

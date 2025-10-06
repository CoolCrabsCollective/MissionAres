use crate::Transform;
use bevy::app::{App, Plugin, Update};
use bevy::color::{Color, Luminance, Srgba};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, BackgroundColor, BorderColor, Changed, Children, Component, ImageNode, Interaction,
    Query, Text, TextColor,
};

#[derive(Component)]
#[component(immutable)]
pub struct InteractiveButton {
    pub regular_background_color: Color,
    pub hover_background_color: Color,
    pub pressed_background_color: Color,
    pub regular_border_color: Color,
    pub hover_border_color: Color,
    pub pressed_border_color: Color,
    pub regular_text_color: Color,
    pub hover_text_color: Color,
    pub pressed_text_color: Color,
    pub regular_image_color: Color,
    pub hover_image_color: Color,
    pub pressed_image_color: Color,
    pub pressed_scale: Vec2,
    pub hover_scale: Vec2,
    pub regular_scale: Vec2,
}

impl InteractiveButton {
    pub fn simple(background_color: Color, text_color: Color, scaling: bool) -> Self {
        let darken_bg = background_color.luminance() > 0.5;
        let darken_text = text_color.luminance() > 0.5;

        let hover_bg = if darken_bg {
            background_color.darker(0.05)
        } else {
            background_color.lighter(0.05)
        };
        let pressed_bg = if darken_bg {
            background_color.darker(0.1)
        } else {
            background_color.lighter(0.1)
        };
        let hover_text = if darken_text {
            text_color.darker(0.1)
        } else {
            text_color.lighter(0.1)
        };
        let pressed_text = if darken_text {
            text_color.darker(0.2)
        } else {
            text_color.lighter(0.2)
        };

        Self {
            regular_background_color: background_color,
            hover_background_color: hover_bg,
            pressed_background_color: pressed_bg,
            regular_border_color: background_color,
            hover_border_color: hover_bg,
            pressed_border_color: pressed_bg,
            regular_text_color: text_color,
            hover_text_color: hover_text,
            pressed_text_color: pressed_text,
            pressed_scale: if scaling {
                Vec2::new(0.9, 0.9)
            } else {
                Vec2::new(1.0, 1.0)
            },
            hover_scale: if scaling {
                Vec2::new(1.1, 1.1)
            } else {
                Vec2::new(1.0, 1.0)
            },
            ..default()
        }
    }

    pub fn simple_image(
        background_color: Color,
        regular_image_color: Color,
        hover_image_color: Color,
        pressed_image_color: Color,
        scaling: bool,
    ) -> Self {
        let mut button = InteractiveButton::simple(background_color, Color::WHITE, scaling);

        button.regular_image_color = regular_image_color;
        button.hover_image_color = hover_image_color;
        button.pressed_image_color = pressed_image_color;

        button
    }
}

impl Default for InteractiveButton {
    fn default() -> Self {
        Self {
            regular_background_color: Color::Srgba(Srgba::new(0.3, 0.3, 0.3, 1.0)),
            hover_background_color: Color::Srgba(Srgba::new(0.1, 0.1, 0.1, 1.0)),
            pressed_background_color: Color::Srgba(Srgba::new(0.5, 0.5, 0.5, 1.0)),
            regular_border_color: Color::Srgba(Srgba::new(0.3, 0.3, 0.3, 1.0)),
            hover_border_color: Color::Srgba(Srgba::new(0.1, 0.1, 0.1, 1.0)),
            pressed_border_color: Color::Srgba(Srgba::new(0.5, 0.5, 0.5, 1.0)),
            regular_text_color: Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 1.0)),
            hover_text_color: Color::Srgba(Srgba::new(0.9, 0.9, 0.9, 1.0)),
            pressed_text_color: Color::Srgba(Srgba::new(0.8, 0.8, 0.8, 1.0)),
            regular_image_color: Color::WHITE,
            hover_image_color: Color::srgb(0.9, 0.9, 0.9),
            pressed_image_color: Color::srgb(0.8, 0.8, 0.8),
            pressed_scale: Vec2::new(0.9, 0.9),
            hover_scale: Vec2::new(1.1, 1.1),
            regular_scale: Vec2::new(1.0, 1.0),
        }
    }
}

pub struct InteractiveButtonPlugin;

impl Plugin for InteractiveButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interact);
    }
}

fn interact(
    mut button_query: Query<
        (
            &Interaction,
            &InteractiveButton,
            Option<&mut Transform>,
            Option<&mut BackgroundColor>,
            Option<&mut BorderColor>,
            Option<&mut ImageNode>,
            Option<&Children>,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<(&Text, &mut TextColor)>,
) {
    for (interaction, button, transform, bg_color, bor_color, img, children) in &mut button_query {
        let color = match *interaction {
            Interaction::Pressed => button.pressed_text_color,
            Interaction::Hovered => button.hover_text_color,
            Interaction::None => button.regular_text_color,
        };

        if children.is_some() {
            for child in children.unwrap() {
                if let Ok(mut text) = text_query.get_mut(*child) {
                    text.1.0 = color;
                }
            }
        }

        if bg_color.is_some() {
            bg_color.unwrap().0 = match *interaction {
                Interaction::Pressed => button.pressed_background_color,
                Interaction::Hovered => button.hover_background_color,
                Interaction::None => button.regular_background_color,
            };
        }

        if bor_color.is_some() {
            bor_color.unwrap().0 = match *interaction {
                Interaction::Pressed => button.pressed_border_color,
                Interaction::Hovered => button.hover_border_color,
                Interaction::None => button.regular_border_color,
            };
        }

        if img.is_some() {
            img.unwrap().color = match *interaction {
                Interaction::Pressed => button.pressed_image_color,
                Interaction::Hovered => button.hover_image_color,
                Interaction::None => button.regular_image_color,
            };
        }

        if transform.is_some() {
            transform.unwrap().scale = match *interaction {
                Interaction::Pressed => {
                    Vec3::new(button.pressed_scale.x, button.pressed_scale.y, 1.0)
                }
                Interaction::Hovered => Vec3::new(button.hover_scale.x, button.hover_scale.y, 1.0),
                Interaction::None => Vec3::new(button.regular_scale.x, button.regular_scale.y, 1.0),
            }
        }
    }
}

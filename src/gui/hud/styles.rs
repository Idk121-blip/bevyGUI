use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.8, 0.8, 0.8, 0.3);
pub const TEXT_BOX_SIDE: f32 = 30.;
pub const IMAGE_SIDE: f32 = 20.;

pub fn hud_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(15.0),
        left: Val::Px(0.),
        ..default()
    }
}
pub fn lhs_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(120.0),
        height: Val::Percent(60.0),
        margin: UiRect::new(Val::Px(10.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
        ..default()
    }
}
pub fn lhs_style_column() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(100.0),
        height: Val::Percent(60.0),
        margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(10.0)),
        top: Val::Px(300.0),
        ..default()
    }
}

pub fn rhs_style_column() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(130.0),
        height: Val::Percent(60.0),
        margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(10.0)),
        ..default()
    }
}

pub fn rhs_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(130.0),
        height: Val::Percent(60.0),
        margin: UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0)),
        ..default()
    }
}

pub fn image_style() -> Style {
    Style {
        margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(10.0), Val::Px(10.0)),
        width: Val::Px(IMAGE_SIDE),
        height: Val::Px(IMAGE_SIDE),
        ..default()
    }
}

pub fn get_text_style() -> TextStyle {
    TextStyle {
        font: Default::default(),
        font_size: 30.0,
        color: Color::rgb(0.0, 0.0, 0.0),
    }
}

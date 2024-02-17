use crate::components::RobotResource;
use crate::components::ENVIRONMENT;
use crate::components::SCORE;
use crate::gui::hud::components::*;
use crate::gui::hud::styles::*;
use crate::gui::hud::systems::utils::{get_clock_asset, get_weather_asset};
use crate::gui::utils::get_asset_of_content;
use bevy::prelude::*;
use bevy_progressbar::{ProgressBar, ProgressBarBundle, ProgressBarMaterial};
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Content;
pub(crate) fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    runner: NonSend<RobotResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    build_hud(
        &mut commands,
        &asset_server,
        &runner,
        &mut texture_atlases,
        &mut materials,
    );
}

pub(crate) fn build_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    runner: &NonSend<RobotResource>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    mut materials: &mut ResMut<Assets<ProgressBarMaterial>>,
) {
    commands
        .spawn((
            NodeBundle {
                style: hud_style(),
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // LHS
            parent
                .spawn(NodeBundle {
                    style: lhs_style_column(),
                    ..default()
                })
                .with_children(|parent| {
                    //commands.insert_resource(Inventory {});

                    for (content, quantity) in
                        runner.runner.get_robot().get_backpack().get_contents()
                    {
                        // let Some((x, y)) =
                        //     get_asset_of_content(content, asset_server, texture_atlases);
                        // let texture_handle = x.textures[y].clone();
                        match content {
                            Content::Fire => {}
                            Content::Bin(_) => {}
                            Content::Crate(_) => {}
                            Content::Bank(_) => {}
                            Content::Market(_) => {}
                            Content::Building => {}
                            Content::Bush(_) => {}
                            Content::JollyBlock(_) => {}
                            Content::Scarecrow => {}
                            Content::None => {}
                            Content::Fish(_) => {
                                parent
                                    .spawn(NodeBundle {
                                        style: lhs_style(),
                                        background_color: BACKGROUND_COLOR.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Star Image

                                        if let Some((texture_atlas, index)) = get_asset_of_content(
                                            content,
                                            &asset_server,
                                            &mut texture_atlases,
                                        ) {
                                            parent.spawn(AtlasImageBundle {
                                                style: image_style(),
                                                texture_atlas,
                                                texture_atlas_image: UiTextureAtlasImage {
                                                    index,
                                                    ..default()
                                                },
                                                ..default()
                                            });
                                            parent.spawn((
                                                TextBundle {
                                                    style: Style {
                                                        width: Val::Px(TEXT_BOX_SIDE),
                                                        ..default()
                                                    },
                                                    text: Text {
                                                        sections: vec![TextSection::new(
                                                            quantity.to_string(),
                                                            get_text_style(),
                                                        )],
                                                        alignment: TextAlignment::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                FishText {},
                                            ));
                                        }
                                    });
                            }
                            Content::Rock(_) => {
                                parent
                                    .spawn(NodeBundle {
                                        style: lhs_style(),
                                        background_color: BACKGROUND_COLOR.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Star Image
                                        if let Some((texture_atlas, index)) = get_asset_of_content(
                                            content,
                                            &asset_server,
                                            &mut texture_atlases,
                                        ) {
                                            parent.spawn(AtlasImageBundle {
                                                style: image_style(),
                                                texture_atlas,
                                                texture_atlas_image: UiTextureAtlasImage {
                                                    index,
                                                    ..default()
                                                },
                                                ..default()
                                            });

                                            parent.spawn((
                                                TextBundle {
                                                    style: Style {
                                                        width: Val::Px(TEXT_BOX_SIDE),
                                                        ..default()
                                                    },
                                                    text: Text {
                                                        sections: vec![TextSection::new(
                                                            quantity.to_string(),
                                                            get_text_style(),
                                                        )],
                                                        alignment: TextAlignment::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                RockText {},
                                            ));
                                        }
                                    });
                            }
                            Content::Tree(_) => {
                                parent
                                    .spawn(NodeBundle {
                                        style: lhs_style(),
                                        background_color: BACKGROUND_COLOR.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Star Image

                                        if let Some((texture_atlas, index)) = get_asset_of_content(
                                            content,
                                            &asset_server,
                                            &mut texture_atlases,
                                        ) {
                                            parent.spawn(AtlasImageBundle {
                                                style: image_style(),
                                                texture_atlas,
                                                texture_atlas_image: UiTextureAtlasImage {
                                                    index,
                                                    ..default()
                                                },
                                                ..default()
                                            });

                                            parent.spawn((
                                                TextBundle {
                                                    style: Style {
                                                        width: Val::Px(TEXT_BOX_SIDE),
                                                        ..default()
                                                    },
                                                    text: Text {
                                                        sections: vec![TextSection::new(
                                                            quantity.to_string(),
                                                            get_text_style(),
                                                        )],
                                                        alignment: TextAlignment::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                TreeText {},
                                            ));
                                        }
                                    });
                            }
                            Content::Garbage(_) => {
                                parent
                                    .spawn(NodeBundle {
                                        style: lhs_style(),
                                        background_color: BACKGROUND_COLOR.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Star Image

                                        if let Some((texture_atlas, index)) = get_asset_of_content(
                                            content,
                                            &asset_server,
                                            &mut texture_atlases,
                                        ) {
                                            parent.spawn(AtlasImageBundle {
                                                style: image_style(),
                                                texture_atlas,
                                                texture_atlas_image: UiTextureAtlasImage {
                                                    index,
                                                    ..default()
                                                },
                                                ..default()
                                            });

                                            parent.spawn((
                                                TextBundle {
                                                    style: Style {
                                                        width: Val::Px(TEXT_BOX_SIDE),
                                                        ..default()
                                                    },
                                                    text: Text {
                                                        sections: vec![TextSection::new(
                                                            quantity.to_string(),
                                                            get_text_style(),
                                                        )],
                                                        alignment: TextAlignment::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                GarbageText {},
                                            ));
                                        }
                                    });
                            }
                            Content::Coin(_) => {
                                parent
                                    .spawn(NodeBundle {
                                        style: lhs_style(),
                                        background_color: BACKGROUND_COLOR.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Star Image
                                        if let Some((texture_atlas, index)) = get_asset_of_content(
                                            content,
                                            &asset_server,
                                            &mut texture_atlases,
                                        ) {
                                            parent.spawn(AtlasImageBundle {
                                                style: image_style(),
                                                texture_atlas,
                                                texture_atlas_image: UiTextureAtlasImage {
                                                    index,
                                                    ..default()
                                                },
                                                ..default()
                                            });

                                            parent.spawn((
                                                TextBundle {
                                                    style: Style {
                                                        width: Val::Px(TEXT_BOX_SIDE),
                                                        ..default()
                                                    },
                                                    text: Text {
                                                        sections: vec![TextSection::new(
                                                            quantity.to_string(),
                                                            get_text_style(),
                                                        )],
                                                        alignment: TextAlignment::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                CoinText {},
                                            ));
                                        }
                                    });
                            }
                            Content::Water(_) => {
                                parent
                                    .spawn(NodeBundle {
                                        style: lhs_style(),
                                        background_color: BACKGROUND_COLOR.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Star Image

                                        if let Some((texture_atlas, index)) = get_asset_of_content(
                                            content,
                                            &asset_server,
                                            &mut texture_atlases,
                                        ) {
                                            parent.spawn(AtlasImageBundle {
                                                style: image_style(),
                                                texture_atlas,
                                                texture_atlas_image: UiTextureAtlasImage {
                                                    index,
                                                    ..default()
                                                },
                                                ..default()
                                            });
                                            parent.spawn((
                                                TextBundle {
                                                    style: Style {
                                                        width: Val::Px(TEXT_BOX_SIDE),
                                                        ..default()
                                                    },
                                                    text: Text {
                                                        sections: vec![TextSection::new(
                                                            quantity.to_string(),
                                                            get_text_style(),
                                                        )],
                                                        alignment: TextAlignment::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                WaterText {},
                                            ));
                                        }
                                    });
                            }
                        }
                    }
                });

            // RHS
            let weather;
            let time;
            {
                let env = ENVIRONMENT.lock().unwrap();
                weather = env.as_ref().unwrap().get_weather_condition();
                time = env.as_ref().unwrap().get_time_of_day_string();
            }
            parent
                .spawn(NodeBundle {
                    style: rhs_style_column(),
                    ..default()
                })
                .with_children(|parent| {
                    // Enemy Text
                    parent
                        .spawn(NodeBundle {
                            style: rhs_style(),
                            background_color: BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            let ((texture_atlas, index)) =
                                get_weather_asset(&weather, &asset_server, &mut texture_atlases);
                            parent.spawn((
                                AtlasImageBundle {
                                    style: Style {
                                        margin: UiRect::new(
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(10.0),
                                            Val::Px(10.0),
                                        ),
                                        width: Val::Px(30.),
                                        height: Val::Px(30.),
                                        ..default()
                                    },
                                    texture_atlas,
                                    texture_atlas_image: UiTextureAtlasImage { index, ..default() },
                                    ..default()
                                },
                                WeatherIcon {},
                            ));
                            parent.spawn(ImageBundle {
                                style: Style {
                                    margin: UiRect::new(
                                        Val::Px(4.0),
                                        Val::Px(4.0),
                                        Val::Px(0.0),
                                        Val::Px(0.0),
                                    ),
                                    width: Val::Px(30.),
                                    height: Val::Px(10.),
                                    ..default()
                                },
                                image: UiImage {
                                    texture: asset_server.load("arrow.png"),
                                    ..default()
                                },
                                ..default()
                            });
                            //todo! future weather
                            let ((texture_atlas, index)) =
                                get_weather_asset(&weather, &asset_server, &mut texture_atlases);
                            parent.spawn((
                                AtlasImageBundle {
                                    style: image_style(),
                                    texture_atlas,
                                    texture_atlas_image: UiTextureAtlasImage { index, ..default() },
                                    ..default()
                                },
                                FutureWeatherIcon {},
                            ));
                        });
                    parent
                        .spawn(NodeBundle {
                            style: rhs_style(),
                            background_color: BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            let (texture_atlas, index) =
                                get_clock_asset(&time, &asset_server, &mut texture_atlases);
                            parent.spawn((
                                AtlasImageBundle {
                                    style: Style {
                                        margin: UiRect::new(
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(10.0),
                                            Val::Px(10.0),
                                        ),
                                        width: Val::Px(30.0),
                                        height: Val::Px(30.0),
                                        ..default()
                                    },
                                    texture_atlas,
                                    texture_atlas_image: UiTextureAtlasImage { index, ..default() },
                                    ..default()
                                },
                                ClockIcon {},
                            ));

                            parent.spawn((
                                TextBundle {
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(time, get_text_style())],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ClockTime {},
                            ));
                        });
                    parent
                        .spawn(NodeBundle {
                            style: rhs_style(),
                            background_color: BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            let energy_lvl =
                                runner.runner.get_robot().get_energy().get_energy_level() as u32;
                            println!("{}", energy_lvl);
                            let mut bar = ProgressBar::new(vec![(1, Color::RED)]);
                            bar.set_progress(1.);

                            let mut bar2 = ProgressBar::new(vec![(1, Color::YELLOW)]);
                            //todo! rename
                            bar2.set_progress(energy_lvl as f32 / 1000.0);
                            let style = Style {
                                position_type: PositionType::Absolute,
                                width: Val::Px(100.),
                                height: Val::Px(10.),
                                top: Val::Px(20.0),
                                ..default()
                            };
                            parent.spawn(ProgressBarBundle::new(
                                style.clone(),
                                bar,
                                &mut materials,
                            ));
                            parent.spawn((
                                ProgressBarBundle::new(style, bar2, &mut materials),
                                EnergyBar {},
                            ));
                        });
                    parent
                        .spawn(NodeBundle {
                            style: rhs_style(),
                            background_color: BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            let energy_lvl =
                                runner.runner.get_robot().get_energy().get_energy_level() as u32;
                            println!("{}", energy_lvl);
                            let mut bar = ProgressBar::new(vec![(1, Color::GRAY)]);
                            bar.set_progress(1.);

                            let mut bar2 = ProgressBar::new(vec![(1, Color::GREEN)]);
                            //todo! rename
                            let score = SCORE.lock().unwrap();
                            bar2.set_progress(*score / 100.);
                            let style = Style {
                                position_type: PositionType::Absolute,
                                width: Val::Px(100.),
                                height: Val::Px(10.),
                                top: Val::Px(60.0),
                                ..default()
                            };
                            parent.spawn(ProgressBarBundle::new(
                                style.clone(),
                                bar,
                                &mut materials,
                            ));
                            parent.spawn((
                                ProgressBarBundle::new(style, bar2, &mut materials),
                                ScoreBar {},
                            ));
                        });
                });
        });
}

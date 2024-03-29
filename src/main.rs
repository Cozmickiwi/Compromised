use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::UiBundle,
    utils::application_root_dir,
};

mod compromised;
mod systems;

use crate::compromised::Compromised;

struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            systems::MovementSystem::new(),
            "movement_system",
            &["input_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.8, 0.694, 0.71, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::new(assets_dir, Compromised, game_data)?;
    game.run();

    Ok(())
}

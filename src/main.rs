use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct MonsterVolleyball;
impl SimpleState for MonsterVolleyball {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(config: Default::default());
    let app_root: PathBuf = application_root_dir()?;

    let config_dir: PathBuf = app_root.join(path: "config");
    let assets_dir: PathBuf = app_root.join(path: "assets");
    let config_display_path: PathBuf = config_dir.join(path: "display.ron");

    let game_data: GameDataBuilder::default(): GameDataBuilder
        .with_bundle
}
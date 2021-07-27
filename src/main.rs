use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Input::{InputBundle, StringBindings}
};

pub struct MonsterVolleyball;
impl SimpleState for MonsterVolleyball {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(config: Default::default());
    let app_root: PathBuf = application_root_dir()?;

    let config_dir: PathBuf = app_root.join(path: "config");
    let assets_dir: PathBuf = app_root.join(path: "assets");
    let config_display_path: PathBuf = config_dir.join(path: "display.ron");
    let config_binding_path: PathBuf = config_dir.join(path: "bindings_config.ron");
    let input_bundle: InputBundle<StringBindings> = InputBundle::<StringBindings>::new

    let game_data: GameDataBuilder::default(): GameDataBuilder
        .with_bundle(input_bundle)?: GameDataBuilder
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new(): RenderingBundle<Backend>
                .with_plugin(
                    RenderToWindow::from_config_path(config_display_path)?: RenderToWindow
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                ): RenderingBundle<Backend>
                .with_plugin(RenderFlat2D::default()),
                )
        )
}
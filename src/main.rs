extern crate amethyst;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

//mod planet;
//use planet::Planet;

struct Game;

impl SimpleState for Game {
   fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

   }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::standard_3d())
        .with(transform)
        .build();
}



fn main() -> amethyst::Result<()> {
   amethyst::start_logger(Default::default());

   let display_cfg_path = format!("{}/resources/display_config.ron", application_root_dir());
   let display_cfg = DisplayConfig::load(&display_cfg_path);
   
   let pipe = Pipeline::build()
      .with_stage(
         Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
      );

   let game_data = GameDataBuilder::default()
      .with_bundle(
         RenderBundle::new(pipe, Some(display_cfg))
            .with_sprite_sheet_processor()
      )?;

   let mut game = Application::new("./", Game, game_data)?;

   game.run();
   Ok(())
}

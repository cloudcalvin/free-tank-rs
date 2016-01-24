//Tankapp : ported by Calvin Maree cloudcalvin@me.com

extern crate piston;

mod ui; //effect land - eq to PaintWorld.scala + 
mod game; //state defs (starting state)
// mod model.world;

pub fn main() {
    let (mut ui, window) = ui::piston::PistonUI::new();

    let mut game = game::GameState::new();

    // game.randomize_map();

    ui.run(window, &mut game);
}

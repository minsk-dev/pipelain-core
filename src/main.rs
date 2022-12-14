use std::env;
use std::fmt::{Display, Formatter};
use std::fs::{read_to_string, File};
use std::process::exit;

use bracket_lib::color::{BLACK, RGB, YELLOW};
use bracket_lib::prelude::{
    main_loop, to_cp437, BError, BTerm, BTermBuilder, FontCharType, GameState, VirtualKeyCode,
};
use bracket_lib::random::RandomNumberGenerator;
use knuffel::{Decode, Error};
use specs::DenseVecStorage;
use specs::{Builder, World, WorldExt};
use specs::{Component, Entity};
use specs_derive::Component;

use crate::components::items::{
    Axe, BlocksMovement, Bush, CraftQueue, FirePit, Flint, InBackpack, Item, PickupQueue, Rose,
    Three, Tier, WoodenStick,
};
use crate::config::{load_config, Config};
use crate::gui::{MenuMode, UserInterfaceState};
use crate::logs::Log;
use crate::map::new_map;
use crate::player::{ControlMode, Player};
use crate::spawner::{axe, generate_items, player};
use crate::state::State;
use crate::systems::pickup::PickupSystem;

mod components;
mod config;
mod gui;
mod logs;
mod map;
mod player;
mod spawner;
mod state;
mod systems;

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

impl Renderable {
    pub fn new(glyph: FontCharType, fg: RGB) -> Self {
        Self {
            glyph,
            fg,
            bg: RGB::named(BLACK),
        }
    }
}

#[derive(Component, Debug)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Self {
        Name {
            name: name.to_string(),
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

fn main() -> BError {
    let config = {
        let inner = load_config();
        match inner {
            Config::Performance(perf) => perf,
        }
    };
    println!("{}", config);

    let context = BTermBuilder::simple80x50()
        .with_title("PipeLain")
        .with_dimensions(160, 100)
        .with_fps_cap(f32::from(config.fps_cap))
        .build()?;
    let mut state = State::default();

    // Qualifiers
    state.world.register::<Position>();
    state.world.register::<Renderable>();
    state.world.register::<Name>();
    state.world.register::<Item>();
    state.world.register::<Tier>();

    // Items
    state.world.register::<Flint>();
    state.world.register::<Bush>();
    state.world.register::<WoodenStick>();
    state.world.register::<Rose>();
    state.world.register::<Axe>();
    state.world.register::<FirePit>();
    state.world.register::<Three>();

    // Tags
    state.world.register::<BlocksMovement>();
    state.world.register::<Player>();
    state.world.register::<InBackpack>();

    // Queues
    state.world.register::<PickupQueue>();
    state.world.register::<CraftQueue>();

    state.world.insert(new_map());
    state.world.insert(Log {
        entries: vec![
            "the game has fully loaded".to_string(),
            "press the apostrophe/grave key to show/hide the logs".to_string(),
            "press tab to show/hide the right side menu".to_string(),
        ],
    });
    state.world.insert(UserInterfaceState::new(config.show_fps));

    let player = player(&mut state.world, 40, 25);
    state.world.insert(player);
    generate_items(&mut state.world);

    main_loop(context, state)
}

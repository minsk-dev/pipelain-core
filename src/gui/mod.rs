use bracket_lib::color::{GREEN, RGB, WHITE};

use crate::map::WIDTH;
use crate::player::ControlMode;
use crate::{BTerm, Log, World, BLACK};

pub mod menu;

#[derive(PartialEq, Copy, Clone, Default)]
pub enum MenuMode {
    #[default]
    Default,
    Interact,
    Inventory,
    Craft,
}

pub struct UserInterfaceState {
    pub log: bool,
    pub menu: bool,
    pub menu_mode: MenuMode,
    pub control_mode: ControlMode,
    pub selected_option: usize,
    pub show_performance_info: bool,
}

impl UserInterfaceState {
    pub fn new(show_performance_info: bool) -> Self {
        Self {
            show_performance_info,
            ..Self::default()
        }
    }
}

impl Default for UserInterfaceState {
    fn default() -> Self {
        UserInterfaceState {
            log: true,
            menu: true,
            menu_mode: MenuMode::default(),
            control_mode: ControlMode::default(),
            selected_option: 0,
            show_performance_info: true,
        }
    }
}

pub fn draw_log(world: &World, ctx: &mut BTerm) {
    let ui = world.fetch::<UserInterfaceState>();

    if !ui.log {
        return;
    }

    let width = WIDTH - 1;
    ctx.draw_box(0, 43, width, 6, RGB::named(WHITE), RGB::named(BLACK));

    let log = world.fetch::<Log>();
    let mut y = 44;
    for entry in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, entry);
            y += 1;
        }
    }
}

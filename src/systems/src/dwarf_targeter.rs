use comps::{Tile, Dwarf, Clickable, TileMap, RenderData};
use utils::Delta;

pub struct System {

}

impl System {
    pub fn new() -> System {
        System {

        }
    }
}

impl ::specs::System<Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, _: Delta) {
        use specs::Join;

        let (tiles, tile_map, mut dwarves, mut clickables, mut render_datas) = arg.fetch(|w|
            (
                w.read::<Tile>(),
                w.read_resource::<TileMap>(),
                w.write::<Dwarf>(),
                w.write::<Clickable>(),
                w.write::<RenderData>()
            )
        );

        for (tile, mut clickable, mut render_data) in (&tiles, &mut clickables, &mut render_datas).iter() {
            if clickable.clicked {
                let target = tile_map.get_tile(tile.get_location()).unwrap();
                clickable.clicked = false;
                render_data.set_tint(::art::spritesheet::tiles::FOREGROUND_TINT);

                for mut dwarf in (&mut dwarves).iter() {
                    if dwarf.get_entity_path().is_empty() {
                        *dwarf.get_mut_target_tile_opt() = Some(*target);
                        break;
                    }
                }
            }
        }
    }
}

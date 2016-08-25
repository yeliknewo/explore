use comps::{TileMap, Transform, PathsStorage, Dwarf};

pub struct System {

}

impl System {
    pub fn new() -> System {
        System {

        }
    }
}

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, _: ::utils::Delta) {
        use specs::Join;

        let (transforms, tile_map, paths_storage, mut dwarves) = arg.fetch(|w|
            (
                w.read::<Transform>(),
                w.read_resource::<TileMap>(),
                w.read_resource::<PathsStorage>(),
                w.write::<Dwarf>()
            )
        );

        // warn!("dwarf path applier run");
        for (transform, mut dwarf) in (&transforms, &mut dwarves).iter() {
            // warn!("dwarf path applier for");
            if let Some(to) = dwarf.get_mut_target_tile_opt().take() {
                // warn!("dwarf path applier if 1");
                if let Some(from) = tile_map.get_nearest_tile(&transform.get_pos()) {
                    warn!("dwarf path applier if 2");
                    if let Some(path_data) = paths_storage.get_path_both(*from, to) {
                        warn!("dwarf path applier if 3");
                        let mut path = path_data.0.clone();
                        if path_data.1 {
                            path.reverse();
                        }
                        dwarf.get_mut_entity_path().append(&mut path);
                    }
                }
            }
        }
    }
}

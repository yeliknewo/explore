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

        for (transform, mut dwarf) in (&transforms, &mut dwarves).iter() {
            if let Some(to) = dwarf.get_mut_target_tile_opt().take() {
                if let Some(from) = tile_map.get_nearest_tile(&transform.get_pos()) {
                    if let Some(path_data) = paths_storage.get_path_both(*from, to) {
                        let mut path = path_data.0.clone();
                        if path_data.1 {
                            path.reverse();
                        }

                        for entity in &path {
                            warn!("Path: {}", transforms.get(*entity).unwrap().get_pos());
                        }
                        dwarf.get_mut_entity_path().append(&mut path);
                    }
                }
            }
        }
    }
}

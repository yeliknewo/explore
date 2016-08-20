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
        use ::specs::Join;

        let (tile_map, mut tiles) = arg.fetch(|w|
            (w.read::<::comps::TileMap>(), w.write::<::comps::Tile>())
        );

        let tile_map = {
            let mut tile_map_opt = None;

            for tm in (&tile_map).iter() {
                tile_map_opt = Some(tm);
            }

            if tile_map_opt.is_none() {
                error!("Tile map is none in tile link updater run");
                return;
            }

            tile_map_opt.unwrap()
        };

        let mut links = vec!();

        for mut t in (&mut tiles).iter() {
            for link in t.get_mut_links().drain(..) {
                links.push(link);
            }

            for link in links.drain(..) {
                if let Some(entity) = tile_map.get_tile(&link.0) {
                    t.get_mut_fast_links().push((*entity, link.1));
                } else {
                    t.get_mut_links().push(link);
                }
            }
        }
    }
}

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
            (
                w.read_resource::<::comps::TileMap>(),
                w.write::<::comps::Tile>()
            )
        );

        let mut links = vec!();

        for mut tile in (&mut tiles).iter() {
            for link in tile.get_mut_links().drain(..) {
                links.push(link);
            }

            for link in links.drain(..) {
                if let Some(entity) = tile_map.get_tile(&link.0) {
                    tile.get_mut_fast_links().push((*entity, link.1));
                } else {
                    tile.get_mut_links().push(link);
                }
            }
        }
    }
}

pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>,
);

#[derive(Debug)]
pub enum SendEvent {
    NewTile(::math::Point2I, Vec<::comps::tile::Link>, ::comps::tile::PathType, [f32; 4]),
}

#[derive(Debug)]
pub enum RecvEvent {
    TileMade(::math::Point2I, ::specs::Entity),
}

pub struct System {
    channel: Channel,
}

impl System {
    pub fn new(channel: Channel) -> System {
        System {
            channel: channel,
        }
    }

    fn get_links(&self, location: &::math::Point2I) -> Vec<::comps::tile::Link> {
        vec!(
            (location.clone() + ::math::Point2I::new(1, -1), 3),
            (location.clone() + ::math::Point2I::new(0, -1), 1),
            (location.clone() + ::math::Point2I::new(-1, -1), 3),
            (location.clone() + ::math::Point2I::new(1, 0), 1),
            (location.clone() + ::math::Point2I::new(1, 1), 3),
            (location.clone() + ::math::Point2I::new(0, 1), 1),
            (location.clone() + ::math::Point2I::new(-1, 1), 3),
            (location.clone() + ::math::Point2I::new(-1, 0), 1),
        )
    }
}

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, _: ::utils::Delta) {
        use ::specs::Join;

        let (mut tile_map, dwarf, transform) = arg.fetch(|w|
            (w.write::<::comps::TileMap>(), w.read::<::comps::Dwarf>(), w.read::<::comps::Transform>())
        );

        let mut tile_map_opt = None;

        for tm in (&mut tile_map).iter() {
            tile_map_opt = Some(tm);
        }

        if tile_map_opt.is_none() {
            error!("Tile map is none in tile builder run");
            return;
        }

        let mut tile_map = tile_map_opt.unwrap();

        while match self.channel.1.try_recv() {
            Ok(event) => match event {
                RecvEvent::TileMade(location, entity) => {
                    tile_map.get_mut_tiles().insert(location, entity);
                    true
                },
            },
            Err(::std::sync::mpsc::TryRecvError::Empty) => false,
            Err(err) => {
                error!("try recv error: {}", err);
                false
            }
        } {

        }

        for (d, t) in (&dwarf, &transform).iter() {
            let location = t.get_pos();

            let gen_range = d.get_gen_range();

            for y in -gen_range..(gen_range + 1) {
                for x in -gen_range..(gen_range + 1) {
                    let conv: ::math::Point2I = location.clone().into();

                    let checking: ::math::Point2I = conv + ::math::Point2I::new(x, y);

                    if tile_map.get_tile(&checking).is_none() {
                        if checking.get_y() < 10
                        && (checking.get_x() != 2 || checking.get_y() != 2)
                        && (checking.get_x() != 3 || checking.get_y() != 2)
                        {
                            match self.channel.0.send(SendEvent::NewTile(
                                checking.clone(),
                                self.get_links(&checking),
                                ::comps::tile::PathType::Walkable,
                                {
                                    if checking.get_y() < 9 {
                                        ::art::spritesheet::tiles::GRASS_CENTER
                                    } else {
                                        ::art::spritesheet::tiles::GRASS_MID
                                    }
                                }
                            )) {
                                Ok(()) => (),
                                Err(err) => {
                                    error!("error while sending new tile: {}", err);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

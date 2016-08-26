use comps::PathFindingData;

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
            // (location.clone() + ::math::Point2I::new(1, -1), 2f64.sqrt()),
            (location.clone() + ::math::Point2I::new(0, -1), 1.0),
            // (location.clone() + ::math::Point2I::new(-1, -1), 2f64.sqrt()),
            (location.clone() + ::math::Point2I::new(1, 0), 1.0),
            // (location.clone() + ::math::Point2I::new(1, 1), 2f64.sqrt()),
            (location.clone() + ::math::Point2I::new(0, 1), 1.0),
            // (location.clone() + ::math::Point2I::new(-1, 1), 2f64.sqrt()),
            (location.clone() + ::math::Point2I::new(-1, 0), 1.0),
        )
    }
}

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, _: ::utils::Delta) {
        use ::specs::Join;

        let (dwarves, transforms, mut path_finding_datas, mut tile_map) = arg.fetch(|w|
            (
                w.read::<::comps::Dwarf>(),
                w.read::<::comps::Transform>(),
                w.write::<PathFindingData>(),
                w.write_resource::<::comps::TileMap>(),
            )
        );

        let mut any_new = false;

        while match self.channel.1.try_recv() {
            Ok(event) => match event {
                RecvEvent::TileMade(location, entity) => {
                    tile_map.get_mut_tiles().insert(location, entity.clone());
                    *path_finding_datas.get_mut(entity).unwrap().get_mut_entity_opt() = Some(entity);
                    any_new = true;
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

        if any_new {
            for mut path_finding_data in (&mut path_finding_datas).iter() {
                *path_finding_data.get_mut_path_data_opt() = None;
                *path_finding_data.get_mut_paths_done() = false;
                *path_finding_data.get_mut_links_done() = false;
            }
        }

        for (dwarf, transform) in (&dwarves, &transforms).iter() {
            let location = transform.get_pos();

            let gen_range = dwarf.get_gen_range();

            for y in -gen_range..(gen_range + 1) {
                for x in -gen_range..(gen_range + 1) {
                    let conv: ::math::Point2I = location.clone().into();

                    let checking: ::math::Point2I = conv + ::math::Point2I::new(x, y);

                    if tile_map.get_tile(&checking).is_none() && !tile_map.is_tile_placeheld(&checking) {
                        if checking.get_y() < 10
                        && checking.get_y() > -10
                        && checking.get_x() > -10
                        && checking.get_x() < 10
                        && !(checking.get_x() == 4 && checking.get_y() == 4)
                        && !(checking.get_x() == 5 && checking.get_y() == 5)
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
                            tile_map.hold_place(&checking);
                        }
                    }
                }
            }
        }
    }
}

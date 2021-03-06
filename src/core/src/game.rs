use sys::{PathFinder, DwarfTargeter};

use comps::{PathFindingData};

pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>,
);

#[derive(Debug)]
pub enum RecvEvent {
    TileBuilder(::sys::tile_builder::SendEvent),
    Exit,
}

#[derive(Debug)]
pub enum SendEvent {
    TileBuilder(::sys::tile_builder::RecvEvent),
    Exited,
}

pub struct Game {
    planner: ::specs::Planner<::utils::Delta>,
    last_time: u64,
    channel: Channel,
    fps_counter: ::utils::fps_counter::FpsCounter,
    tiles_render: ::comps::RenderType,
    p1_render: ::comps::RenderType,
}

impl Game {
    pub fn new(
        factory: &mut ::gfx_device_gl::Factory,
        mut game_event_hub: ::event::GameEventHub,
        mouse_location: ::math::Point2,
        screen_resolution: ::math::Point2,
        ortho_helper: ::math::OrthographicHelper
    ) -> Result<Game, ::utils::Error> {
        let target_delta_time = 1.0 / 60.0;

        let mut planner = {
            let mut w = ::specs::World::new();

            w.register::<::comps::RenderType>();
            w.register::<::comps::Transform>();
            w.register::<::comps::Camera>();
            w.register::<::comps::RenderData>();
            w.register::<::comps::Clickable>();
            w.register::<::comps::Dwarf>();
            w.register::<::comps::Living>();
            w.register::<::comps::Physical>();
            w.register::<::comps::Tile>();
            w.register::<::comps::PathFindingData>();

            w.add_resource(::comps::TileMap::new());
            w.add_resource(::comps::PathsStorage::new());

            ::specs::Planner::<::utils::Delta>::new(w, 8)
        };

        let mut renderer = try!(::sys::render::System::new(match game_event_hub.render_channel.take() {
            Some(channel) => channel,
            None => {
                error!("game event hub render channel was none");
                return Err(::utils::Error::Logged)
            },
        }));

        planner.mut_world().create_now()
            .with(::comps::Camera::new_from_ortho_helper(
                ::nalgebra::Point3::new(0.0, 0.0, 2.0),
                ::nalgebra::Point3::new(0.0, 0.0, 0.0),
                ::nalgebra::Vector3::new(0.0, 1.0, 0.0),
                &ortho_helper,
                true
            ))
            .build();

        let packet = ::art::spritesheet::make_square_render();

        let assets_folder = match ::find_folder::Search::ParentsThenKids(3, 3).for_folder("assets") {
            Ok(path) => path,
            Err(err) => {
                error!("error finding assets folder: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let tiles_render = {
            let texture = try!(
                ::graphics::texture::load_texture(
                    factory,
                    assets_folder.join(
                        "Tiles/tiles_spritesheet.png"
                    )
                )
            );
            try!(
                renderer.add_render_type_spritesheet(
                    factory,
                    &packet,
                    texture
                )
            )
        };

        let p1_render = {
            let texture = try!(
                ::graphics::texture::load_texture(
                    factory,
                    assets_folder.join(
                        "Player/p1_spritesheet.png"
                    )
                )
            );
            try!(
                renderer.add_render_type_spritesheet(
                    factory,
                    &packet,
                    texture
                )
            )
        };

        let p1_idle = vec!(::art::spritesheet::p1::STAND);

        let mut p1_walk = vec!();
        p1_walk.extend_from_slice(&::art::spritesheet::p1::WALK);

        let p1_fall = vec!(::art::spritesheet::p1::HURT);

        for _ in 0..1 {
            planner.mut_world().create_now()
                .with(p1_render)
                .with(::comps::Transform::new(
                    ::nalgebra::Isometry3::new(
                        ::nalgebra::Vector3::new(0.0, 0.0, 1.0),
                        ::nalgebra::Vector3::new(0.0, 0.0, 0.0)
                    ),
                    ::nalgebra::Vector3::new(1.0, 1.0, 1.0)
                ))
                .with(::comps::RenderData::new(::art::spritesheet::layers::PLAYER, ::art::spritesheet::p1::DEFAULT_TINT, ::art::spritesheet::p1::STAND, ::art::spritesheet::p1::SIZE))
                .with(::comps::Physical::new(::math::Point2::new(0.0, 0.0), ::math::Point2::new(1.0, 1.0), ::math::Point2::new(0.001, 0.001)))
                .with(::comps::Living::new(
                    p1_idle.clone(),
                    p1_walk.clone(),
                    p1_fall.clone()
                ))
                .with(::comps::Dwarf::new(5.0))
                .with(::comps::PathFindingData::new())
                .build();
        }

        planner.add_system(
            ::sys::control::System::new(
                match game_event_hub.control_channel.take() {
                    Some(channel) => channel,
                    None => {
                        error!("game event hub control channel was none");
                        return Err(::utils::Error::Logged);
                    }
                },
                ::math::Point2::new(10.0, 10.0),
                mouse_location,
                screen_resolution,
                ortho_helper,
            ),
            "control",
            30);

        planner.add_system(
            ::sys::TileLinkUpdater::new(),
            "tile_link_updater",
            29
        );

        planner.add_system(
            PathFinder::new(target_delta_time),
            "path finder",
            28
        );

        planner.add_system(
            DwarfTargeter::new(),
            "dwarf targeter",
            27
        );

        planner.add_system(
            ::sys::DwarfPathApplier::new(),
            "dwarf path applier",
            26
        );

        planner.add_system(
            ::sys::dwarf::System::new(),
            "dwarf",
            25
        );

        planner.add_system(
            ::sys::living::System::new(),
            "living",
            20
        );

        planner.add_system(
            ::sys::physical::System::new(),
            "physical",
            15
        );

        planner.add_system(
            ::sys::TileBuilder::new(match game_event_hub.tile_builder_channel.take() {
                Some(channel) => channel,
                None => {
                    error!("game event hub tile builder channel was none");
                    return Err(::utils::Error::Logged);
                }
            }),
            "tile_builder",
            14
        );

        planner.add_system(renderer, "renderer", 10);

        Ok(Game {
            planner: planner,
            last_time: ::time::precise_time_ns(),
            channel: match game_event_hub.game_channel.take() {
                Some(channel) => channel,
                None => {
                    error!("game event hub game channel was none");
                    return Err(::utils::Error::Logged);
                }
            },
            fps_counter: ::utils::fps_counter::FpsCounter::new(),
            p1_render: p1_render,
            tiles_render: tiles_render,
        })
    }


    pub fn frame(&mut self) -> bool {
        let new_time = ::time::precise_time_ns();
        let delta = (new_time - self.last_time) as ::utils::Delta / 1e9;
        self.last_time = new_time;

        match self.channel.1.try_recv() {
            Ok(RecvEvent::TileBuilder(::sys::tile_builder::SendEvent::NewTile(location, links, path_type, art_rect))) => {
                match self.channel.0.send(SendEvent::TileBuilder(::sys::tile_builder::RecvEvent::TileMade(location.clone(), self.planner.mut_world().create_now()
                    .with(::comps::Tile::new(location.clone(), links, path_type))
                    .with(self.tiles_render)
                    .with(::comps::Transform::new(
                        ::nalgebra::Isometry3::new(
                             ::nalgebra::Vector3::new(location.get_x() as f32, location.get_y() as f32, 0.0),
                             ::nalgebra::Vector3::new(0.0, 0.0, 0.0),
                            ),
                            ::nalgebra::Vector3::new(1.0, 1.0, 1.0)
                        )
                    )
                    .with(::comps::RenderData::new(::art::spritesheet::layers::TILES, ::art::spritesheet::tiles::FOREGROUND_TINT, art_rect, ::art::spritesheet::tiles::SIZE))
                    .with(::comps::Clickable::new(::math::Rect::new_from_coords(0.0, 0.0, 1.0, 1.0)))
                    .with(PathFindingData::new())
                    .build()))) {
                    Ok(()) => true,
                    Err(err) => {
                        error!("error while sending tile to tile builder: {}", err);
                        false
                    }
                }
            }
            Err(::std::sync::mpsc::TryRecvError::Empty) => {
                self.planner.dispatch(delta);
                self.fps_counter.frame(delta);
                true
            },
            Ok(RecvEvent::Exit) |
            Err(::std::sync::mpsc::TryRecvError::Disconnected) => {
                self.planner.wait();
                false
            },
        }
    }
}

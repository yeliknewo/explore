pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>,
);

#[derive(Debug)]
pub enum RecvEvent {
    Exit,
}

#[derive(Debug)]
pub enum SendEvent {
    Exited,
}

pub struct Game {
    planner: ::specs::Planner<::utils::Delta>,
    last_time: u64,
    target_fps_delta: ::utils::Delta,
    current_fps_delta: ::utils::Delta,
    channel: Channel,
    fps_counter: ::utils::fps_counter::FpsCounter,
}

impl Game {
    pub fn new(
        factory: &mut ::gfx_device_gl::Factory,
        mut game_event_hub: ::event::GameEventHub,
        mouse_location: ::math::Point2,
        screen_resolution: ::math::Point2,
        ortho_helper: ::math::OrthographicHelper
    ) -> Result<Game, ::utils::Error> {
        let default_tint = [0.3, 0.3, 0.3, 1.0];

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

        for y in -10..11i32 {
            for x in -10..11i32 {
                planner.mut_world().create_now()
                    .with(tiles_render)
                    .with(::comps::Transform::new(
                        ::nalgebra::Isometry3::new(
                            ::nalgebra::Vector3::new(x as f32, y as f32, 0.0),
                            ::nalgebra::Vector3::new(0.0, 0.0, 0.0),
                        ),
                        ::nalgebra::Vector3::new(1.0, 1.0, 1.0)
                    ))
                    .with(::comps::RenderData::new(default_tint, ::art::spritesheet::tiles::GRASS_MID, ::art::spritesheet::tiles::SIZE))
                    .with(::comps::Clickable::new(::math::Rect::new_from_coords(0.0, 0.0, 1.0, 1.0)))
                    .build();
            }
        }

        let p1_idle = vec!(::art::spritesheet::p1::STAND);

        let mut p1_walk = vec!();
        p1_walk.extend_from_slice(&::art::spritesheet::p1::WALK);

        let p1_fall = vec!(::art::spritesheet::p1::HURT);

        planner.mut_world().create_now()
            .with(p1_render)
            .with(::comps::Transform::new(
                ::nalgebra::Isometry3::new(
                    ::nalgebra::Vector3::new(0.0, 12.0, 1.0),
                    ::nalgebra::Vector3::new(0.0, 0.0, 0.0)
                ),
                ::nalgebra::Vector3::new(1.0, 1.0, 1.0)
            ))
            .with(::comps::RenderData::new(default_tint, ::art::spritesheet::p1::STAND, ::art::spritesheet::p1::SIZE))
            .with(::comps::Physical::new(::math::Point2::new(0.0, 0.0), ::math::Point2::new(1.0, 1.0), ::math::Point2::new(0.02, 0.02)))
            .with(::comps::Living::new(
                p1_idle,
                p1_walk,
                p1_fall
            ))
            .with(::comps::Dwarf::new(5.0))
            .build();

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
                default_tint
            ),
            "control",
            30);

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

        planner.add_system(renderer, "renderer", 10);

        Ok(Game {
            planner: planner,
            last_time: ::time::precise_time_ns(),
            target_fps_delta: 1.0 / 60.0,
            current_fps_delta: 0.0,
            channel: match game_event_hub.game_channel.take() {
                Some(channel) => channel,
                None => {
                    error!("game event hub game channel was none");
                    return Err(::utils::Error::Logged);
                }
            },
            fps_counter: ::utils::fps_counter::FpsCounter::new(),
        })
    }


    pub fn frame(&mut self) -> bool {
        let new_time = ::time::precise_time_ns();
        let delta = (new_time - self.last_time) as ::utils::Delta / 1e9;
        self.current_fps_delta += delta;
        self.last_time = new_time;

        match self.channel.1.try_recv() {
            Err(::std::sync::mpsc::TryRecvError::Empty) => {
                if self.current_fps_delta > self.target_fps_delta {
                    self.planner.dispatch(self.current_fps_delta);
                    self.fps_counter.frame(self.current_fps_delta);
                    self.current_fps_delta = 0.0;
                } else {
                    ::std::thread::sleep(::std::time::Duration::new(0, ((self.target_fps_delta - self.current_fps_delta* 0.99) * 1e9) as u32));
                }
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

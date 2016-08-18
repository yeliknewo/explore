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

        let (mut physical, mut render_data, mut living) = arg.fetch(|w|
            (w.write::<::comps::Physical>(), w.write::<::comps::RenderData>(), w.write::<::comps::Living>())
        );

        for (mut p, mut rd, mut l) in (&mut physical, &mut render_data, &mut living).iter() {
            match l.get_state() {
                ::comps::living::State::Idle => (),
                ::comps::living::State::Walking(dir) => {
                    if l.is_state_new() {
                        rd.set_spritesheet_rect(l.get_next_walking().clone());
                        let mirror = dir.get_x().is_sign_positive();
                        if mirror != rd.get_mirror() {
                            rd.set_mirror(mirror);
                        }
                    }
                    *p.get_mut_speed() = dir;
                },
                ::comps::living::State::Falling(speed) => {
                    *p.get_mut_speed().get_mut_y() = speed;
                    if l.is_state_new() {
                        rd.set_spritesheet_rect(l.get_next_falling().clone());
                    }
                },
            }
        }
    }
}

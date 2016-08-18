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

        let (mut physical, mut render_data, living) = arg.fetch(|w|
            (w.write::<::comps::Physical>(), w.write::<::comps::RenderData>(), w.read::<::comps::Living>())
        );

        for (mut p, mut rd, l) in (&mut physical, &mut render_data, &living).iter() {
            match l.get_state() {
                ::comps::living::State::Idle => (),
                ::comps::living::State::Walking(dir) => {
                    *p.get_mut_speed() = dir;
                    if l.is_state_new() {
                        // rd.set_texture_index(l.get_walking_index().clone());
                    }
                },
                ::comps::living::State::Falling(speed) => {
                    p.get_mut_speed().set_y(speed);
                    if l.is_state_new() {
                        // rd.set_texture_index(l.get_falling_index().clone());
                    }
                },
            }
        }
    }
}

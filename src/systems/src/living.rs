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

        let (mut physical, living) = arg.fetch(|w|
            (w.write::<::comps::Physical>(), w.read::<::comps::Living>())
        );

        for (mut p, l) in (&mut physical, &living).iter() {
            match l.get_state() {
                ::comps::living::State::Idle => (),
                ::comps::living::State::Walking(dir) => *p.get_mut_speed() = dir,
                ::comps::living::State::Falling(speed) => p.get_mut_speed().set_y(speed),
            }
        }
    }
}

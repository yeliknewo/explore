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
            match l.get_state_pair() {
                &(::comps::living::State::Idle, ::comps::living::StateData::Idle) => {
                    *p.get_mut_speed() = ::math::Point2::zero();
                },
                &(::comps::living::State::Walking, ::comps::living::StateData::Walking(ref dir)) => {
                    let mirror = dir.get_x().is_sign_positive();
                    if mirror != rd.get_mirror() && dir.get_x().abs() > 0.1 {
                        rd.set_mirror(mirror);
                    }
                    *p.get_mut_speed() = dir.clone();
                },
                &(::comps::living::State::Walking, ::comps::living::StateData::MoveTo(ref location)) => {
                    let mirror = location.get_x().is_sign_positive();
                    if mirror != rd.get_mirror() && location.get_x().abs() > 0.1 {
                        rd.set_mirror(mirror);
                    }
                    *p.get_mut_speed() = ::math::Point2::zero();
                    p.move_to(location.clone());
                }
                &(::comps::living::State::Falling, ::comps::living::StateData::Falling(speed)) => {
                    *p.get_mut_speed().get_mut_y() = speed;
                },
                &(ref other_state, _) => {
                    error!("invalid state pair: {}", other_state);
                },
            }
            l.update_state();
            rd.set_spritesheet_rect(l.get_next_rect().clone());
        }
    }
}

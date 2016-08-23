pub struct System {

}

impl System {
    pub fn new() -> System {
        System {

        }
    }
}

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, time: ::utils::Delta) {
        use specs::Join;

        let (mut transform, mut physical) = arg.fetch(|w|
            (
                w.write::<::comps::Transform>(),
                w.write::<::comps::Physical>()
            )
        );

        for (mut t, mut p) in (&mut transform, &mut physical).iter() {
            if let Some(move_to) = p.get_move_to() {
                t.set_position(move_to);
                continue;
            }

            if p.get_speed().is_zero() {
                continue;
            }

            let friction = p.get_friction();

            *p.get_mut_speed() *= friction;

            let mut speed = p.get_speed();

            if speed.length() < p.get_speed_break().length() {
                speed = ::math::Point2::zero();
            }

            if speed.is_zero() {
                continue;
            }

            speed *= time;
            t.add_position(speed);
        }
    }
}

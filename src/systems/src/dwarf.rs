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
        use ::specs::Join;

        let (transform, physical, mut dwarf, mut living) = arg.fetch(|w|
            (
                w.read::<::comps::Transform>(),
                w.read::<::comps::Physical>(),
                w.write::<::comps::Dwarf>(),
                w.write::<::comps::Living>()
            )
        );

        for (mut d, mut l, t, p) in (&mut dwarf, &mut living, &transform, &physical).iter() {
            let speed = d.get_speed();

            let mut pp = d.get_mut_point_path();

            while {
                if let Some(target) = pp.pop() {
                    let offset_from_target = target.clone() - t.get_pos();

                    if offset_from_target.length() < p.get_speed_break().length() {
                        true
                    } else if offset_from_target.length() < speed * time {
                        l.walk_to(target.clone());
                        false
                    } else {
                        l.walk(offset_from_target.normalized() * speed);
                        pp.push(target);
                        false
                    }
                } else {
                    l.idle();
                    false
                }
            } {

            }
        }
    }
}

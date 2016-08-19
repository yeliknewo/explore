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

        let (physical, transform, clickable, dwarf, mut living) = arg.fetch(|w|
            (w.read::<::comps::Physical>(), w.read::<::comps::Transform>(), w.read::<::comps::Clickable>(), w.read::<::comps::Dwarf>(), w.write::<::comps::Living>())
        );

        let mut target_opt = None;

        for (t, c) in (&transform, &clickable).iter() {
            if c.clicked {
                target_opt = Some(t.get_pos());
            }
        }

        for (d, mut l, t, p) in (&dwarf, &mut living, &transform, &physical).iter() {
            if let Some(target) = target_opt.as_ref() {
                let offset_from_target = target.clone() - t.get_pos();

                if offset_from_target.length() < p.get_speed_break().length() {
                    l.idle();
                } else if offset_from_target.length() < d.get_speed() * time {
                    l.walk_to(target.clone());
                } else {
                    l.walk(offset_from_target.normalized() * d.get_speed());
                }
            } else {
                l.idle();
            }
        }
    }
}

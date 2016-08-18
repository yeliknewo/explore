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

        let (physical, transform, clickable, mut dwarf, mut living) = arg.fetch(|w|
            (w.read::<::comps::Physical>(), w.read::<::comps::Transform>(), w.read::<::comps::Clickable>(), w.write::<::comps::Dwarf>(), w.write::<::comps::Living>())
        );

        let mut target_opt = None;

        for (t, c) in (&transform, &clickable).iter() {
            if c.clicked {
                target_opt = Some(t.get_pos());
            }
        }

        for (mut d, mut l, t, p) in (&mut dwarf, &mut living, &transform, &physical).iter() {
            if let Some(target) = target_opt.as_ref() {
                *d.get_mut_target_tile() = target.clone() - t.get_pos();
            }

            let normal = {
                let length = d.get_target_tile().length();
                if length < d.get_speed() * time {
                    if length < p.get_speed_break().length() {
                        d.get_target_tile() / time
                    } else {
                        d.get_target_tile()
                    }
                } else {
                    d.get_target_tile().normalized() * d.get_speed()
                }
            };

            if normal.is_finite() {
                l.walk(normal);
            } else {
                debug!("non finite dwarf target tile: {}", normal);
            }
        }
    }
}

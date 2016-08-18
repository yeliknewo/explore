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
                *d.get_mut_target_tile_opt() = Some(target.clone() - t.get_pos()); //unwraps are ok because of this

                if d.get_target_tile().unwrap().length() >= p.get_speed_break().length() {
                    let normal = {
                        let length = d.get_target_tile().unwrap().length();
                        if length < d.get_speed() * time {
                            l.walk_to(d.get_target_tile().unwrap().clone());
                            l.idle();
                            continue;
                        } else {
                            d.get_target_tile().unwrap().normalized() * d.get_speed()
                        }
                    };

                    if normal.is_finite() {
                        l.walk(normal);
                    } else {
                        debug!("non finite dwarf target tile: {}", normal);
                    }
                } else {
                    l.idle();
                }
            } else {
                l.idle();
            }
        }
    }
}

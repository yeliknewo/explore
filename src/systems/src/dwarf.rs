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

        let (transforms, physicals, mut dwarves, mut livings) = arg.fetch(|w|
            (
                w.read::<::comps::Transform>(),
                w.read::<::comps::Physical>(),
                w.write::<::comps::Dwarf>(),
                w.write::<::comps::Living>()
            )
        );

        for (mut dwarf, mut living, transform, physical) in (&mut dwarves, &mut livings, &transforms, &physicals).iter() {
            let speed = dwarf.get_speed();

            let mut entity_path = dwarf.get_mut_entity_path();

            while {
                if let Some(target_entity) = entity_path.pop() {
                    let target = transforms.get(target_entity).unwrap();
                    let offset_from_target = target.get_pos().clone() - transform.get_pos();

                    if offset_from_target.length() < physical.get_speed_break().length() {
                        true
                    } else if offset_from_target.length() < speed * time {
                        living.walk_to(target.get_pos());
                        false
                    } else {
                        living.walk(offset_from_target.normalized() * speed);
                        entity_path.push(target_entity);
                        false
                    }
                } else {
                    living.idle();
                    false
                }
            } {

            }
        }
    }
}

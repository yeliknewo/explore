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

        let (mut dwarf, mut path_finding_data) = arg.fetch(|w|
            (
                w.write::<::comps::Dwarf>(),
                w.write::<::comps::PathFindingData>()
            )
        );

        for (mut d, mut pfd) in (&mut dwarf, &mut path_finding_data).iter() {
            if *pfd.get_mut_path_ready() {
                d.get_mut_point_path().append(pfd.get_mut_path());
                *pfd.get_mut_path_ready() = false;
            }
        }
    }
}

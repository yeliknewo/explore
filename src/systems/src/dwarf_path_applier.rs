use comps::PathsStorage;

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

        let (mut dwarves, mut paths_storage) = arg.fetch(|w|
            (
                w.write::<::comps::Dwarf>(),
                w.write_resource::<PathsStorage>()
            )
        );

        for mut dwarf in (&mut dwarves).iter() {
            if let Some(target) = dwarf.get_target() {
                
            }
            if *pfd.get_mut_path_ready() {
                d.get_mut_point_path().append(pfd.get_mut_path());
                *pfd.get_mut_path_ready() = false;
            }
        }
    }
}

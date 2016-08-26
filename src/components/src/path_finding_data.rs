use specs::Entity;

pub struct Component {
    path_data: Option<PathData>,
    paths_done: bool,
    links_done: bool,
    entity: Option<Entity>,
}

impl Component {
    pub fn new() -> Component {
        Component {
            path_data: None,
            paths_done: false,
            links_done: false,
            entity: None,
        }
    }

    pub fn get_mut_entity_opt(&mut self) -> &mut Option<Entity> {
        &mut self.entity
    }

    pub fn get_mut_path_data_opt(&mut self) -> &mut Option<PathData> {
        &mut self.path_data
    }

    pub fn get_mut_path_data(&mut self) -> Option<&mut PathData> {
        self.path_data.as_mut()
    }

    pub fn get_mut_links_done(&mut self) -> &mut bool {
        &mut self.links_done
    }

    pub fn get_mut_paths_done(&mut self) -> &mut bool {
        &mut self.paths_done
    }

    pub fn get_entity(&self) -> Option<&Entity> {
        self.entity.as_ref()
    }

    pub fn are_paths_done(&self) -> bool {
        self.paths_done
    }

    pub fn are_links_done(&self) -> bool {
        self.links_done
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

// #[derive(Debug, Clone)]
// pub enum PathData {
//     Data(Vec<PathFindingNode>, Vec<usize>, Vec<usize>, ::math::Point2),
// }

pub type PathData = (
    Vec<PathNode>,  //nodes
    Vec<usize>,     //open
    Vec<usize>      //closed
);

pub type PathNode = (
    ::specs::Entity,    //tile id
    usize,              //from node index
    f64                 //distance
);

// #[derive(Debug, Clone)]
// pub enum PathFindingNode {
//     Start(::specs::Entity, usize, f64),
//     Node(::specs::Entity, usize, f64),
// }

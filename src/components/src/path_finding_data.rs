use specs::Entity;

pub struct Component {
    path_data: Option<PathData>,
    path_status: PathStatus,
    entity: Option<Entity>,
}

impl Component {
    pub fn new() -> Component {
        Component {
            path_data: None,
            path_status: PathStatus::Empty,
            entity: None,
        }
    }

    pub fn get_mut_entity_opt(&mut self) -> &mut Option<Entity> {
        &mut self.entity
    }

    pub fn get_mut_path_status(&mut self) -> &mut PathStatus {
        &mut self.path_status
    }

    pub fn get_mut_path_data_opt(&mut self) -> &mut Option<PathData> {
        &mut self.path_data
    }

    pub fn get_entity(&self) -> Option<&Entity> {
        self.entity.as_ref()
    }

    pub fn get_path_status(&self) -> &PathStatus {
        &self.path_status
    }

    pub fn get_mut_path_data(&mut self) -> Option<&mut PathData> {
        self.path_data.as_mut()
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

// #[derive(Debug, Clone)]
// pub enum PathData {
//     Data(Vec<PathFindingNode>, Vec<usize>, Vec<usize>, ::math::Point2),
// }

#[derive(Debug, Clone)]
pub enum PathStatus {
    Empty,
    DoneButHasLinks,
    WaitForNewTiles,
}

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

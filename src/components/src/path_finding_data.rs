pub struct Component {
    path_data: Option<PathData>,
    path_ready: bool,
    path: Vec<::math::Point2>,
    priority: usize,
}

impl Component {
    pub fn new() -> Component {
        Component {
            path_data: None,
            path_ready: false,
            path: vec!(),
            priority: 0,
        }
    }

    pub fn get_mut_path_ready(&mut self) -> &mut bool {
        &mut self.path_ready
    }

    pub fn get_mut_path(&mut self) -> &mut Vec<::math::Point2> {
        &mut self.path
    }

    pub fn get_mut_path_data_opt(&mut self) -> &mut Option<PathData> {
        &mut self.path_data
    }

    pub fn get_mut_path_data(&mut self) -> Option<&mut PathData> {
        self.path_data.as_mut()
    }

    pub fn get_mut_priority(&mut self) -> &mut usize {
        &mut self.priority
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

#[derive(Debug, Clone)]
pub enum PathData {
    Data(Vec<PathFindingNode>, Vec<usize>, Vec<usize>, ::math::Point2),
}

#[derive(Debug, Clone)]
pub enum PathFindingNode {
    Start(::specs::Entity, usize, f64),
    Node(::specs::Entity, usize, f64),
}

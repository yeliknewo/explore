#[derive(Debug)]
pub struct Component {
    location: ::math::Point2I,
    connections: Vec<::math::Point2I>,
    path_type: PathType,
}

impl Component {
    pub fn new(location: ::math::Point2I, connections: Vec<::math::Point2I>, path_type: PathType) -> Component {
        Component {
            location: location,
            connections: connections,
            path_type: path_type,
        }
    }

    pub fn get_location(&self) -> &::math::Point2I {
        &self.location
    }

    pub fn get_connections(&self) -> &Vec<::math::Point2I> {
        &self.connections
    }

    pub fn get_mut_connections(&mut self) -> &mut Vec<::math::Point2I> {
        &mut self.connections
    }

    pub fn get_path_type(&self) -> &PathType {
        &self.path_type
    }

    pub fn get_mut_path_type(&mut self) -> &mut PathType {
        &mut self.path_type
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

#[derive(Debug)]
pub enum PathType {
    Empty,
    Walkable,
    Blocking,
}

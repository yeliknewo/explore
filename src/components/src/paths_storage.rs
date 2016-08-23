pub struct Component {
    paths: Vec<FullPath>,
}

impl Component {
    pub fn new() -> Component {
        Component {
            paths: vec!(),
        }
    }

    pub fn get_paths(&self) -> &Vec<FullPath> {
        &self.paths
    }

    pub fn get_mut_paths(&mut self) -> &mut Vec<FullPath> {
        &mut self.paths
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

pub struct FullPath {
    start_entity: ::specs::Entity,
    start_location: ::math::Point2,
    start_tile_location: ::math::Point2I,
    end_entity: ::specs::Entity,
    end_location: ::math::Point2,
    end_tile_location: ::math::Point2I,
    path_entities: Vec<::specs::Entity>,
    path_locations: Vec<::math::Point2>,
    path_tile_locations: Vec<::math::Point2I>,
}

impl FullPath {
    pub fn new(start: (::specs::Entity, ::math::Point2, ::math::Point2I), end: (::specs::Entity, ::math::Point2, ::math::Point2I), path: (Vec<::specs::Entity>, Vec<::math::Point2>, Vec<::math::Point2I>)) -> FullPath {
        FullPath {
            start_entity: start.0,
            start_location: start.1,
            start_tile_location: start.2,
            end_entity: end.0,
            end_location: end.1,
            end_tile_location: end.2,
            path_entities: path.0,
            path_locations: path.1,
            path_tile_locations: path.2,
        }
    }

    pub fn get_start_entity(&self) -> ::specs::Entity {
        self.start_entity
    }

    pub fn get_start_location(&self) -> &::math::Point2 {
        &self.start_location
    }

    pub fn get_start_tile_location(&self) -> &::math::Point2I {
        &self.start_tile_location
    }

    pub fn get_end_entity(&self) -> ::specs::Entity {
        self.end_entity
    }

    pub fn get_end_location(&self) -> &::math::Point2 {
        &self.end_location
    }

    pub fn get_end_tile_location(&self) -> &::math::Point2I {
        &self.end_tile_location
    }

    pub fn get_path_entities(&self) -> &Vec<::specs::Entity> {
        &self.path_entities
    }

    pub fn get_path_locations(&self) -> &Vec<::math::Point2> {
        &self.path_locations
    }

    pub fn get_path_tile_locations(&self) -> &Vec<::math::Point2I> {
        &self.path_tile_locations
    }
}

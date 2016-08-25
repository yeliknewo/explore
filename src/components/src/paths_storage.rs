use std::collections::HashMap;
use specs::Entity;

pub struct Component {
    paths: HashMap<(Entity, Entity), Vec<Entity>>,
}

impl Component {
    pub fn new() -> Component {
        Component {
            paths: HashMap::new(),
        }
    }

    pub fn get_paths(&self) -> &HashMap<(Entity, Entity), Vec<Entity>> {
        &self.paths
    }

    pub fn get_mut_paths(&mut self) -> &mut HashMap<(Entity, Entity), Vec<Entity>> {
        &mut self.paths
    }

    pub fn add_path(&mut self, from: Entity, to: Entity, path: Vec<Entity>) {
        if to.get_id() > 500 {
            // warn!("add path from: {}, to: {}", from.get_id(), to.get_id());
        }
        self.paths.insert((from, to), path);
    }

    //doesn't need to be reversed
    pub fn get_path_from_to(&self, from: Entity, to: Entity) -> Option<&Vec<Entity>> {
        self.paths.get(&(from, to))
    }

    //must be reversed
    pub fn get_path_to_from(&self, to: Entity, from: Entity) -> Option<&Vec<Entity>> {
        self.paths.get(&(to, from))
    }

    //bool is if it needs to be reversed
    pub fn get_path_both(&self, from: Entity, to: Entity) -> Option<(&Vec<Entity>, bool)> {
        warn!("get path from: {}, to: {}", from.get_id(), to.get_id());
        match self.get_path_from_to(from, to) {
            Some(path) => Some((path, false)),
            None => match self.get_path_to_from(to, from) {
                Some(path) => Some((path, true)),
                None => {
                    warn!("path not found");
                    None
                },
            },
        }
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

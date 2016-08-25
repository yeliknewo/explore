use specs::Entity;

#[derive(Debug)]
pub struct Component {
    target_tile: Option<::specs::Entity>,
    entity_path: Vec<::specs::Entity>,
    speed: ::utils::Coord,
    gen_range: ::utils::CoordI,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new(speed: ::utils::Coord) -> Component {
        Component {
            target_tile: None,
            entity_path: vec!(),
            speed: speed,
            gen_range: 5,
        }
    }

    pub fn get_target_tile(&self) -> Option<&Entity> {
        self.target_tile.as_ref()
    }

    pub fn get_speed(&self) -> ::utils::Coord {
        self.speed
    }

    pub fn get_entity_path(&self) -> &Vec<::specs::Entity> {
        &self.entity_path
    }

    pub fn get_mut_entity_path(&mut self) -> &mut Vec<::specs::Entity> {
        &mut self.entity_path
    }

    pub fn get_mut_target_tile(&mut self) -> Option<&mut Entity> {
        self.target_tile.as_mut()
    }

    pub fn get_mut_target_tile_opt(&mut self) -> &mut Option<Entity> {
        &mut self.target_tile
    }

    pub fn get_gen_range(&self) -> ::utils::CoordI {
        self.gen_range
    }
}

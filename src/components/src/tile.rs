#[derive(Debug)]
pub struct Component {
    location: ::math::Point2I,
    links: Vec<Link>,
    fast_links: Vec<FastLink>,
    path_type: PathType,
}

impl Component {
    pub fn new(location: ::math::Point2I, links: Vec<Link>, path_type: PathType) -> Component {
        Component {
            location: location,
            links: links,
            fast_links: vec!(),
            path_type: path_type,
        }
    }

    pub fn get_location(&self) -> &::math::Point2I {
        &self.location
    }

    pub fn get_links(&self) -> &Vec<Link> {
        &self.links
    }

    pub fn get_fast_links(&self) -> &Vec<FastLink> {
        &self.fast_links
    }

    pub fn get_mut_fast_links(&mut self) -> &mut Vec<FastLink> {
        &mut self.fast_links
    }

    pub fn get_mut_links(&mut self) -> &mut Vec<Link> {
        &mut self.links
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

pub type Link = (
    ::math::Point2I,
    f64
);

pub type FastLink = (
    ::specs::Entity,
    f64
);

#[derive(Debug)]
pub enum PathType {
    Empty,
    Walkable,
    Blocking,
}

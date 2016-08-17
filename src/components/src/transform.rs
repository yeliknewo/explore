use nalgebra::{Translation, ToHomogeneous};

#[derive(Debug)]
pub struct Component {
    isometry: ::nalgebra::Isometry3<::utils::Coord>,
    scale: ::nalgebra::Vector3<::utils::Coord>,
}

impl Component {
    pub fn new_identity() -> Component {
        Component::new(
            ::nalgebra::Isometry3::new(::nalgebra::Vector3::new(0.0, 0.0, 0.0), ::nalgebra::Vector3::new(0.0, 0.0, 0.0)),
            ::nalgebra::Vector3::new(1.0, 1.0, 1.0)
        )
    }

    pub fn new(isometry: ::nalgebra::Isometry3<::utils::Coord>, scale: ::nalgebra::Vector3<::utils::Coord>) -> Component {
        Component {
            isometry: isometry,
            scale: scale,
        }
    }

    pub fn add_position(&mut self, pos_delta: ::math::Point2) {
        self.isometry.translation.x += pos_delta.get_x();
        self.isometry.translation.y += pos_delta.get_y();
    }

    pub fn get_model(&self) -> [[f32; 4]; 4] {
        let mut refer = *self.isometry.to_homogeneous().as_ref() as [[f32; 4]; 4];
        refer[0][0] *= self.scale.x;
        refer[1][1] *= self.scale.y;
        refer[2][2] *= self.scale.z;
        refer
    }

    pub fn get_pos(&self) -> ::math::Point2 {
        ::math::Point2::new(self.isometry.translation.x, self.isometry.translation.y)
    }

    pub fn get_gui_offset(&self) -> ::math::Point2 {
        let translation = self.isometry.translation();
        ::math::Point2::new(-translation.x, -translation.y)
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

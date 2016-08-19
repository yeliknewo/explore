use nalgebra::{Translation, ToHomogeneous};

#[derive(Debug)]
pub struct Component {
    isometry: ::nalgebra::Isometry3<::utils::GfxCoord>,
    scale: ::nalgebra::Vector3<::utils::GfxCoord>,
    pos: ::math::Point2,
}

impl Component {
    pub fn new_identity() -> Component {
        Component::new(
            ::nalgebra::Isometry3::new(::nalgebra::Vector3::new(0.0, 0.0, 0.0), ::nalgebra::Vector3::new(0.0, 0.0, 0.0)),
            ::nalgebra::Vector3::new(1.0, 1.0, 1.0)
        )
    }

    pub fn new(isometry: ::nalgebra::Isometry3<::utils::GfxCoord>, scale: ::nalgebra::Vector3<::utils::GfxCoord>) -> Component {
        Component {
            isometry: isometry,
            scale: scale,
            pos: ::math::Point2::new(isometry.translation.x as ::utils::Coord, isometry.translation.y as ::utils::Coord),
        }
    }

    pub fn set_position(&mut self, pos: ::math::Point2) {
        self.isometry.translation.x = pos.get_x() as ::utils::GfxCoord;
        self.isometry.translation.y = pos.get_y() as ::utils::GfxCoord;
    }

    pub fn add_position(&mut self, pos_delta: ::math::Point2) {
        self.isometry.translation.x += pos_delta.get_x() as ::utils::GfxCoord;
        self.isometry.translation.y += pos_delta.get_y() as ::utils::GfxCoord;
    }

    pub fn get_model(&self) -> [[::utils::GfxCoord; 4]; 4] {
        let mut refer = *self.isometry.to_homogeneous().as_ref();
        refer[0][0] *= self.scale.x;
        refer[1][1] *= self.scale.y;
        refer[2][2] *= self.scale.z;
        refer
    }

    pub fn get_pos(&self) -> ::math::Point2 {
        ::math::Point2::new(self.isometry.translation.x as f64, self.isometry.translation.y as f64)
    }

    pub fn get_gui_offset(&self) -> ::math::Point2 {
        let translation = self.isometry.translation();
        ::math::Point2::new(-translation.x as f64, -translation.y as f64)
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

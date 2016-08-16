use nalgebra::{Translation, ToHomogeneous};

pub struct Transform {
    isometry: ::nalgebra::Isometry3<::utils::Coord>,
    scale: ::nalgebra::Vector3<::utils::Coord>,
}

impl Transform {
    pub fn new_identity() -> Transform {
        Transform::new(
            ::nalgebra::Isometry3::new(::nalgebra::Vector3::new(0.0, 0.0, 0.0), ::nalgebra::Vector3::new(0.0, 0.0, 0.0)),
            ::nalgebra::Vector3::new(1.0, 1.0, 1.0)
        )
    }

    pub fn new(isometry: ::nalgebra::Isometry3<::utils::Coord>, scale: ::nalgebra::Vector3<::utils::Coord>) -> Transform {
        Transform {
            isometry: isometry,
            scale: scale,
        }
    }

    pub fn get_model(&self) -> [[::utils::Coord; 4]; 4] {
        let mut refer = *self.isometry.to_homogeneous().as_ref() as [[f32; 4]; 4];
        refer[0][0] *= self.scale.x;
        refer[1][1] *= self.scale.y;
        refer[2][2] *= self.scale.z;
        refer
    }

    pub fn get_gui_offset(&self) -> ::math::Point2 {
        let translation = self.isometry.translation();
        ::math::Point2::new(-translation.x, translation.y)
    }
}

impl ::specs::Component for Transform {
    type Storage = ::specs::VecStorage<Transform>;
}

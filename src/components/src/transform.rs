use nalgebra;
use nalgebra::ToHomogeneous;
use specs;

pub struct Transform {
    isometry: nalgebra::Isometry3<f32>,
    scale: nalgebra::Vector3<f32>,
}

impl Transform {
    pub fn new_identity() -> Transform {
        Transform::new(
            nalgebra::Isometry3::new(nalgebra::Vector3::new(0.0, 0.0, 0.0), nalgebra::Vector3::new(0.0, 0.0, 0.0)),
            nalgebra::Vector3::new(1.0, 1.0, 1.0)
        )
    }

    pub fn new(isometry: nalgebra::Isometry3<f32>, scale: nalgebra::Vector3<f32>) -> Transform {
        Transform {
            isometry: isometry,
            scale: scale,
        }
    }

    pub fn get_model(&self) -> [[f32; 4]; 4] {
        let mut refer = *self.isometry.to_homogeneous().as_ref();
        refer[0][0] *= self.scale.x;
        refer[1][1] *= self.scale.y;
        refer[2][2] *= self.scale.z;
        refer
    }
}

impl specs::Component for Transform {
    type Storage = specs::VecStorage<Transform>;
}

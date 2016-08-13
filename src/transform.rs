use nalgebra;
use nalgebra::ToHomogeneous;
use specs;

pub struct CompTransform {
    isometry: nalgebra::Isometry3<f32>,
    scale: nalgebra::Vector3<f32>,
}

impl CompTransform {
    pub fn new_identity() -> CompTransform {
        CompTransform::new(
            nalgebra::Isometry3::new(nalgebra::Vector3::new(0.0, 0.0, 0.0), nalgebra::Vector3::new(0.0, 0.0, 0.0)),
            nalgebra::Vector3::new(1.0, 1.0, 1.0)
        )
    }

    pub fn new(isometry: nalgebra::Isometry3<f32>, scale: nalgebra::Vector3<f32>) -> CompTransform {
        CompTransform {
            isometry: isometry,
            scale: scale,
        }
    }

    pub fn get_model(&self) -> [[f32; 4]; 4] {
        *self.isometry.to_homogeneous().as_ref()
    }
}

impl specs::Component for CompTransform {
    type Storage = specs::VecStorage<CompTransform>;
}

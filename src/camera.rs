use specs;
use nalgebra;
use nalgebra::ToHomogeneous;

pub struct CompCamera {
    eye: nalgebra::Point3<f32>,
    target: nalgebra::Point3<f32>,
    up: nalgebra::Vector3<f32>,
    proj: nalgebra::PerspectiveMatrix3<f32>,
}

impl CompCamera {
    pub fn new(
        eye: nalgebra::Point3<f32>,
        target: nalgebra::Point3<f32>,
        up: nalgebra::Vector3<f32>,
        proj: nalgebra::PerspectiveMatrix3<f32>
    ) -> CompCamera
    {
        CompCamera {
            eye: eye,
            target: target,
            up: up,
            proj: proj,
        }
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        *nalgebra::Isometry3::look_at_rh(&self.eye, &self.target, &self.up).to_homogeneous().as_ref()
    }

    pub fn get_proj(&self) -> [[f32; 4]; 4] {
        *self.proj.as_matrix().as_ref()
    }
}

impl specs::Component for CompCamera {
    type Storage = specs::VecStorage<CompCamera>;
}

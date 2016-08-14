use specs;
use nalgebra;
use nalgebra::ToHomogeneous;

pub struct CompCamera {
    eye: nalgebra::Point3<f32>,
    target: nalgebra::Point3<f32>,
    up: nalgebra::Vector3<f32>,
    proj: nalgebra::OrthographicMatrix3<f32>,
}

impl CompCamera {
    pub fn new(
        eye: nalgebra::Point3<f32>,
        target: nalgebra::Point3<f32>,
        up: nalgebra::Vector3<f32>,
        proj: nalgebra::OrthographicMatrix3<f32>
    ) -> CompCamera
    {
        CompCamera {
            eye: eye,
            target: target,
            up: up,
            proj: proj,
        }
    }

    pub fn set_offset(&mut self, (x, y): (f32, f32)) {
        self.set_eye(nalgebra::Point3::new(x, y, 2.0));
        self.set_target(nalgebra::Point3::new(x, y, 0.0));
    }

    fn set_eye(&mut self, eye: nalgebra::Point3<f32>) {
        self.eye = eye;
    }

    fn set_target(&mut self, target: nalgebra::Point3<f32>) {
        self.target = target;
    }

    pub fn get_offset(&self) -> (f32, f32) {
        (self.eye.x, self.eye.y)
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

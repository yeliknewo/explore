use nalgebra::ToHomogeneous;

#[derive(Debug)]
pub struct Camera {
    eye: ::nalgebra::Point3<::utils::Coord>,
    target: ::nalgebra::Point3<::utils::Coord>,
    up: ::nalgebra::Vector3<::utils::Coord>,
    proj: ::nalgebra::OrthographicMatrix3<::utils::Coord>,
    aspect_ratio: ::utils::Coord,
    is_main: bool,
}

impl Camera {
    pub fn new(
        eye: ::nalgebra::Point3<f32>,
        target: ::nalgebra::Point3<f32>,
        up: ::nalgebra::Vector3<f32>,
        proj: ::nalgebra::OrthographicMatrix3<f32>,
        aspect_ratio: ::utils::Coord,
        is_main: bool
    ) -> Camera
    {
        Camera {
            eye: eye,
            target: target,
            up: up,
            proj: proj,
            aspect_ratio: aspect_ratio,
            is_main: is_main,
        }
    }

    pub fn new_from_proj_args(
        eye: ::nalgebra::Point3<f32>,
        target: ::nalgebra::Point3<f32>,
        up: ::nalgebra::Vector3<f32>,
        aspect_ratio: ::utils::Coord,
        fov: ::utils::Coord,
        near: ::utils::Coord,
        far: ::utils::Coord,
        is_main: bool
    ) -> Camera {
        Camera::new(eye, target, up, ::nalgebra::OrthographicMatrix3::new_with_fov(aspect_ratio, fov, near, far), aspect_ratio, is_main)
    }

    pub fn new_from_ortho_helper(
        eye: ::nalgebra::Point3<f32>,
        target: ::nalgebra::Point3<f32>,
        up: ::nalgebra::Vector3<f32>,
        ortho_helper: &::math::OrthographicHelper,
        is_main: bool
    ) -> Camera {
        Camera::new(eye, target, up, ortho_helper.build_matrix(), ortho_helper.get_aspect_ratio(), is_main)
    }

    pub fn set_offset(&mut self, (x, y): (f32, f32)) {
        self.set_eye(::nalgebra::Point3::new(x, y, 2.0));
        self.set_target(::nalgebra::Point3::new(x, y, 0.0));
    }

    fn set_eye(&mut self, eye: ::nalgebra::Point3<f32>) {
        self.eye = eye;
    }

    fn set_target(&mut self, target: ::nalgebra::Point3<f32>) {
        self.target = target;
    }

    pub fn set_proj(&mut self, ortho_helper: & ::math::OrthographicHelper) {
        self.proj = ortho_helper.build_matrix();
        self.aspect_ratio = ortho_helper.get_aspect_ratio();
    }

    pub fn get_offset(&self) -> (f32, f32) {
        (self.eye.x, self.eye.y)
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        *::nalgebra::Isometry3::look_at_rh(&self.eye, &self.target, &self.up).to_homogeneous().as_ref()
    }

    pub fn get_proj(&self) -> [[f32; 4]; 4] {
        *self.proj.as_matrix().as_ref()
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }

    pub fn get_gui_offset(&self) -> ::math::Point2 {
        ::math::Point2::new(self.eye.x - (self.proj.zfar() - self.proj.znear()) / 2.0 * self.aspect_ratio, -self.eye.y - (self.proj.zfar() - self.proj.znear()) / 2.0)
    }

    pub fn screen_to_world_point(&self, screen_point: ::math::Point2) -> ::math::Point2 {
        let view_depth = self.proj.zfar() - self.proj.znear();

        ::math::Point2::new(
            (screen_point.get_x() + 1.0) / 2.0 * view_depth * self.aspect_ratio - view_depth,
            (screen_point.get_y() + 1.0) / -2.0 * view_depth + view_depth
        )
    }
}

impl ::specs::Component for Camera {
    type Storage = ::specs::VecStorage<Camera>;
}

use nalgebra::ToHomogeneous;

#[derive(Debug)]
pub struct Camera {
    eye: ::nalgebra::Point3<::utils::Coord>,
    target: ::nalgebra::Point3<::utils::Coord>,
    up: ::nalgebra::Vector3<::utils::Coord>,
    proj: ::nalgebra::OrthographicMatrix3<::utils::Coord>,
    aspect_ratio: ::utils::Coord,
    is_main: bool,
    dirty: bool,
}

impl Camera {
    pub fn new(
        eye: ::nalgebra::Point3<::utils::Coord>,
        target: ::nalgebra::Point3<::utils::Coord>,
        up: ::nalgebra::Vector3<::utils::Coord>,
        proj: ::nalgebra::OrthographicMatrix3<::utils::Coord>,
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
            dirty: true,
        }
    }

    pub fn new_from_proj_args(
        eye: ::nalgebra::Point3<::utils::Coord>,
        target: ::nalgebra::Point3<::utils::Coord>,
        up: ::nalgebra::Vector3<::utils::Coord>,
        aspect_ratio: ::utils::Coord,
        fov: ::utils::Coord,
        near: ::utils::Coord,
        far: ::utils::Coord,
        is_main: bool
    ) -> Camera {
        Camera::new(eye, target, up, ::nalgebra::OrthographicMatrix3::new_with_fov(aspect_ratio, fov, near, far), aspect_ratio, is_main)
    }

    pub fn new_from_ortho_helper(
        eye: ::nalgebra::Point3<::utils::Coord>,
        target: ::nalgebra::Point3<::utils::Coord>,
        up: ::nalgebra::Vector3<::utils::Coord>,
        ortho_helper: &::math::OrthographicHelper,
        is_main: bool
    ) -> Camera {
        Camera::new(eye, target, up, ortho_helper.build_matrix(), ortho_helper.get_aspect_ratio(), is_main)
    }

    pub fn set_offset(&mut self, offset: ::math::Point2) {
        self.set_eye(::nalgebra::Point3::new(offset.get_x(), offset.get_y(), 2.0));
        self.set_target(::nalgebra::Point3::new(offset.get_x(), offset.get_y(), 0.0));
        self.dirty = true;
    }

    fn set_eye(&mut self, eye: ::nalgebra::Point3<::utils::Coord>) {
        self.eye = eye;
    }

    fn set_target(&mut self, target: ::nalgebra::Point3<::utils::Coord>) {
        self.target = target;
    }

    pub fn set_proj(&mut self, ortho_helper: & ::math::OrthographicHelper) {
        self.proj = ortho_helper.build_matrix();
        self.aspect_ratio = ortho_helper.get_aspect_ratio();
        self.dirty = true;
    }

    pub fn get_offset(&self) -> ::math::Point2 {
        ::math::Point2::new(self.eye.x, self.eye.y)
    }

    pub fn get_view(&self) -> [[::utils::Coord; 4]; 4] {
        *::nalgebra::Isometry3::look_at_rh(&self.eye, &self.target, &self.up).to_homogeneous().as_ref()
    }

    pub fn get_proj(&self) -> [[::utils::Coord; 4]; 4] {
        *self.proj.as_matrix().as_ref()
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }

    pub fn screen_to_world_point(&self, screen_point: ::math::Point2) -> ::math::Point2 {
        let view_depth = self.proj.zfar() - self.proj.znear();

        let world_point = ::math::Point2::new(
            (((screen_point.get_x() * 2.0) - 1.0) * view_depth) * 4.0 / 5.0 + self.eye.x,
            (((1.0 - screen_point.get_y()) * 2.0 - 1.0) * view_depth / self.aspect_ratio) * 4.0 / 5.0 + self.eye.y
        );

        world_point
    }

    pub fn take_dirty(&mut self) -> bool {
        let temp = self.dirty;
        self.dirty = false;
        temp
    }
}

impl ::specs::Component for Camera {
    type Storage = ::specs::VecStorage<Camera>;
}

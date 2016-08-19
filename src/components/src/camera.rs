use nalgebra::ToHomogeneous;

#[derive(Debug)]
pub struct Component {
    eye: ::nalgebra::Point3<::utils::GfxCoord>,
    target: ::nalgebra::Point3<::utils::GfxCoord>,
    up: ::nalgebra::Vector3<::utils::GfxCoord>,
    proj: ::nalgebra::OrthographicMatrix3<::utils::GfxCoord>,
    aspect_ratio: ::utils::GfxCoord,
    is_main: bool,
    dirty: bool,
    dirty_2: bool,
}

impl Component {
    pub fn new(
        eye: ::nalgebra::Point3<::utils::GfxCoord>,
        target: ::nalgebra::Point3<::utils::GfxCoord>,
        up: ::nalgebra::Vector3<::utils::GfxCoord>,
        proj: ::nalgebra::OrthographicMatrix3<::utils::GfxCoord>,
        aspect_ratio: ::utils::GfxCoord,
        is_main: bool
    ) -> Component
    {
        Component {
            eye: eye,
            target: target,
            up: up,
            proj: proj,
            aspect_ratio: aspect_ratio,
            is_main: is_main,
            dirty: true,
            dirty_2: true,
        }
    }

    pub fn new_from_proj_args(
        eye: ::nalgebra::Point3<::utils::GfxCoord>,
        target: ::nalgebra::Point3<::utils::GfxCoord>,
        up: ::nalgebra::Vector3<::utils::GfxCoord>,
        aspect_ratio: ::utils::GfxCoord,
        fov: ::utils::GfxCoord,
        near: ::utils::GfxCoord,
        far: ::utils::GfxCoord,
        is_main: bool
    ) -> Component {
        Component::new(eye, target, up, ::nalgebra::OrthographicMatrix3::new_with_fov(aspect_ratio, fov, near, far), aspect_ratio, is_main)
    }

    pub fn new_from_ortho_helper(
        eye: ::nalgebra::Point3<::utils::GfxCoord>,
        target: ::nalgebra::Point3<::utils::GfxCoord>,
        up: ::nalgebra::Vector3<::utils::GfxCoord>,
        ortho_helper: &::math::OrthographicHelper,
        is_main: bool
    ) -> Component {
        Component::new(eye, target, up, ortho_helper.build_matrix(), ortho_helper.get_aspect_ratio(), is_main)
    }

    pub fn set_offset(&mut self, offset: ::math::Point2) {
        self.set_eye(::nalgebra::Point3::new(offset.get_x() as ::utils::GfxCoord, offset.get_y() as ::utils::GfxCoord, 2.0));
        self.set_target(::nalgebra::Point3::new(offset.get_x() as ::utils::GfxCoord, offset.get_y() as ::utils::GfxCoord, 0.0));
        self.set_dirty();
    }

    fn set_eye(&mut self, eye: ::nalgebra::Point3<::utils::GfxCoord>) {
        self.eye = eye;
    }

    fn set_target(&mut self, target: ::nalgebra::Point3<::utils::GfxCoord>) {
        self.target = target;
    }

    pub fn set_proj(&mut self, ortho_helper: & ::math::OrthographicHelper) {
        self.proj = ortho_helper.build_matrix();
        self.aspect_ratio = ortho_helper.get_aspect_ratio();
        self.dirty = true;
    }

    pub fn get_offset(&self) -> ::math::Point2 {
        ::math::Point2::new(self.eye.x as ::utils::Coord, self.eye.y as ::utils::Coord)
    }

    pub fn get_view(&self) -> [[::utils::GfxCoord; 4]; 4] {
        *::nalgebra::Isometry3::look_at_rh(&self.eye, &self.target, &self.up).to_homogeneous().as_ref()
    }

    pub fn get_proj(&self) -> [[::utils::GfxCoord; 4]; 4] {
        *self.proj.as_matrix().as_ref()
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }

    pub fn screen_to_world_point(&self, screen_point: ::math::Point2) -> ::math::Point2 {
        let view_depth = self.proj.zfar() - self.proj.znear();

        let world_point = ::math::Point2::new(
            (((screen_point.get_x() * 2.0) - 1.0) * view_depth as ::utils::Coord) * 4.0 / 5.0 + self.get_offset().get_x(),
            (((1.0 - screen_point.get_y()) * 2.0 - 1.0) * view_depth as ::utils::Coord / self.aspect_ratio as ::utils::Coord) * 4.0 / 5.0 + self.get_offset().get_y()
        );

        world_point
    }

    fn set_dirty(&mut self) {
        self.dirty = true;
        self.dirty_2 = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        self.dirty = false;
        if self.dirty {
            self.dirty_2 = false;
            return true;
        }
        return self.dirty_2;
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

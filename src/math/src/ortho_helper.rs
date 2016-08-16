#[derive(Clone, Debug)]
pub struct OrthographicHelper {
    aspect_ratio: ::utils::Coord,
    fov: ::utils::Coord,
    znear: ::utils::Coord,
    zfar: ::utils::Coord,
}

impl OrthographicHelper {
    pub fn new(
        aspect_ratio: ::utils::Coord,
        fov: ::utils::Coord,
        znear: ::utils::Coord,
        zfar: ::utils::Coord
    ) -> OrthographicHelper {
        OrthographicHelper {
            aspect_ratio: aspect_ratio,
            fov: fov,
            znear: znear,
            zfar: zfar,
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: ::utils::Coord) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn get_aspect_ratio(&self) -> ::utils::Coord {
        self.aspect_ratio
    }

    pub fn get_fov(&self) -> ::utils::Coord {
        self.fov
    }

    pub fn get_znear(&self) -> ::utils::Coord {
        self.znear
    }

    pub fn get_zfar(&self) -> ::utils::Coord {
        self.zfar
    }

    pub fn get_view_depth(&self) -> ::utils::Coord {
        self.get_zfar() - self.get_znear()
    }

    pub fn build_matrix(&self) -> ::nalgebra::OrthographicMatrix3<::utils::Coord> {
        ::nalgebra::OrthographicMatrix3::new_with_fov(self.get_aspect_ratio(), self.get_fov(), self.get_znear(), self.get_zfar())
    }
}

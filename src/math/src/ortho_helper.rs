#[derive(Clone, Debug)]
pub struct OrthographicHelper {
    aspect_ratio: ::utils::GfxCoord,
    fov: ::utils::GfxCoord,
    znear: ::utils::GfxCoord,
    zfar: ::utils::GfxCoord,
}

impl OrthographicHelper {
    pub fn new(
        aspect_ratio: ::utils::GfxCoord,
        fov: ::utils::GfxCoord,
        znear: ::utils::GfxCoord,
        zfar: ::utils::GfxCoord
    ) -> OrthographicHelper {
        OrthographicHelper {
            aspect_ratio: aspect_ratio,
            fov: fov,
            znear: znear,
            zfar: zfar,
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: ::utils::GfxCoord) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn get_aspect_ratio(&self) -> ::utils::GfxCoord {
        self.aspect_ratio
    }

    pub fn get_fov(&self) -> ::utils::GfxCoord {
        self.fov
    }

    pub fn get_znear(&self) -> ::utils::GfxCoord {
        self.znear
    }

    pub fn get_zfar(&self) -> ::utils::GfxCoord {
        self.zfar
    }

    pub fn get_view_depth(&self) -> ::utils::GfxCoord {
        self.get_zfar() - self.get_znear()
    }

    pub fn build_matrix(&self) -> ::nalgebra::OrthographicMatrix3<::utils::GfxCoord> {
        ::nalgebra::OrthographicMatrix3::new_with_fov(self.get_aspect_ratio(), self.get_fov(), self.get_znear(), self.get_zfar())
    }
}

use std::os::raw::c_int;

pub struct Interpolation(c_int);

impl Interpolation {
    const INTER_NEAREST: c_int = 0;
    const INTER_LINEAR: c_int = 1;
    const INTER_CUBIC: c_int = 2;
    const INTER_AREA: c_int = 3;
    const INTER_LANCZOS4: c_int = 4;
    const INTER_MAX: c_int = 7;
    const WARP_FILL_OUTLIERS: c_int = 8;
    const WARP_INVERSE_MAP: c_int = 1;


    pub fn nearest() -> Interpolation {
        return Interpolation(Self::INTER_NEAREST);
    }
    pub fn linear() -> Interpolation {
        return Interpolation(Self::INTER_LINEAR);
    }
    pub fn cubic() -> Interpolation {
        return Interpolation(Self::INTER_CUBIC);
    }
    pub fn area() -> Interpolation {
        return Interpolation(Self::INTER_AREA);
    }
    pub fn lanczos4() -> Interpolation {
        return Interpolation(Self::INTER_LANCZOS4);
    }
    pub fn max() -> Interpolation {
        return Interpolation(Self::INTER_MAX);
    }
    pub fn warp_fill_outliers() -> Interpolation {
        return Interpolation(Self::WARP_FILL_OUTLIERS);
    }
    pub fn warp_inverse_map() -> Interpolation {
        return Interpolation(Self::WARP_INVERSE_MAP);
    }

    pub (in crate) fn as_c_int(&self) -> c_int {
        return self.0;
    }
}
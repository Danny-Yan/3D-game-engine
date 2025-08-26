use std::collections::VecDeque;

pub trait Cycle {
    fn cycle(&mut self) -> anyhow::Result<()>;
}

impl<T> Cycle for VecDeque<T> {
    fn cycle(&mut self) -> anyhow::Result<()> {
        let front_item = self.pop_front().ok_or(wgpu::SurfaceError::OutOfMemory)?;
        self.push_back(front_item);
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RadWrapper(pub cgmath::Rad<f32>);

impl RadWrapper {
    pub fn new(value: f32) -> Self {
        RadWrapper(cgmath::Rad(value))
    }

    pub fn get_inside(&self) -> f32 {
        self.0.0
    }
}

impl Default for RadWrapper {
    fn default() -> Self {
        RadWrapper(cgmath::Rad(0.0))
    }
}

impl std::ops::AddAssign for RadWrapper {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TranslationValue(pub f32);

impl Default for TranslationValue {
    fn default() -> Self {
        TranslationValue(0.0)
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ScaleValue(pub f32);

impl Default for ScaleValue {
    fn default() -> Self {
        ScaleValue(1.0)
    }
}

impl std::ops::AddAssign for ScaleValue {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl std::ops::MulAssign for ScaleValue {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3Wrapper<T>(pub cgmath::Vector3<T>);

impl<T: Default> Default for Vector3Wrapper<T> {
    fn default() -> Self {
        Vector3Wrapper(cgmath::Vector3::new(
            T::default(),
            T::default(),
            T::default(),
        ))
    }
}

impl<T> Vector3Wrapper<T> {
    pub fn x(self) -> T {
        self.0.x
    }

    pub fn y(self) -> T {
        self.0.y
    }

    pub fn z(self) -> T {
        self.0.z
    }
}

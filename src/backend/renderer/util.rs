use egui_wgpu::wgpu;
use serde::de::value;
use std::collections::VecDeque;

pub trait Cycle<T> {
    fn cycle(&mut self) -> anyhow::Result<()>;
    fn front_mut_err(&mut self) -> anyhow::Result<&mut T>;
    fn front_err(&self) -> anyhow::Result<&T>;
}

impl<T> Cycle<T> for VecDeque<T> {
    fn cycle(&mut self) -> anyhow::Result<()> {
        let front_item = self.pop_front().ok_or(wgpu::SurfaceError::OutOfMemory)?;
        self.push_back(front_item);
        Ok(())
    }

    fn front_mut_err(&mut self) -> anyhow::Result<&mut T> {
        self.front_mut()
            .ok_or(wgpu::SurfaceError::OutOfMemory.into())
    }

    fn front_err(&self) -> anyhow::Result<&T> {
        self.front().ok_or(wgpu::SurfaceError::OutOfMemory.into())
    }
}

pub trait FloatWrapper {
    fn new(value: f32) -> Self;
    fn get_inside(&self) -> f32;
    fn get_mut(&mut self) -> &mut f32;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RadWrapper(pub cgmath::Rad<f32>);

impl FloatWrapper for RadWrapper {
    fn new(value: f32) -> Self {
        RadWrapper(cgmath::Rad(value))
    }

    fn get_inside(&self) -> f32 {
        self.0.0
    }

    fn get_mut(&mut self) -> &mut f32 {
        &mut self.0.0
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
pub struct CoordValue(pub f32);

impl Default for CoordValue {
    fn default() -> Self {
        CoordValue(0.0)
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

impl FloatWrapper for ScaleValue {
    fn new(value: f32) -> Self {
        ScaleValue(value)
    }

    fn get_inside(&self) -> f32 {
        self.0
    }
    fn get_mut(&mut self) -> &mut f32 {
        &mut self.0
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

macro_rules! vec_boxed {
    [ $( $x:expr ), * $(,)? ] => {
        vec![$( Box::new($x) ),* ]
    }
}
pub(crate) use vec_boxed;

macro_rules! vec_boxed_default {
    [ $( $x:ty ), * $(,)? ] => {
        vec_boxed![$( <$x>::default() ),* ]
    }
}
pub(crate) use vec_boxed_default;

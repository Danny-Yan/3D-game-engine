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
pub struct RadWrapper<T>(pub cgmath::Rad<T>);

impl<T> RadWrapper<T> {
    pub fn new(value: T) -> Self {
        RadWrapper(cgmath::Rad(value))
    }
}

impl Default for RadWrapper<f32> {
    fn default() -> Self {
        RadWrapper(cgmath::Rad(0.0))
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

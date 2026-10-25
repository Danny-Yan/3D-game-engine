use crate::backend::model::camera::{
    camera_editor::CameraControllerEditor, camera_fps::CameraControllerFPS,
};
use crate::backend::renderer::controller::CameraControllerTrait;
use crate::backend::renderer::util::{vec_boxed, vec_boxed_default};
// Camera Controller Array ---------------------------------------------------------------
// Append Camera Controllers here
//
// TODO: Probs do a more dynamic solution later on to search for interfaces and add them automatically
pub struct CameraControllerArray(pub Vec<Box<dyn CameraControllerTrait>>);
impl Default for CameraControllerArray {
    fn default() -> Self {
        Self(vec_boxed_default![
            CameraControllerEditor,
            CameraControllerFPS
        ])
    }
}

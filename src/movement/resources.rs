use bevy::prelude::*;

#[derive(Resource)]
pub struct View {
    pub yaw: f32,
    pub pitch: f32,
    pub mouse_locked: bool,
}

impl Default for View {
    fn default() -> Self {
        return View {
            yaw: 0.,
            pitch: 0.,
            mouse_locked: true,
        };
    }
}

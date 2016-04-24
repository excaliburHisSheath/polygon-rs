use math::*;
use super::AnchorId;

/// A camera in the scene.
#[derive(Debug, Clone)]
pub struct Camera
{
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,

    anchor: Option<AnchorId>,
}

impl Camera
{
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Camera {
        Camera {
            fov: fov,
            aspect: aspect,
            near: near,
            far: far,

            anchor: None,
        }
    }

    /// Calculates the projection matrix for the camera.
    ///
    /// The projection matrix is the matrix that converts from camera space to
    /// clip space. This effectively converts the viewing frustrum into a unit cube.
    pub fn projection_matrix(&self) -> Matrix4 {
        let height = 2.0 * self.near * (self.fov * 0.5).tan();
        let width = self.aspect * height;

        let mut projection = Matrix4::new();
        projection[0][0] = 2.0 * self.near / width;
        projection[1][1] = 2.0 * self.near / height;
        projection[2][2] = -(self.far + self.near) / (self.far - self.near);
        projection[2][3] = -2.0 * self.far * self.near / (self.far - self.near);
        projection[3][2] = -1.0;
        projection
    }

    pub fn anchor(&self) -> Option<AnchorId> {
        self.anchor
    }

    pub fn set_anchor(&mut self, anchor_id: AnchorId) {
        self.anchor = Some(anchor_id);
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            fov: PI / 3.0,
            aspect: 1.0,
            near: 0.001,
            far: 1_000.0,

            anchor: None,
        }
    }
}

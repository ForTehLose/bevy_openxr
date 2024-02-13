use glam::{UVec2, Vec2, Vec3A};
use wgpu::{Adapter, AdapterInfo, Device, Queue, TextureView};

use crate::prelude::*;

use crate::path::{InputComponent, UntypedActionPath};

pub trait EntryTrait {
    /// Return currently available extensions
    fn available_extensions(&self) -> Result<ExtensionSet>;
    /// Create an [Instance] with the enabled extensions.
    fn create_instance(&self, exts: ExtensionSet) -> Result<Instance>;
}

pub trait InstanceTrait {
    /// Returns the [Entry] used to create this.
    fn entry(&self) -> Entry;
    /// Returns an [ExtensionSet] listing all enabled extensions.
    fn enabled_extensions(&self) -> ExtensionSet;
    /// Creates a [Session] with the requested properties
    fn create_session(&self, info: SessionCreateInfo) -> Result<Session>;
}

pub trait SessionTrait {
    /// Returns the [Instance] used to create this.
    fn instance(&self) -> &Instance;
    /// Get render resources compatible with this session.
    fn get_render_resources(&self)
        -> Option<(Device, Queue, AdapterInfo, Adapter, wgpu::Instance)>;
    /// Returns the position of the headset.
    fn headset_location(&self) -> Result<Pose>;
    /// Request input modules with the specified bindings.
    fn create_input(&self, bindings: Bindings) -> Result<Input>;
    /// Wait until a frame is ready to render to.
    fn wait_frame(&self) -> Result<FrameData>;
    /// Begin rendering work for the frame.
    fn begin_frame(&self) -> Result<()>;
    /// Locate the views of each eye.
    fn locate_views(&self) -> Result<(View, View)>;
    /// Submits rendering work for this frame.
    fn end_frame(&self, data: FrameData) -> Result<()>;
    /// Gets the resolution of a single eye.
    fn resolution(&self) -> UVec2;
    /// Gets the texture format for the session.
    fn format(&self) -> wgpu::TextureFormat;
}

pub trait ViewTrait {
    /// Returns the [TextureView] used to render this view.
    fn texture_view(&self) -> Option<TextureView>;
    /// Returns the [Pose] representing the current position of this view.
    fn pose(&self) -> Pose;
    /// Returns the projection matrix for the current view.
    fn projection_matrix(&self, near: f32, far: f32) -> glam::Mat4;
    /// Gets the fov of the camera.
    fn fov(&self) -> Fov;
    /// Gets the resolution for this view.
    fn resolution(&self) -> UVec2;
    /// Gets the texture format for the view.
    fn format(&self) -> wgpu::TextureFormat;
}

pub trait InputTrait {
    /// Get the haptic action at the specified path.
    fn create_action_haptics(&self, path: UntypedActionPath) -> Result<Action<Haptic>>;
    /// Get the pose action at the specified path.
    fn create_action_pose(&self, path: UntypedActionPath) -> Result<Action<Pose>>;
    /// Get the float action at the specified path.
    fn create_action_float(&self, path: UntypedActionPath) -> Result<Action<f32>>;
    /// Get the boolean action at the specified path.
    fn create_action_bool(&self, path: UntypedActionPath) -> Result<Action<bool>>;
    /// Get the Vec2 action at the specified path.
    fn create_action_vec2(&self, path: UntypedActionPath) -> Result<Action<Vec2>>;
}

// This impl is moved outside of the trait to ensure that InputTrait stays object safe.
impl dyn InputTrait {
    /// Get the action at the specified path.
    pub fn create_action<P: InputComponent>(
        &self,
        path: ActionPath<P>,
    ) -> Result<Action<P::PathType>> {
        P::PathType::get(self, path.untyped())
    }
}

/// Represents input actions, such as bools, floats, and poses
pub trait ActionInputTrait<A> {
    fn get(&self) -> Result<A>;
}

/// Represents haptic actions.
pub trait HapticTrait {}

impl<T: InstanceTrait> EntryTrait for T {
    fn available_extensions(&self) -> Result<ExtensionSet> {
        self.entry().available_extensions()
    }

    fn create_instance(&self, exts: ExtensionSet) -> Result<Instance> {
        self.entry().create_instance(exts)
    }
}

impl<T: SessionTrait> InstanceTrait for T {
    fn entry(&self) -> Entry {
        self.instance().entry()
    }

    fn enabled_extensions(&self) -> ExtensionSet {
        self.instance().enabled_extensions()
    }

    fn create_session(&self, info: SessionCreateInfo) -> Result<Session> {
        self.instance().create_session(info)
    }
}

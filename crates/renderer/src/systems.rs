//! Systems: free functions that read and mutate the ECS [`World`].
//!
//! Each system takes the world (plus any frame inputs it needs) and runs one
//! query. They're plain functions called in order from the app's frame loop —
//! there's no scheduler yet, which keeps the data flow explicit.

use ferron_ecs::World;

use crate::gfx::RenderItem;
use crate::scene::{LocalTransform, MeshHandle, Spin};

/// Advance every entity that has a [`Spin`] by one frame of `dt` seconds.
pub fn spin(world: &World, dt: f32) {
    world
        .query::<(&mut LocalTransform, &Spin)>()
        .for_each(|_entity, (transform, spin)| spin.apply(transform, dt));
}

/// Build this frame's draw list from every entity that has both a
/// [`LocalTransform`] and a [`MeshHandle`].
///
/// This is the bridge from ECS data to the renderer: it produces plain
/// [`RenderItem`]s so the backend never has to know about the world.
pub fn extract_renderables(world: &World) -> Vec<RenderItem> {
    let mut items = Vec::new();
    world
        .query::<(&LocalTransform, &MeshHandle)>()
        .for_each(|_entity, (transform, mesh)| {
            items.push(RenderItem {
                model: transform.matrix(),
                mesh: *mesh,
            });
        });
    items
}

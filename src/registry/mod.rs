//! way-cooler registry.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

mod registry;
mod category;
mod client;

use self::registry::Registry;
pub use self::registry::{ReadHandle, WriteHandle};
pub use self::category::Category;

pub use self::client::{Client, Clients, Permissions};

lazy_static! {
    /// Static HashMap for the registry
    static ref REGISTRY: RwLock<Registry> = RwLock::new(Registry::new());
    static ref CLIENTS: RwLock<Clients> = RwLock::new(Clients::new());
}

#[allow(dead_code)]
pub fn clients_write<'a>() -> RwLockWriteGuard<'a, Clients> {
    CLIENTS.write().expect("Unable to write client mapping")
}

pub fn clients_read<'a>() -> RwLockReadGuard<'a, Clients> {
    CLIENTS.read().expect("Unable to read client mapping")
}

/// Initialize the registry and client mapping
pub fn init() {
    let mut registry = REGISTRY.write()
        .expect("Could not write to the registry");
    // Construct the layout category
    registry.add_category("windows".into())
        .expect("Could not add windows category");
    // Construct the programs category
    registry.add_category("programs".into())
        .expect("Could not add programs category");
    // Construct the mouse category
    registry.add_category("mouse".into())
        .expect("Could not add mouse category");
    // Construct the screen category
    registry.add_category("screen".into())
        .expect("Could not add screen category");
}

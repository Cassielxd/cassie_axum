// Forked from Gotham:
// https://github.com/gotham-rs/gotham/blob/bcbbf8923789e341b7a0e62c59909428ca4e22e2/gotham/src/state/mod.rs
// Copyright 2017 Gotham Project Developers. MIT license.

use log::trace;
use std::any::type_name;
use std::any::Any;
use std::any::TypeId;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct GothamState {
    data: BTreeMap<TypeId, Box<dyn Any>>,
}

impl GothamState {
    /// Puts a value into the `GothamState` storage. One value of each type is retained.
    /// Successive calls to `put` will overwrite the existing value of the same
    /// type.
    pub fn put<T: 'static>(&mut self, t: T) {
        let type_id = TypeId::of::<T>();
        trace!(" inserting record to state for type_id `{:?}`", type_id);
        self.data.insert(type_id, Box::new(t));
    }

    /// Determines if the current value exists in `GothamState` storage.
    pub fn has<T: 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.data.get(&type_id).is_some()
    }

    /// Tries to borrow a value from the `GothamState` storage.
    pub fn try_borrow<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        trace!(" borrowing state data for type_id `{:?}`", type_id);
        self.data.get(&type_id).and_then(|b| b.downcast_ref())
    }

    /// Borrows a value from the `GothamState` storage.
    pub fn borrow<T: 'static>(&self) -> &T {
        self.try_borrow().unwrap_or_else(|| missing::<T>())
    }

    /// Tries to mutably borrow a value from the `GothamState` storage.
    pub fn try_borrow_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        trace!(" mutably borrowing state data for type_id `{:?}`", type_id);
        self.data.get_mut(&type_id).and_then(|b| b.downcast_mut())
    }

    /// Mutably borrows a value from the `GothamState` storage.
    pub fn borrow_mut<T: 'static>(&mut self) -> &mut T {
        self.try_borrow_mut().unwrap_or_else(|| missing::<T>())
    }

    /// Tries to move a value out of the `GothamState` storage and return ownership.
    pub fn try_take<T: 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        trace!(
            " taking ownership from state data for type_id `{:?}`",
            type_id
        );
        self.data
            .remove(&type_id)
            .and_then(|b| b.downcast().ok())
            .map(|b| *b)
    }

    /// Moves a value out of the `GothamState` storage and returns ownership.
    ///
    /// # Panics
    ///
    /// If a value of type `T` is not present in `GothamState`.
    pub fn take<T: 'static>(&mut self) -> T {
        self.try_take().unwrap_or_else(|| missing::<T>())
    }
}

fn missing<T: 'static>() -> ! {
    panic!(
        "required type {} is not present in GothamState container",
        type_name::<T>()
    );
}

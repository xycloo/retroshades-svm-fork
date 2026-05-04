#![allow(dead_code)]

use soroban_env_common::Val;

use super::{RetroshadeEvent, RetroshadeExport, ZephyrAdapter};
use crate::{Host, HostError};

impl Host {
    pub(crate) fn record_retroshade(
        &self,
        target: Val,
        event_object: Val,
    ) -> Result<(), HostError> {
        let retroshade = RetroshadeEvent {
            contract_id: self.bytesobj_from_internal_contract_id()?,
            target: Some(target),
            event_object,
        };

        self.with_zephyr(|zephyr| Ok(zephyr.record(retroshade)))
    }

    pub(crate) fn with_zephyr<F, U>(&self, f: F) -> Result<U, HostError>
    where
        F: FnOnce(&mut ZephyrAdapter) -> Result<U, HostError>,
    {
        f(&mut *self.try_borrow_zephyr_mut()?)
    }

    pub fn get_retroshades(&self) -> Result<Vec<RetroshadeExport>, HostError> {
        self.with_zephyr(|zephyr| zephyr.externalize(&self))
    }
}

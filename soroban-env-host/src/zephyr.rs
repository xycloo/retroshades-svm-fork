#![allow(dead_code)]

use soroban_env_common::{
    xdr::{Error, Hash, ScVal},
    BytesObject, Val,
};

use crate::{Host, HostError};

mod retroshades;

/// The idea is that the guest can call zephyradapter functionality from within its execution providing,
/// at a low level, sequential sql instructions. These instructions are then extracted and passed to the zephyr worker
/// that processes then, asserts the safety by binding the table names and user ids to produce the final table.

#[derive(Clone, Debug, Default)]
pub(crate) struct RetroshadeEvent {
    contract_id: Option<BytesObject>,
    target: Option<Val>,
    event_object: Val,
}

#[derive(Clone, Debug)]
pub struct RetroshadeExport {
    pub contract_id: Hash,
    pub target: ScVal,
    pub event_object: ScVal,
}

impl RetroshadeEvent {
    fn to_xdr(&self, host: &Host) -> Result<RetroshadeExport, HostError> {
        let contract_id = host
            .hash_from_bytesobj_input("contract_id", self.contract_id.ok_or(Error::Unsupported)?)?;
        let target = host.from_host_val(self.target.ok_or(Error::Unsupported)?)?;
        let event_object = host.from_host_val(self.event_object)?;

        let export = RetroshadeExport {
            contract_id,
            target,
            event_object,
        };

        Ok(export)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ZephyrAdapter {
    pub(crate) retroshades: Vec<RetroshadeEvent>,
}

impl ZephyrAdapter {
    pub(crate) fn record(&mut self, retroshade: RetroshadeEvent) {
        self.retroshades.push(retroshade)
    }

    pub(crate) fn externalize(&self, host: &Host) -> Result<Vec<RetroshadeExport>, HostError> {
        self.retroshades
            .iter()
            .map(|retroshade| retroshade.to_xdr(host))
            .collect()
    }
}

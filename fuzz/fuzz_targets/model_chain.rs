//! Walks the SunSpec model chain of an arbitrary register map, then decodes each model
//! found. Whatever a device returns, discovery must end with a chain or an error,
//! never panic or loop, and a chain it accepts must be well formed.

#![no_main]

use std::future::{ready, Future};
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use libfuzzer_sys::fuzz_target;
use modbus_client::{ClientError, ExceptionCode};
use poller_actor::{base_candidates, discover_models, ReadRegisters, STANDARD_BASE_ADDRESSES};
use sunspec_parser::{decode_model_labeled, SUNSPEC_END_ID, SUNSPEC_ID};

/// A device's holding registers from `base` on. Reads outside them fail with "illegal
/// data address", as on a real device.
struct RegisterMap {
    base: u16,
    registers: Vec<u16>,
}

impl RegisterMap {
    fn get(&self, start: u16, count: u16) -> Option<&[u16]> {
        let offset = usize::from(start.checked_sub(self.base)?);
        self.registers.get(offset..offset + usize::from(count))
    }
}

impl ReadRegisters for RegisterMap {
    fn read_registers(
        &self,
        _unit_id: u8,
        start: u16,
        count: u16,
    ) -> impl Future<Output = Result<Vec<u16>, ClientError>> + Send {
        ready(
            self.get(start, count)
                .map(<[u16]>::to_vec)
                .ok_or(ClientError::Exception(ExceptionCode::IllegalDataAddress)),
        )
    }
}

/// Runs a future that never waits, as every read of a register map completes at once.
fn run<F: Future>(future: F) -> F::Output {
    match pin!(future).poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(output) => output,
        Poll::Pending => unreachable!("register map reads never wait"),
    }
}

fuzz_target!(|data: &[u8]| {
    // Byte 0: bits 0-1 pick where the map starts (the configured base, or one of the
    // standard bases), bit 2 starts it with the "SunS" marker. Bytes 1-2: the
    // configured base. The rest: the registers.
    let Some(([flags, high, low], rest)) = data.split_first_chunk::<3>() else {
        return;
    };
    let configured = u16::from_be_bytes([*high, *low]);
    let base = match flags & 0b11 {
        0 => configured,
        n => STANDARD_BASE_ADDRESSES[usize::from(n - 1)],
    };
    let mut registers = Vec::new();
    if flags & 0b100 != 0 {
        registers.extend(SUNSPEC_ID);
    }
    registers.extend(
        rest.as_chunks::<2>()
            .0
            .iter()
            .map(|pair| u16::from_be_bytes(*pair)),
    );
    let map = RegisterMap { base, registers };

    let bases = base_candidates(configured);
    let Ok((found, models)) = run(discover_models(&map, 1, &bases)) else {
        return;
    };

    assert!(bases.contains(&found), "the map is at a candidate base");
    assert_eq!(
        map.get(found, 2),
        Some(&SUNSPEC_ID[..]),
        "the map starts with SunS"
    );
    assert!(models.len() <= 256, "at most 256 models");
    // Discovery checked these additions for overflow.
    let mut next = found + 2;
    for model in &models {
        assert_eq!(model.start, next, "models follow each other");
        assert_ne!(model.id, SUNSPEC_END_ID);
        next = model.start + model.length;
        // Polling reads the whole model; decode it when the map holds all of it.
        if let Some(registers) = map.get(model.start, model.length) {
            decode_model_labeled(model.id, registers);
        }
    }
    assert_eq!(
        map.get(next, 2).map(|header| header[0]),
        Some(SUNSPEC_END_ID),
        "the chain ends with the end marker"
    );
});

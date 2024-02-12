//! No-op program

#![no_std]

extern crate alloc;

use alloc::{string::ToString, vec::Vec};

use entropy_programs_core::{bindgen::Error, bindgen::*, export_program, prelude::*};

// TODO confirm this isn't an issue for audit
register_custom_getrandom!(always_fail);

pub struct ProgramAlwaysFails;

impl Program for ProgramAlwaysFails {
    fn evaluate(
        _signature_request: SignatureRequest,
        _config: Option<Vec<u8>>,
    ) -> Result<(), Error> {
        Err(Error::Evaluation("This program always fails".to_string()))
    }

    /// Since we don't use a custom hash function, we can just return `None` here.
    fn custom_hash(_data: Vec<u8>) -> Option<Vec<u8>> {
        None
    }
}

export_program!(ProgramAlwaysFails);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_fail() {
        let signature_request = SignatureRequest {
            message: Vec::new(),
            auxilary_data: None,
        };

        assert!(ProgramAlwaysFails::evaluate(signature_request, None).is_err());
    }
}

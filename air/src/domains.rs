use starknet_crypto::Felt;
use swiftness_field::SimpleField;

const FIELD_GENERATOR: Felt = Felt::from_hex_unchecked("0x3");

const STARK_PRIME_MINUS_ONE: Felt =
    Felt::from_hex_unchecked("800000000000011000000000000000000000000000000000000000000000000");

#[derive(Debug, PartialEq)]
pub struct StarkDomains<F: SimpleField> {
    // Log2 of the evaluation domain size.
    pub log_eval_domain_size: F,
    // The evaluation domain size.
    pub eval_domain_size: F,
    // The generator of the evaluation domain (a primitive root of unity of order eval_domain_size).
    pub eval_generator: F,
    // Log2 of the trace domain size.
    pub log_trace_domain_size: F,
    // The trace domain size.
    pub trace_domain_size: F,
    // The generator of the trace domain (a primitive root of unity of order trace_domain_size).
    pub trace_generator: F,
}

impl<F: SimpleField> StarkDomains<F> {
    pub fn new(log_trace_domain_size: F, log_n_cosets: F) -> Self {
        let log_eval_domain_size = log_trace_domain_size.clone() + log_n_cosets.clone();
        let eval_domain_size = F::two().powers_felt(&log_eval_domain_size);
        let trace_domain_size = F::two().powers_felt(&log_trace_domain_size);
        Self {
            log_eval_domain_size: log_trace_domain_size.clone() + log_n_cosets,
            eval_domain_size: F::two().powers_felt(&log_eval_domain_size),
            eval_generator: F::from_stark_felt(FIELD_GENERATOR).powers_felt(
                &F::from_stark_felt(STARK_PRIME_MINUS_ONE).field_div(&eval_domain_size),
            ),
            trace_generator: F::from_stark_felt(FIELD_GENERATOR).powers_felt(
                &F::from_stark_felt(STARK_PRIME_MINUS_ONE).field_div(&trace_domain_size),
            ),
            trace_domain_size,
            log_trace_domain_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use starknet_crypto::Felt;
    use swiftness_field::{Fp, SimpleField};

    use crate::domains::StarkDomains;

    #[test]
    fn test_domain_creation() {
        let log_trace_domain_size = Fp::from_stark_felt(Felt::from(0x12));
        let log_n_cosets = Fp::from_stark_felt(Felt::from(0x4));

        assert!(
            StarkDomains::new(log_trace_domain_size, log_n_cosets)
                == StarkDomains {
                    log_eval_domain_size: Fp::from_stark_felt(Felt::from_hex_unchecked("0x16")),
                    eval_domain_size: Fp::from_stark_felt(Felt::from_hex_unchecked("0x400000")),
                    eval_generator: Fp::from_stark_felt(Felt::from_hex_unchecked(
                        "0x3e4383531eeac7c9822fb108d24a344d841544dd6482f17ead331453e3a2f4b"
                    )),
                    log_trace_domain_size: Fp::from_stark_felt(Felt::from_hex_unchecked("0x12")),
                    trace_domain_size: Fp::from_stark_felt(Felt::from_hex_unchecked("0x40000")),
                    trace_generator: Fp::from_stark_felt(Felt::from_hex_unchecked(
                        "0x4768803ef85256034f67453635f87997ff61841e411ee63ce7b0a8b9745a046"
                    )),
                },
        )
    }
}

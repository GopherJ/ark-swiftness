pub mod autogenerated;
pub mod global_values;

use crate::{
    diluted::get_diluted_product,
    layout::stark_curve,
    periodic_columns::{
        eval_ecdsa_x, eval_ecdsa_y, eval_pedersen_x, eval_pedersen_y,
        eval_poseidon_poseidon_full_round_key0, eval_poseidon_poseidon_full_round_key1,
        eval_poseidon_poseidon_full_round_key2, eval_poseidon_poseidon_partial_round_key0,
        eval_poseidon_poseidon_partial_round_key1,
    },
    public_memory::{PublicInput, INITIAL_PC, MAX_ADDRESS, MAX_LOG_N_STEPS},
};
use alloc::vec;
use alloc::vec::Vec;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::{fields::{fp::FpVar, FieldOpsBounds, FieldVar}, prelude::Boolean};
use global_values::{CurveConfig, EcPoint, EcdsaSigConfig, GlobalValues, InteractionElements};
use starknet_crypto::Felt;
use swiftness_commitment::table::{commit::table_commit, decommit::table_decommit};
use swiftness_field::SimpleField;
use swiftness_hash::{pedersen::PedersenHash, poseidon::PoseidonHash};
use swiftness_transcript::ensure;

use super::{CompositionPolyEvalError, LayoutTrait, PublicInputError};

pub const BITWISE_RATIO: usize = 64;
pub const BITWISE_ROW_RATIO: usize = 1024;
pub const BITWISE_TOTAL_N_BITS: usize = 251;
pub const CPU_COMPONENT_HEIGHT: usize = 16;
pub const CPU_COMPONENT_STEP: usize = 1;
pub const DILUTED_N_BITS: usize = 16;
pub const DILUTED_SPACING: usize = 4;
pub const EC_OP_BUILTIN_RATIO: usize = 1024;
pub const EC_OP_BUILTIN_ROW_RATIO: usize = 16384;
pub const EC_OP_N_BITS: usize = 252;
pub const EC_OP_SCALAR_HEIGHT: usize = 256;
pub const ECDSA_BUILTIN_RATIO: usize = 2048;
pub const ECDSA_BUILTIN_REPETITIONS: usize = 1;
pub const ECDSA_BUILTIN_ROW_RATIO: usize = 32768;
pub const ECDSA_ELEMENT_BITS: usize = 251;
pub const ECDSA_ELEMENT_HEIGHT: usize = 256;
pub const HAS_BITWISE_BUILTIN: usize = 1;
pub const HAS_DILUTED_POOL: usize = 1;
pub const HAS_EC_OP_BUILTIN: usize = 1;
pub const HAS_ECDSA_BUILTIN: usize = 1;
pub const HAS_KECCAK_BUILTIN: usize = 0;
pub const HAS_OUTPUT_BUILTIN: usize = 1;
pub const HAS_PEDERSEN_BUILTIN: usize = 1;
pub const HAS_POSEIDON_BUILTIN: usize = 1;
pub const HAS_RANGE_CHECK_BUILTIN: usize = 1;
pub const HAS_RANGE_CHECK96_BUILTIN: usize = 0;
pub const IS_DYNAMIC_AIR: usize = 0;
pub const LAYOUT_CODE: Felt = Felt::from_hex_unchecked("0x737461726b6e6574");
pub const LOG_CPU_COMPONENT_HEIGHT: usize = 4;
pub const N_DYNAMIC_PARAMS: usize = 0;
pub const PEDERSEN_BUILTIN_RATIO: usize = 32;
pub const PEDERSEN_BUILTIN_REPETITIONS: usize = 1;
pub const PEDERSEN_BUILTIN_ROW_RATIO: usize = 512;
pub const POSEIDON_M: usize = 3;
pub const POSEIDON_RATIO: usize = 32;
pub const POSEIDON_ROUNDS_FULL: usize = 8;
pub const POSEIDON_ROUNDS_PARTIAL: usize = 83;
pub const POSEIDON_ROW_RATIO: usize = 512;
pub const PUBLIC_MEMORY_STEP: usize = 8;
pub const RANGE_CHECK_BUILTIN_RATIO: usize = 16;
pub const RANGE_CHECK_BUILTIN_ROW_RATIO: usize = 256;
pub const RANGE_CHECK_N_PARTS: usize = 8;

pub mod segments {
    pub const BITWISE: usize = 6;
    pub const EC_OP: usize = 7;
    pub const ECDSA: usize = 5;
    pub const N_SEGMENTS: usize = 9;
    pub const PEDERSEN: usize = 3;
    pub const POSEIDON: usize = 8;
    pub const RANGE_CHECK: usize = 4;
}

pub mod builtins {
    use starknet_crypto::Felt;

    pub const OUTPUT: Felt = Felt::from_hex_unchecked("0x6F7574707574");
    pub const PEDERSEN: Felt = Felt::from_hex_unchecked("0x706564657273656E");
    pub const RANGE_CHECK: Felt = Felt::from_hex_unchecked("0x72616E67655F636865636B");
    pub const ECDSA: Felt = Felt::from_hex_unchecked("0x6563647361");
    pub const BITWISE: Felt = Felt::from_hex_unchecked("0x62697477697365");
    pub const EC_OP: Felt = Felt::from_hex_unchecked("0x65635F6F70");
    pub const POSEIDON: Felt = Felt::from_hex_unchecked("0x706F736569646F6E");
}

// Pedersen builtin
pub const SHIFT_POINT_X: Felt =
    Felt::from_hex_unchecked("0x49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804");
pub const SHIFT_POINT_Y: Felt =
    Felt::from_hex_unchecked("0x3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a");

pub const BUILTINS: [Felt; 7] = [
    builtins::OUTPUT,
    builtins::PEDERSEN,
    builtins::RANGE_CHECK,
    builtins::ECDSA,
    builtins::BITWISE,
    builtins::EC_OP,
    builtins::POSEIDON,
];

pub struct Layout {}

impl<F: SimpleField + PoseidonHash> LayoutTrait<F> for Layout {
    const CONSTRAINT_DEGREE: usize = 2;
    const MASK_SIZE: usize = 271;
    const N_CONSTRAINTS: usize = 198;
    const NUM_COLUMNS_FIRST: usize = 9;
    const NUM_COLUMNS_SECOND: usize = 1;
    type InteractionElements = InteractionElements<F>;

    fn eval_composition_polynomial(
        interaction_elements: &Self::InteractionElements,
        public_input: &PublicInput<F>,
        mask_values: &[F],
        constraint_coefficients: &[F],
        point: &F,
        trace_domain_size: &F,
        trace_generator: &F,
    ) -> Result<F, CompositionPolyEvalError> {
        let memory_z = interaction_elements.memory_multi_column_perm_perm_interaction_elm.clone();
        let memory_alpha = interaction_elements.memory_multi_column_perm_hash_interaction_elm0.clone();

        // Public memory
        let public_memory_column_size = trace_domain_size
            .field_div(&F::from_constant(PUBLIC_MEMORY_STEP as u128));
        // assert!(public_memory_column_size < u128::MAX.into());
        public_memory_column_size.assert_lt(&F::from_constant(u128::MAX));
        let public_memory_prod_ratio = public_input.get_public_memory_product_ratio(
            memory_z.clone(),
            memory_alpha.clone(),
            public_memory_column_size,
        );

        // Diluted
        let diluted_z = interaction_elements.diluted_check_interaction_z.clone();
        let diluted_alpha = interaction_elements.diluted_check_interaction_alpha.clone();
        let diluted_prod = get_diluted_product(
            F::from_constant(DILUTED_N_BITS  as u128),
            F::from_constant(DILUTED_SPACING as u128),
            diluted_z.clone(),
            diluted_alpha.clone(),
        );

        // Periodic columns
        let n_steps = F::two().powers_felt(&public_input.log_n_steps);
        let n_pedersen_hash_copies = n_steps.field_div(&
            F::from_constant((PEDERSEN_BUILTIN_RATIO * PEDERSEN_BUILTIN_REPETITIONS) as u128),
        );
        // assert!(n_pedersen_hash_copies < u128::MAX.into());
        n_pedersen_hash_copies.assert_lt(&F::from_constant(u128::MAX));
        let pedersen_point = point.powers_felt(&n_pedersen_hash_copies);
        let pedersen_points_x = eval_pedersen_x(pedersen_point.clone());
        let pedersen_points_y = eval_pedersen_y(pedersen_point);

        let n_ecdsa_signature_copies = n_steps.field_div(&
            F::from_constant((ECDSA_BUILTIN_RATIO * ECDSA_BUILTIN_REPETITIONS) as u128),
        );
        // assert!(n_ecdsa_signature_copies < u128::MAX.into());
        n_ecdsa_signature_copies.assert_lt(&F::from_constant(u128::MAX));
        let ecdsa_point = point.powers_felt(&n_ecdsa_signature_copies);
        let ecdsa_generator_points_x = eval_ecdsa_x(ecdsa_point.clone());
        let ecdsa_generator_points_y = eval_ecdsa_y(ecdsa_point);

        let n_poseidon_copies =
            n_steps.field_div(&F::from_constant(POSEIDON_RATIO as u128));
        // assert!(n_pedersen_hash_copies < u128::MAX.into());
        n_pedersen_hash_copies.assert_lt(&F::from_constant(u128::MAX));
        let poseidon_point = point.powers_felt(&n_poseidon_copies);
        let poseidon_poseidon_full_round_key0 =
            eval_poseidon_poseidon_full_round_key0(poseidon_point.clone());
        let poseidon_poseidon_full_round_key1 =
            eval_poseidon_poseidon_full_round_key1(poseidon_point.clone());
        let poseidon_poseidon_full_round_key2 =
            eval_poseidon_poseidon_full_round_key2(poseidon_point.clone());
        let poseidon_poseidon_partial_round_key0 =
            eval_poseidon_poseidon_partial_round_key0(poseidon_point.clone());
        let poseidon_poseidon_partial_round_key1 =
            eval_poseidon_poseidon_partial_round_key1(poseidon_point);

        let global_values = GlobalValues {
            trace_length: trace_domain_size.clone(),
            initial_pc: public_input
                .segments
                .get(crate::layout::segments::PROGRAM)
                .ok_or(CompositionPolyEvalError::SegmentMissing {
                    segment: crate::layout::segments::PROGRAM,
                })?
                .begin_addr.clone(),
            final_pc: public_input
                .segments
                .get(crate::layout::segments::PROGRAM)
                .ok_or(CompositionPolyEvalError::SegmentMissing {
                    segment: crate::layout::segments::PROGRAM,
                })?
                .stop_ptr.clone(),
            initial_ap: public_input
                .segments
                .get(crate::layout::segments::EXECUTION)
                .ok_or(CompositionPolyEvalError::SegmentMissing {
                    segment: crate::layout::segments::EXECUTION,
                })?
                .begin_addr.clone(),
            final_ap: public_input
                .segments
                .get(crate::layout::segments::EXECUTION)
                .ok_or(CompositionPolyEvalError::SegmentMissing {
                    segment: crate::layout::segments::EXECUTION,
                })?
                .stop_ptr.clone(),
            initial_pedersen_addr: public_input
                .segments
                .get(segments::PEDERSEN)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::PEDERSEN })?
                .begin_addr.clone(),
            initial_range_check_addr: public_input
                .segments
                .get(segments::RANGE_CHECK)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::RANGE_CHECK })?
                .begin_addr.clone(),
            initial_ecdsa_addr: public_input
                .segments
                .get(segments::ECDSA)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::ECDSA })?
                .begin_addr.clone(),
            initial_bitwise_addr: public_input
                .segments
                .get(segments::BITWISE)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::BITWISE })?
                .begin_addr.clone(),
            initial_ec_op_addr: public_input
                .segments
                .get(segments::EC_OP)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::EC_OP })?
                .begin_addr.clone(),
            initial_poseidon_addr: public_input
                .segments
                .get(segments::POSEIDON)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::POSEIDON })?
                .begin_addr.clone(),
            range_check_min: public_input.range_check_min.clone(),
            range_check_max: public_input.range_check_max.clone(),
            offset_size: F::from_stark_felt(Felt::from(0x10000)),     // 2**16
            half_offset_size: F::from_stark_felt(Felt::from(0x8000)), // 2**15
            pedersen_shift_point: EcPoint { x: F::from_stark_felt(SHIFT_POINT_X), y: F::from_stark_felt(SHIFT_POINT_Y) },
            ecdsa_sig_config: EcdsaSigConfig {
                alpha: F::from_stark_felt(stark_curve::ALPHA),
                beta:  F::from_stark_felt(stark_curve::BETA),
                shift_point: EcPoint { x: F::from_stark_felt(SHIFT_POINT_X), y: F::from_stark_felt(SHIFT_POINT_Y) },
            },
            ec_op_curve_config: CurveConfig { alpha: F::from_stark_felt(stark_curve::ALPHA), beta: F::from_stark_felt(stark_curve::BETA) },
            pedersen_points_x,
            pedersen_points_y,
            ecdsa_generator_points_x,
            ecdsa_generator_points_y,
            poseidon_poseidon_full_round_key0,
            poseidon_poseidon_full_round_key1,
            poseidon_poseidon_full_round_key2,
            poseidon_poseidon_partial_round_key0,
            poseidon_poseidon_partial_round_key1,
            memory_multi_column_perm_perm_interaction_elm: memory_z,
            memory_multi_column_perm_hash_interaction_elm0: memory_alpha,
            range_check16_perm_interaction_elm: interaction_elements
                .range_check16_perm_interaction_elm.clone(),
            diluted_check_permutation_interaction_elm: interaction_elements
                .diluted_check_permutation_interaction_elm.clone(),
            diluted_check_interaction_z: diluted_z,
            diluted_check_interaction_alpha: diluted_alpha,
            memory_multi_column_perm_perm_public_memory_prod: public_memory_prod_ratio,
            range_check16_perm_public_memory_prod: F::one(),
            diluted_check_first_elm: F::zero(),
            diluted_check_permutation_public_memory_prod: F::one(),
            diluted_check_final_cum_val: diluted_prod,
        };

        Ok(autogenerated::eval_composition_polynomial_inner(
            mask_values,
            constraint_coefficients,
            point,
            trace_generator,
            &global_values,
        ))
    }
    fn eval_oods_polynomial(
        column_values: &[F],
        oods_values: &[F],
        constraint_coefficients: &[F],
        point: &F,
        oods_point: &F,
        trace_generator: &F,
    ) -> F {
        autogenerated::eval_oods_polynomial_inner::<F, Self>(
            column_values,
            oods_values,
            constraint_coefficients,
            point,
            oods_point,
            trace_generator,
        )
    }
    fn traces_commit(
        transcript: &mut swiftness_transcript::transcript::Transcript<F>,
        unsent_commitment: &crate::trace::UnsentCommitment<F>,
        config: crate::trace::config::Config<F>,
    ) -> crate::trace::Commitment<Self::InteractionElements, F> {
        // Read original commitment.
        let original_commitment =
            table_commit(transcript, unsent_commitment.original.clone(), config.original);

        // Generate interaction elements for the first interaction.
        let interaction_elements = Self::InteractionElements::new(transcript);

        // Read interaction commitment.
        let interaction_commitment =
            table_commit(transcript, unsent_commitment.interaction.clone(), config.interaction);

        crate::trace::Commitment {
            original: original_commitment,
            interaction_elements,
            interaction: interaction_commitment,
        }
    }
    fn traces_decommit(
        queries: &[F],
        commitment: crate::trace::Commitment<Self::InteractionElements, F>,
        decommitment: crate::trace::Decommitment<F>,
        witness: crate::trace::Witness<F>,
    ) -> Result<(), crate::trace::decommit::Error<F>> {
        Ok(table_decommit(commitment.original, queries, decommitment.original, witness.original)
            .and(table_decommit(
                commitment.interaction,
                queries,
                decommitment.interaction,
                witness.interaction,
            ))?)
    }
    fn validate_public_input(
        public_input: &PublicInput<F>,
        stark_domains: &crate::domains::StarkDomains<F>,
    ) -> Result<(), PublicInputError> {
        // ensure!(public_input.log_n_steps < MAX_LOG_N_STEPS, PublicInputError::MaxSteps);
        public_input.log_n_steps .assert_lt(&F::from_stark_felt(MAX_LOG_N_STEPS));

        let n_steps = F::two().powers_felt(&public_input.log_n_steps);
        let trace_length = stark_domains.trace_domain_size.clone();
        // TODO: enable check
        // ensure!(
        //     n_steps * Felt::from(CPU_COMPONENT_HEIGHT) * Felt::from(CPU_COMPONENT_STEP)
        //         == trace_length,
        //     PublicInputError::TraceLengthInvalid
        // );
        //
        // ensure!(F::zero() <= public_input.range_check_min, PublicInputError::RangeCheckInvalid);
        // ensure!(
        //     public_input.range_check_min < public_input.range_check_max,
        //     PublicInputError::RangeCheckInvalid
        // );
        // ensure!(
        //     public_input.range_check_max <= MAX_RANGE_CHECK,
        //     PublicInputError::RangeCheckInvalid
        // );
        //
        // ensure!(public_input.layout == LAYOUT_CODE, PublicInputError::LayoutCodeInvalid);

        let output_uses = public_input
            .segments
            .get(crate::layout::segments::OUTPUT)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::OUTPUT })?
            .stop_ptr.clone()
            - &public_input
                .segments
                .get(crate::layout::segments::OUTPUT)
                .ok_or(PublicInputError::SegmentMissing {
                    segment: crate::layout::segments::OUTPUT,
                })?
                .begin_addr;
        // ensure!(output_uses < u128::MAX.into(), PublicInputError::UsesInvalid);
        output_uses.assert_lt(&F::from_constant(u128::MAX));

        let pedersen_copies = trace_length
            .field_div(&F::from_constant(PEDERSEN_BUILTIN_ROW_RATIO as u128));
        let pedersen_uses = (public_input
            .segments
            .get(segments::PEDERSEN)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::OUTPUT })?
            .stop_ptr.clone()
            - &public_input
                .segments
                .get(segments::PEDERSEN)
                .ok_or(PublicInputError::SegmentMissing {
                    segment: crate::layout::segments::OUTPUT,
                })?
                .begin_addr)
            .field_div(&F::three());
        // ensure!(pedersen_uses < pedersen_copies, PublicInputError::UsesInvalid);
        pedersen_uses.assert_lt(&pedersen_copies);

        let range_check_copies = trace_length.field_div(&
            F::from_constant(RANGE_CHECK_BUILTIN_ROW_RATIO as u128),
        );
        let range_check_uses = public_input
            .segments
            .get(segments::RANGE_CHECK)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::OUTPUT })?
            .stop_ptr.clone()
            - &public_input
                .segments
                .get(segments::RANGE_CHECK)
                .ok_or(PublicInputError::SegmentMissing {
                    segment: crate::layout::segments::OUTPUT,
                })?
                .begin_addr;
        // ensure!(range_check_uses < range_check_copies, PublicInputError::UsesInvalid);
        range_check_uses.assert_lt(&range_check_copies);

        let bitwise_copies = trace_length
            .field_div(&F::from_constant(BITWISE_ROW_RATIO as u128));
        let bitwise_uses = (public_input
            .segments
            .get(segments::BITWISE)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::OUTPUT })?
            .stop_ptr.clone()
            - &public_input
                .segments
                .get(segments::BITWISE)
                .ok_or(PublicInputError::SegmentMissing {
                    segment: crate::layout::segments::OUTPUT,
                })?
                .begin_addr)
            .field_div(&F::from_constant(0x5 as u128));
        // ensure!(bitwise_uses < bitwise_copies, PublicInputError::UsesInvalid);
        bitwise_uses.assert_lt(&bitwise_copies);
        Ok(())
    }

    fn verify_public_input<P: SWCurveConfig>(public_input: &PublicInput<F>) -> Result<(F, F), PublicInputError>
    where
        F: PedersenHash<P>,
        P::BaseField: PrimeField + SimpleField,
        <P::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<P::BaseField>:
            FieldVar<P::BaseField, <P::BaseField as Field>::BasePrimeField> + SimpleField,
        for<'a> &'a FpVar<P::BaseField>: FieldOpsBounds<'a, P::BaseField, FpVar<P::BaseField>>,
        <FpVar<P::BaseField> as SimpleField>::BooleanType:
            From<Boolean<<P::BaseField as Field>::BasePrimeField>>
    {
        let public_segments = &public_input.segments;

        let initial_pc = public_segments
            .get(crate::layout::segments::PROGRAM)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .begin_addr.clone();
        let final_pc = public_segments
            .get(crate::layout::segments::PROGRAM)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .stop_ptr.clone();
        let initial_ap = public_segments
            .get(crate::layout::segments::EXECUTION)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .begin_addr.clone();
        let initial_fp = initial_ap.clone();
        let final_ap = public_segments
            .get(crate::layout::segments::EXECUTION)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .stop_ptr.clone();
        let output_start = public_segments
            .get(crate::layout::segments::OUTPUT)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .begin_addr.clone();
        let output_stop = public_segments
            .get(crate::layout::segments::OUTPUT)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .stop_ptr.clone();

        // ensure!(initial_ap < MAX_ADDRESS, PublicInputError::MaxSteps);
        initial_ap.assert_lt(&F::from_stark_felt(MAX_ADDRESS));
        // ensure!(final_ap < MAX_ADDRESS, PublicInputError::MaxSteps);
        final_ap.assert_lt(&F::from_stark_felt(MAX_ADDRESS));

        // TODO support more pages?
        ensure!(public_input.continuous_page_headers.is_empty(), PublicInputError::MaxSteps);

        let memory = &public_input
            .main_page
            .iter()
            .flat_map(|v| vec![v.address.clone(), v.value.clone()])
            .collect::<Vec<F>>();

        // ensure!(initial_pc == INITIAL_PC, PublicInputError::MaxSteps);
        initial_pc.assert_equal(&F::from_stark_felt(INITIAL_PC));
        // ensure!(final_pc == INITIAL_PC + 4, PublicInputError::MaxSteps);
        final_pc.assert_equal(&F::from_stark_felt(INITIAL_PC + 4));

        let program_end_pc = initial_fp - F::two();

        let program: Vec<&F> = memory
            .iter()
            .skip(initial_pc.into_constant().try_into().unwrap())
            .step_by(2)
            .take((program_end_pc - F::one()).into_constant().try_into().unwrap())
            .collect();

        let hash = program.iter().fold(F::zero(), |acc, &e| PedersenHash::<P>::hash(acc.clone(), e.clone()));
        let program_hash = PedersenHash::<P>::hash(hash, F::from_constant(program.len() as u128));

        let output_len: usize = (output_stop - output_start).into_constant().try_into().unwrap();
        let output = &memory[memory.len() - output_len * 2..];
        let hash =
            output.iter().skip(1).step_by(2).fold(F::zero(), |acc, e| PedersenHash::<P>::hash(acc.clone(), e.clone()));
        let output_hash = PedersenHash::<P>::hash(hash, F::from_constant(output_len as u128));

        Ok((program_hash, output_hash))
    }
}

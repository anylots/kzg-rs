#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod consts;
pub mod dtypes;
pub mod enums;
pub mod kzg_proof;
pub mod trusted_setup;
pub mod pairings;

pub use consts::*;
pub use dtypes::*;
pub use kzg_proof::KzgProof;
pub use trusted_setup::*;
pub use pairings::pairings_verify;

pub use enums::KzgError;

#[cfg(test)]
mod test_files {
    // Tests
    pub const VERIFY_KZG_PROOF_TESTS: [(&str, &str); 122] = [
        (
            "verify_kzg_proof_case_correct_proof_02e696ada7d4631d",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_02e696ada7d4631d/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_05c1f3685f3393f0",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_05c1f3685f3393f0/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_08f9e2f1cb3d39db",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_08f9e2f1cb3d39db/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_0cf79b17cb5f4ea2",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_0cf79b17cb5f4ea2/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_177b58dc7a46b08f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_177b58dc7a46b08f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_1ce8e4f69d5df899",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_1ce8e4f69d5df899/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_26b753dec0560daa",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_26b753dec0560daa/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_2b76dc9e3abf42f3",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_2b76dc9e3abf42f3/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_31ebd010e6098750",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_31ebd010e6098750/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_3208425794224c3f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_3208425794224c3f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_36817bfd67de97a8",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_36817bfd67de97a8/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_392169c16a2e5ef6",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_392169c16a2e5ef6/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_395cf6d697d1a743",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_395cf6d697d1a743/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_3ac8dc31e9aa6a70",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_3ac8dc31e9aa6a70/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_3c1e8b38219e3e12",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_3c1e8b38219e3e12/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_3c87ec986c2656c2",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_3c87ec986c2656c2/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_3cd183d0bab85fb7",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_3cd183d0bab85fb7/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_420f2a187ce77035",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_420f2a187ce77035/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_444b73ff54a19b44",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_444b73ff54a19b44/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_53a9bdf4f75196da",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_53a9bdf4f75196da/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_585454b31673dd62",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_585454b31673dd62/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_7db4f140a955dd1a",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_7db4f140a955dd1a/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_83e53423a2dd93fe",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_83e53423a2dd93fe/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_9b24f8997145435c",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_9b24f8997145435c/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_9b754afb690c47e1",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_9b754afb690c47e1/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_a0be66af9a97ea52",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_a0be66af9a97ea52/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_af669445747d2585",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_af669445747d2585/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_af8b75f664ed7d43",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_af8b75f664ed7d43/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_b6cb6698327d9835",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_b6cb6698327d9835/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_b6ec3736f9ff2c62",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_b6ec3736f9ff2c62/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_becf2e1641bbd4e6",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_becf2e1641bbd4e6/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_c3d4322ec17fe7cd",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_c3d4322ec17fe7cd/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_c5e1490d672d026d",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_c5e1490d672d026d/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_cae5d3491190b777",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_cae5d3491190b777/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_d0992bc0387790a4",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_d0992bc0387790a4/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_d736268229bd87ec",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_d736268229bd87ec/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_e68d7111a2364a49",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_e68d7111a2364a49/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_ed6b180ec759bcf6",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_ed6b180ec759bcf6/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_f0ed3dc11cdeb130",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_f0ed3dc11cdeb130/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_f47eb9fc139f6bfd",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_f47eb9fc139f6bfd/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_f7f44e1e864aa967",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_f7f44e1e864aa967/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_ffa6e97b97146517",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_ffa6e97b97146517/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_05c1f3685f3393f0",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_05c1f3685f3393f0/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_177b58dc7a46b08f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_177b58dc7a46b08f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_2b76dc9e3abf42f3",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_2b76dc9e3abf42f3/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_395cf6d697d1a743",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_395cf6d697d1a743/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_585454b31673dd62",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_585454b31673dd62/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_a0be66af9a97ea52",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly_a0be66af9a97ea52/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_02e696ada7d4631d",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_02e696ada7d4631d/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_0cf79b17cb5f4ea2",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_0cf79b17cb5f4ea2/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_3208425794224c3f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_3208425794224c3f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_3ac8dc31e9aa6a70",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_3ac8dc31e9aa6a70/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_c3d4322ec17fe7cd",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_c3d4322ec17fe7cd/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_ffa6e97b97146517",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly_ffa6e97b97146517/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_02e696ada7d4631d",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_02e696ada7d4631d/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_05c1f3685f3393f0",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_05c1f3685f3393f0/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_08f9e2f1cb3d39db",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_08f9e2f1cb3d39db/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_0cf79b17cb5f4ea2",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_0cf79b17cb5f4ea2/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_177b58dc7a46b08f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_177b58dc7a46b08f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_1ce8e4f69d5df899",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_1ce8e4f69d5df899/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_26b753dec0560daa",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_26b753dec0560daa/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_2b76dc9e3abf42f3",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_2b76dc9e3abf42f3/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_31ebd010e6098750",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_31ebd010e6098750/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_3208425794224c3f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_3208425794224c3f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_36817bfd67de97a8",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_36817bfd67de97a8/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_392169c16a2e5ef6",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_392169c16a2e5ef6/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_395cf6d697d1a743",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_395cf6d697d1a743/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_3ac8dc31e9aa6a70",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_3ac8dc31e9aa6a70/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_3c1e8b38219e3e12",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_3c1e8b38219e3e12/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_3c87ec986c2656c2",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_3c87ec986c2656c2/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_3cd183d0bab85fb7",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_3cd183d0bab85fb7/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_420f2a187ce77035",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_420f2a187ce77035/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_444b73ff54a19b44",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_444b73ff54a19b44/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_53a9bdf4f75196da",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_53a9bdf4f75196da/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_585454b31673dd62",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_585454b31673dd62/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_7db4f140a955dd1a",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_7db4f140a955dd1a/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_83e53423a2dd93fe",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_83e53423a2dd93fe/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_9b24f8997145435c",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_9b24f8997145435c/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_9b754afb690c47e1",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_9b754afb690c47e1/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_a0be66af9a97ea52",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_a0be66af9a97ea52/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_af669445747d2585",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_af669445747d2585/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_af8b75f664ed7d43",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_af8b75f664ed7d43/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_b6cb6698327d9835",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_b6cb6698327d9835/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_b6ec3736f9ff2c62",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_b6ec3736f9ff2c62/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_becf2e1641bbd4e6",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_becf2e1641bbd4e6/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_c3d4322ec17fe7cd",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_c3d4322ec17fe7cd/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_c5e1490d672d026d",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_c5e1490d672d026d/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_cae5d3491190b777",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_cae5d3491190b777/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_d0992bc0387790a4",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_d0992bc0387790a4/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_d736268229bd87ec",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_d736268229bd87ec/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_e68d7111a2364a49",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_e68d7111a2364a49/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_ed6b180ec759bcf6",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_ed6b180ec759bcf6/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_f0ed3dc11cdeb130",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_f0ed3dc11cdeb130/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_f47eb9fc139f6bfd",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_f47eb9fc139f6bfd/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_f7f44e1e864aa967",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_f7f44e1e864aa967/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_ffa6e97b97146517",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_ffa6e97b97146517/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_point_at_infinity_392169c16a2e5ef6",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_point_at_infinity_392169c16a2e5ef6/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_point_at_infinity_3c1e8b38219e3e12",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_point_at_infinity_3c1e8b38219e3e12/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_point_at_infinity_3c87ec986c2656c2",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_point_at_infinity_3c87ec986c2656c2/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_point_at_infinity_420f2a187ce77035",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_point_at_infinity_420f2a187ce77035/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_point_at_infinity_83e53423a2dd93fe",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_point_at_infinity_83e53423a2dd93fe/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_incorrect_proof_point_at_infinity_ed6b180ec759bcf6",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_incorrect_proof_point_at_infinity_ed6b180ec759bcf6/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_commitment_1b44e341d56c757d",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_commitment_1b44e341d56c757d/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_commitment_32afa9561a4b3b91",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_commitment_32afa9561a4b3b91/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_commitment_3e55802a5ed3c757",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_commitment_3e55802a5ed3c757/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_commitment_e9d3e9ec16fbc15f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_commitment_e9d3e9ec16fbc15f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_proof_1b44e341d56c757d",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_proof_1b44e341d56c757d/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_proof_32afa9561a4b3b91",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_proof_32afa9561a4b3b91/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_proof_3e55802a5ed3c757",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_proof_3e55802a5ed3c757/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_proof_e9d3e9ec16fbc15f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_proof_e9d3e9ec16fbc15f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_y_35d08d612aad2197",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_y_35d08d612aad2197/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_y_4aa6def8c35c9097",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_y_4aa6def8c35c9097/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_y_4e51cef08a61606f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_y_4e51cef08a61606f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_y_64b9ff2b8f7dddee",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_y_64b9ff2b8f7dddee/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_y_b358a2e763727b70",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_y_b358a2e763727b70/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_y_eb0601fec84cc5e9",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_y_eb0601fec84cc5e9/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_z_35d08d612aad2197",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_z_35d08d612aad2197/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_z_4aa6def8c35c9097",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_z_4aa6def8c35c9097/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_z_4e51cef08a61606f",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_z_4e51cef08a61606f/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_z_64b9ff2b8f7dddee",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_z_64b9ff2b8f7dddee/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_z_b358a2e763727b70",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_z_b358a2e763727b70/data.yaml"),
        ),
        (
            "verify_kzg_proof_case_invalid_z_eb0601fec84cc5e9",
            include_str!("../tests/verify_kzg_proof/verify_kzg_proof_case_invalid_z_eb0601fec84cc5e9/data.yaml"),
        ),
    ];

    // pub const VERIFY_BLOB_KZG_PROOF_BATCH_TESTS: [(&str, &str); 27] = [
    pub const VERIFY_BLOB_KZG_PROOF_BATCH_TESTS: [(&str, &str); 1] = [
        // (
        //     "verify_blob_kzg_proof_case_correct_proof_0951cfd9ab47a8d3",
        //     include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_0951cfd9ab47a8d3/data.yaml"),
        // ),
        // (
        //     "verify_blob_kzg_proof_case_correct_proof_19b3f3f8c98ea31e",
        //     include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_19b3f3f8c98ea31e/data.yaml"),
        // ),
        // (
        //     "verify_blob_kzg_proof_case_correct_proof_84d8089232bc23a8",
        //     include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_84d8089232bc23a8/data.yaml"),
        // ),
        // (
        //     "verify_blob_kzg_proof_case_correct_proof_a87a4e636e0f58fb",
        //     include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_a87a4e636e0f58fb/data.yaml"),
        // ),
        // (
        //     "verify_blob_kzg_proof_case_correct_proof_c40b9b515df8721b",
        //     include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_c40b9b515df8721b/data.yaml"),
        // ),
        (
            "verify_blob_kzg_proof_case_correct_proof_cdb3e6d49eb12307",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_cdb3e6d49eb12307/data.yaml"),
        ),
        // (
        //     "verify_blob_kzg_proof_case_correct_proof_fb324bc819407148",
        //     include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_fb324bc819407148/data.yaml"),
        // ),
    //     (
    //         "verify_blob_kzg_proof_case_incorrect_proof_0951cfd9ab47a8d3",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_0951cfd9ab47a8d3/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_incorrect_proof_19b3f3f8c98ea31e",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_19b3f3f8c98ea31e/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_incorrect_proof_84d8089232bc23a8",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_84d8089232bc23a8/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_incorrect_proof_a87a4e636e0f58fb",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_a87a4e636e0f58fb/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_incorrect_proof_c40b9b515df8721b",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_c40b9b515df8721b/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_incorrect_proof_cdb3e6d49eb12307",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_cdb3e6d49eb12307/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_incorrect_proof_fb324bc819407148",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_fb324bc819407148/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_incorrect_proof_point_at_infinity",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_point_at_infinity/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_blob_59d64ff6b4648fad",
    //         include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_blob_59d64ff6b4648fad/data.yaml"),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_blob_635fb2de5b0dc429",
    //         include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_blob_635fb2de5b0dc429/data.yaml"),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_blob_a3b9ff28507767f8",
    //         include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_blob_a3b9ff28507767f8/data.yaml"),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_blob_d3afbd98123a3434",
    //         include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_blob_d3afbd98123a3434/data.yaml"),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_commitment_1a68c47b68148e78",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_commitment_1a68c47b68148e78/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_commitment_24b932fb4dec5b2d",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_commitment_24b932fb4dec5b2d/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_commitment_3a6eb616efae0627",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_commitment_3a6eb616efae0627/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_commitment_d070689c3e15444c",
    //         include_str!(
    //             "../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_commitment_d070689c3e15444c/data.yaml"
    //         ),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_proof_1a68c47b68148e78",
    //         include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_proof_1a68c47b68148e78/data.yaml"),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_proof_24b932fb4dec5b2d",
    //         include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_proof_24b932fb4dec5b2d/data.yaml"),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_proof_3a6eb616efae0627",
    //         include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_proof_3a6eb616efae0627/data.yaml"),
    //     ),
    //     (
    //         "verify_blob_kzg_proof_case_invalid_proof_d070689c3e15444c",
    //         include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_proof_d070689c3e15444c/data.yaml"),
    //     ),
    ];

    pub const VERIFY_BLOB_KZG_PROOF_TESTS: [(&str, &str); 29] = [
        (
            "verify_blob_kzg_proof_case_correct_proof_0951cfd9ab47a8d3",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_0951cfd9ab47a8d3/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_correct_proof_19b3f3f8c98ea31e",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_19b3f3f8c98ea31e/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_correct_proof_84d8089232bc23a8",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_84d8089232bc23a8/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_correct_proof_a87a4e636e0f58fb",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_a87a4e636e0f58fb/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_correct_proof_c40b9b515df8721b",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_c40b9b515df8721b/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_correct_proof_cdb3e6d49eb12307",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_cdb3e6d49eb12307/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_correct_proof_fb324bc819407148",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_fb324bc819407148/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_point_at_infinity_for_twos_poly/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_correct_proof_point_at_infinity_for_zero_poly/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_incorrect_proof_0951cfd9ab47a8d3",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_0951cfd9ab47a8d3/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_incorrect_proof_19b3f3f8c98ea31e",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_19b3f3f8c98ea31e/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_incorrect_proof_84d8089232bc23a8",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_84d8089232bc23a8/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_incorrect_proof_a87a4e636e0f58fb",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_a87a4e636e0f58fb/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_incorrect_proof_c40b9b515df8721b",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_c40b9b515df8721b/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_incorrect_proof_cdb3e6d49eb12307",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_cdb3e6d49eb12307/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_incorrect_proof_fb324bc819407148",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_fb324bc819407148/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_incorrect_proof_point_at_infinity",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_incorrect_proof_point_at_infinity/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_blob_59d64ff6b4648fad",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_blob_59d64ff6b4648fad/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_blob_635fb2de5b0dc429",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_blob_635fb2de5b0dc429/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_blob_a3b9ff28507767f8",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_blob_a3b9ff28507767f8/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_blob_d3afbd98123a3434",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_blob_d3afbd98123a3434/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_commitment_1a68c47b68148e78",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_commitment_1a68c47b68148e78/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_commitment_24b932fb4dec5b2d",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_commitment_24b932fb4dec5b2d/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_commitment_3a6eb616efae0627",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_commitment_3a6eb616efae0627/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_commitment_d070689c3e15444c",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_commitment_d070689c3e15444c/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_proof_1a68c47b68148e78",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_proof_1a68c47b68148e78/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_proof_24b932fb4dec5b2d",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_proof_24b932fb4dec5b2d/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_proof_3a6eb616efae0627",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_proof_3a6eb616efae0627/data.yaml"),
        ),
        (
            "verify_blob_kzg_proof_case_invalid_proof_d070689c3e15444c",
            include_str!("../tests/verify_blob_kzg_proof/verify_blob_kzg_proof_case_invalid_proof_d070689c3e15444c/data.yaml"),
        ),
    ];
}
use super::{u128_to_uint256, Q128};
use alloy_primitives::U256;

/// Computes the amount of fees owed to a position
pub fn get_tokens_owed(
    fee_growth_inside_0_last_x128: U256,
    fee_growth_inside_1_last_x128: U256,
    liquidity: u128,
    fee_growth_inside_0_x128: U256,
    fee_growth_inside_1_x128: U256,
) -> (U256, U256) {
    let liquidity = u128_to_uint256(liquidity);
    let tokens_owed_0 =
        (fee_growth_inside_0_x128 - fee_growth_inside_0_last_x128) * liquidity / Q128;
    let tokens_owed_1 =
        (fee_growth_inside_1_x128 - fee_growth_inside_1_last_x128) * liquidity / Q128;
    (tokens_owed_0, tokens_owed_1)
}

use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::{constants::{FEED_ID, MAX_AGE, PRICE_FEED_DECIMALS_ADJUSTMENT}, Collateral, Config};
use crate::error::CustomError;


pub fn check_health_factor(
    collateral:&Account<Collateral>,
    config: &Account<Config>,
    price_feed:&Account<PriceUpdateV2>,
) -> Result<u64> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;
    require!(health_factor >= config.min_health_factor, CustomError::BelowMinimumHealthFactor);
    Ok(health_factor)
}

pub fn calculate_health_factor(
    collateral:&Account<Collateral>,
    config: &Account<Config>,
    price_feed:&Account<PriceUpdateV2>,
) -> Result<u64> {
    let collateral_value_in_usd = get_price_in_usd(
        &collateral.lamports_balance,
        price_feed,
    )?;
    let colleteral_adjusted_for_liquidation_threshold = (collateral_value_in_usd * config.liquidation_threshold) / 100;
    if collateral.amount_minted == 0 {
        return Ok(u64::MAX);
    }
    let health_factor = (colleteral_adjusted_for_liquidation_threshold) / config.min_health_factor;
    Ok(health_factor as u64)
}

pub fn get_price_in_usd(
    amount_in_lamports: &u64,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAX_AGE, &feed_id)?;
    require!(price.price > 0, CustomError::InvalidPrice );

    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMALS_ADJUSTMENT;
    let amount_in_usd = (*amount_in_lamports as u128 * price_in_usd) / LAMPORTS_PER_SOL as u128;
    Ok(amount_in_usd as u64)
}  

pub fn get_lamports_from_usd(
    amount_in_usd: &u64,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAX_AGE, &feed_id)?;
    require!(price.price > 0, CustomError::InvalidPrice );
    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMALS_ADJUSTMENT;
    let amount_in_lamports = (*amount_in_usd as u128 * LAMPORTS_PER_SOL as u128) / price_in_usd;
    Ok(amount_in_lamports as u64)
}
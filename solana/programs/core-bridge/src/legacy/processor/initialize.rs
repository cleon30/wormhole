use crate::{
    error::CoreBridgeError,
    legacy::{instruction::InitializeArgs, utils::LegacyAnchorized},
    state::{Config, GuardianSet},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(args: InitializeArgs)]
pub struct Initialize<'info> {
    /// Account to warehouse Core Bridge program info. This account is especially important for
    /// redeeming governance VAAs, where the guardian set attesting for a governance decree must be
    /// the one encoded in this account.
    #[account(
        init,
        payer = payer,
        space = Config::INIT_SPACE,
        seeds = [Config::SEED_PREFIX],
        bump,
    )]
    config: Account<'info, LegacyAnchorized<Config>>,

    /// New guardian set account acting as the active guardian set. This account is set up as the
    /// legacy schema (without a discriminator) for local testing purposes.
    ///
    /// NOTE: There are other Core Bridge smart contracts that take an additional guardian set index
    /// parameter to initialize a present-day guardian set at initialization. But because the Core
    /// Bridge already exists on Solana's mainnet and devnet, we keep initialization assuming the
    /// initial guardian set is index 0.
    #[account(
        init,
        payer = payer,
        space = GuardianSet::compute_size(args.initial_guardians.len()),
        seeds = [
            GuardianSet::SEED_PREFIX,
            u32::to_be_bytes(0).as_ref()
        ],
        bump,
    )]
    guardian_set: Account<'info, LegacyAnchorized<GuardianSet>>,

    /// Account used to collect Wormhole fees.
    ///
    /// CHECK: This system account is created and will be used whenever the post message
    /// instructions are invoked.
    #[account(
        init,
        payer = payer,
        space = 0,
        seeds = [crate::constants::FEE_COLLECTOR_SEED_PREFIX],
        bump,
        owner = system_program.key(),
    )]
    fee_collector: AccountInfo<'info>,

    #[account(mut)]
    payer: Signer<'info>,

    /// Previously needed sysvar.
    ///
    /// CHECK: This account is unchecked.
    _clock: UncheckedAccount<'info>,

    /// Previously needed sysvar.
    ///
    /// CHECK: This account is unchecked.
    _rent: UncheckedAccount<'info>,

    system_program: Program<'info, System>,
}

impl<'info> crate::legacy::utils::ProcessLegacyInstruction<'info, InitializeArgs>
    for Initialize<'info>
{
    const LOG_IX_NAME: &'static str = "LegacyInitialize";

    const ANCHOR_IX_FN: fn(Context<Self>, InitializeArgs) -> Result<()> = initialize;
}

/// Processor to initialize the program.
///
/// NOTE: This instruction handler does not set the upgrade authority to the Core Bridge's upgrade
/// authority PDA. Because this instruction is from the legacy program's implementation, we do not
/// want to disturb the peace by implementing a new instruction to replace this one. Practically,
/// the Core Bridge is already deployed on Solana's mainnet-beta and devnet, so would never need to
/// initialize again. And for local validator testing (in most cases) the program is simply loaded
/// in the validator and cannot be upgraded.
fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
    let InitializeArgs {
        guardian_set_ttl_seconds,
        fee_lamports,
        initial_guardians,
    } = args;

    // We need at least one guardian for the initial guardian set.
    require!(
        !initial_guardians.is_empty(),
        CoreBridgeError::ZeroGuardians
    );

    // Check initial guardians.
    let mut keys = Vec::with_capacity(initial_guardians.len());
    for &guardian in initial_guardians.iter() {
        // We disallow guardian pubkeys that have zero address.
        require!(guardian != [0; 20], CoreBridgeError::GuardianZeroAddress);

        // Check if this pubkey is a duplicate of any already added.
        require!(
            !keys.iter().any(|key| *key == guardian),
            CoreBridgeError::DuplicateGuardianAddress
        );
        keys.push(guardian);
    }

    // Set Bridge data account fields.
    ctx.accounts.config.set_inner(
        Config {
            guardian_set_index: 0,
            guardian_set_ttl: guardian_set_ttl_seconds.into(),
            fee_lamports,
            _gap_0: Default::default(),
        }
        .into(),
    );

    // Set guardian set account fields.
    ctx.accounts.guardian_set.set_inner(
        GuardianSet {
            index: 0,
            creation_time: Clock::get().map(Into::into)?,
            keys,
            expiration_time: Default::default(),
        }
        .into(),
    );

    Ok(())
}

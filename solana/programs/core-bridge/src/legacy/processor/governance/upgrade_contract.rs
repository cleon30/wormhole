use crate::{
    constants::{SOLANA_CHAIN, UPGRADE_SEED_PREFIX},
    error::CoreBridgeError,
    legacy::{instruction::EmptyArgs, utils::LegacyAnchorized},
    state::Config,
    utils::{self, vaa::VaaAccount},
};
use anchor_lang::prelude::*;
use solana_program::{bpf_loader_upgradeable, program::invoke_signed};

#[derive(Accounts)]
pub struct UpgradeContract<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// For governance VAAs, we need to make sure that the current guardian set was used to attest
    /// for this governance decree.
    #[account(
        mut,
        seeds = [Config::SEED_PREFIX],
        bump,
    )]
    config: Account<'info, LegacyAnchorized<Config>>,

    /// VAA account, which may either be the new EncodedVaa account or legacy PostedVaaV1
    /// account.
    ///
    /// CHECK: This account will be read via zero-copy deserialization in the instruction
    /// handler, which will determine which type of VAA account is being used. If this account
    /// is the legacy PostedVaaV1 account, its PDA address will be verified by this zero-copy
    /// reader.
    #[account(owner = crate::ID)]
    vaa: AccountInfo<'info>,

    /// Claim account (mut), which acts as replay protection after consuming data from the VAA
    /// account.
    ///
    /// Seeds: [emitter_address, emitter_chain, sequence],
    /// seeds::program = core_bridge_program.
    ///
    /// CHECK: This account is created via [claim_vaa](crate::utils::vaa::claim_vaa).
    /// This account can only be created once for this VAA.
    #[account(mut)]
    claim: AccountInfo<'info>,

    /// CHECK: We need this upgrade authority to invoke the BPF Loader Upgradeable program to
    /// upgrade this program's executable. We verify this PDA address here out of convenience to get
    /// the PDA bump seed to invoke the upgrade.
    #[account(
        seeds = [UPGRADE_SEED_PREFIX],
        bump,
    )]
    upgrade_authority: AccountInfo<'info>,

    /// Spill account to collect excess lamports.
    ///
    /// CHECK: This account receives any lamports after the result of the upgrade.
    #[account(mut)]
    spill: AccountInfo<'info>,

    /// Deployed implementation.
    ///
    /// CHECK: The pubkey of this account is checked in access control against the one encoded in
    /// the governance VAA.
    #[account(mut)]
    buffer: AccountInfo<'info>,

    /// Core Bridge program data needed for BPF Loader Upgradable program.
    ///
    /// CHECK: BPF Loader Upgradeable program needs this account to upgrade the program's
    /// implementation.
    #[account(
        mut,
        seeds = [crate::ID.as_ref()],
        bump,
        seeds::program = solana_program::bpf_loader_upgradeable::id(),
    )]
    program_data: AccountInfo<'info>,

    /// CHECK: This must equal the Core Bridge program ID for the BPF Loader Upgradeable program.
    #[account(
        mut,
        address = crate::ID
    )]
    this_program: AccountInfo<'info>,

    /// CHECK: BPF Loader Upgradeable program needs this sysvar.
    #[account(address = solana_program::sysvar::rent::id())]
    rent: AccountInfo<'info>,

    /// CHECK: BPF Loader Upgradeable program needs this sysvar.
    #[account(address = solana_program::sysvar::clock::id())]
    clock: AccountInfo<'info>,

    /// BPF Loader Upgradeable program.
    ///
    /// CHECK: In order to upgrade the program, we need to invoke the BPF Loader Upgradeable
    /// program.
    #[account(address = solana_program::bpf_loader_upgradeable::id())]
    bpf_loader_upgradeable_program: AccountInfo<'info>,

    system_program: Program<'info, System>,
}

impl<'info> crate::legacy::utils::ProcessLegacyInstruction<'info, EmptyArgs>
    for UpgradeContract<'info>
{
    const LOG_IX_NAME: &'static str = "LegacyUpgradeContract";

    const ANCHOR_IX_FN: fn(Context<Self>, EmptyArgs) -> Result<()> = upgrade_contract;
}

impl<'info> UpgradeContract<'info> {
    fn constraints(ctx: &Context<Self>) -> Result<()> {
        let vaa = VaaAccount::load(&ctx.accounts.vaa)?;
        let gov_payload = super::require_valid_governance_vaa(&ctx.accounts.config, &vaa)?;

        let decree = gov_payload
            .contract_upgrade()
            .ok_or(error!(CoreBridgeError::InvalidGovernanceAction))?;

        // Make sure that the contract upgrade is intended for this network.
        require_eq!(
            decree.chain(),
            SOLANA_CHAIN,
            CoreBridgeError::GovernanceForAnotherChain
        );

        // Read the implementation pubkey and check against the buffer in our account context.
        require_keys_eq!(
            Pubkey::from(decree.implementation()),
            ctx.accounts.buffer.key(),
            CoreBridgeError::ImplementationMismatch
        );

        // Done.
        Ok(())
    }
}

/// Processor for contract upgrade governance decrees. This instruction handler invokes the BPF
/// Loader Upgradeable program to upgrade this program's executable to the provided buffer.
#[access_control(UpgradeContract::constraints(&ctx))]
fn upgrade_contract(ctx: Context<UpgradeContract>, _args: EmptyArgs) -> Result<()> {
    let vaa = VaaAccount::load(&ctx.accounts.vaa).unwrap();

    // Create the claim account to provide replay protection. Because this instruction creates this
    // account every time it is executed, this account cannot be created again with this emitter
    // address, chain and sequence combination.
    utils::vaa::claim_vaa(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            utils::vaa::ClaimVaa {
                claim: ctx.accounts.claim.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
            },
        ),
        &crate::ID,
        &vaa,
        None,
    )?;

    // Finally upgrade.
    invoke_signed(
        &bpf_loader_upgradeable::upgrade(
            &crate::ID,
            &ctx.accounts.buffer.key(),
            &ctx.accounts.upgrade_authority.key(),
            &ctx.accounts.spill.key(),
        ),
        &ctx.accounts.to_account_infos(),
        &[&[UPGRADE_SEED_PREFIX, &[ctx.bumps["upgrade_authority"]]]],
    )
    .map_err(Into::into)
}

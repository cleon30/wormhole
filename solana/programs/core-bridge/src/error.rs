//! Errors that may arise when interacting with the Core Bridge Program.

use anchor_lang::prelude::error_code;

/// Errors relevant to Core Bridge's malfunction.
///
/// * \>= 0x0    -- General program related.
/// * \>= 0x10   -- General Core Bridge.
/// * \>= 0x20   -- General Core Bridge Governance.
/// * \>= 0x100  -- Legacy Post Message.
/// * \>= 0x200  -- Legacy Post VAA.
/// * \>= 0x300  -- Legacy Set Message Fee.
/// * \>= 0x400  -- Legacy Transfer Fees.
/// * \>= 0x500  -- Legacy Upgrade Contract.
/// * \>= 0x600  -- Legacy Guardian Set Update.
/// * \>= 0x700  -- Legacy Verify Signatures.
/// * \>= 0x800  -- Legacy Post Message Unreliable.
/// * \>= 0x1000 -- Core Bridge Anchor Instruction.
/// * \>= 0x2000 -- Core Bridge SDK.
///
/// NOTE: All of these error codes when triggered are offset by `ERROR_CODE_OFFSET` (6000). So for
/// example, `U64Overflow` will return as 6006.
#[error_code]
pub enum CoreBridgeError {
    #[msg("InvalidInstructionArgument")]
    InvalidInstructionArgument = 0x2,

    #[msg("AccountNotZeroed")]
    AccountNotZeroed = 0x3,

    #[msg("InvalidDataConversion")]
    InvalidDataConversion = 0x4,

    #[msg("U64Overflow")]
    U64Overflow = 0x6,

    #[msg("InvalidComputeSize")]
    InvalidComputeSize = 0x8,

    #[msg("InvalidChain")]
    InvalidChain = 0x10,

    #[msg("InvalidGovernanceEmitter")]
    InvalidGovernanceEmitter = 0x20,

    #[msg("InvalidGovernanceAction")]
    InvalidGovernanceAction = 0x22,

    #[msg("LatestGuardianSetRequired")]
    LatestGuardianSetRequired = 0x24,

    #[msg("GovernanceForAnotherChain")]
    GovernanceForAnotherChain = 0x26,

    #[msg("InvalidGovernanceVaa")]
    InvalidGovernanceVaa = 0x28,

    #[msg("InsufficientFees")]
    InsufficientFees = 0x100,

    #[msg("EmitterMismatch")]
    EmitterMismatch = 0x102,

    #[msg("NotReadyForPublishing")]
    NotReadyForPublishing = 0x104,

    #[msg("InvalidPreparedMessage")]
    InvalidPreparedMessage = 0x106,

    #[msg("ExecutableEmitter")]
    ExecutableEmitter = 0x108,

    #[msg("LegacyEmitter")]
    LegacyEmitter = 0x10a,

    #[msg("InvalidSignatureSet")]
    InvalidSignatureSet = 0x200,

    #[msg("InvalidMessageHash")]
    InvalidMessageHash = 0x202,

    #[msg("NoQuorum")]
    NoQuorum,

    #[msg("MessageMismatch")]
    MessageMismatch = 0x204,

    #[msg("NotEnoughLamports")]
    NotEnoughLamports = 0x400,

    #[msg("InvalidFeeRecipient")]
    InvalidFeeRecipient = 0x402,

    #[msg("ImplementationMismatch")]
    ImplementationMismatch = 0x500,

    #[msg("InvalidGuardianSetIndex")]
    InvalidGuardianSetIndex = 0x600,

    #[msg("GuardianSetMismatch")]
    GuardianSetMismatch = 0x700,

    #[msg("InstructionAtWrongIndex")]
    InstructionAtWrongIndex = 0x702,

    #[msg("EmptySigVerifyInstruction")]
    EmptySigVerifyInstruction = 0x703,

    #[msg("InvalidSigVerifyInstruction")]
    InvalidSigVerifyInstruction = 0x704,

    #[msg("GuardianSetExpired")]
    GuardianSetExpired = 0x706,

    #[msg("InvalidGuardianKeyRecovery")]
    InvalidGuardianKeyRecovery = 0x708,

    #[msg("SignerIndicesMismatch")]
    SignerIndicesMismatch = 0x70a,

    #[msg("PayloadSizeMismatch")]
    PayloadSizeMismatch = 0x800,

    #[msg("ZeroGuardians")]
    ZeroGuardians = 0x1010,

    #[msg("GuardianZeroAddress")]
    GuardianZeroAddress = 0x1020,

    #[msg("DuplicateGuardianAddress")]
    DuplicateGuardianAddress = 0x1030,

    #[msg("MessageAlreadyPublished")]
    MessageAlreadyPublished = 0x1040,

    #[msg("VaaWritingDisallowed")]
    VaaWritingDisallowed = 0x1050,

    #[msg("VaaAlreadyVerified")]
    VaaAlreadyVerified = 0x1060,

    #[msg("InvalidGuardianIndex")]
    InvalidGuardianIndex = 0x1070,

    #[msg("InvalidSignature")]
    InvalidSignature = 0x1080,

    #[msg("UnverifiedVaa")]
    UnverifiedVaa = 0x10a0,

    #[msg("VaaStillProcessing")]
    VaaStillProcessing = 0x10a2,

    #[msg("InWritingStatus")]
    InWritingStatus = 0x10a4,

    #[msg("NotInWritingStatus")]
    NotInWritingStatus = 0x10a6,

    #[msg("InvalidMessageStatus")]
    InvalidMessageStatus = 0x10a8,

    #[msg("HashNotComputed")]
    HashNotComputed = 0x10aa,

    #[msg("InvalidVaaVersion")]
    InvalidVaaVersion = 0x10ac,

    #[msg("InvalidCreatedAccountSize")]
    InvalidCreatedAccountSize = 0x10ae,

    #[msg("DataOverflow")]
    DataOverflow = 0x10b0,

    #[msg("ExceedsMaxPayloadSize (30KB)")]
    ExceedsMaxPayloadSize = 0x10b2,

    #[msg("CannotParseVaa")]
    CannotParseVaa = 0x10b4,

    #[msg("EmitterAuthorityMismatch")]
    EmitterAuthorityMismatch = 0x10b6,

    #[msg("InvalidProgramEmitter")]
    InvalidProgramEmitter = 0x10b8,

    #[msg("WriteAuthorityMismatch")]
    WriteAuthorityMismatch = 0x10ba,

    #[msg("PostedVaaPayloadTooLarge")]
    PostedVaaPayloadTooLarge = 0x10bc,

    #[msg("ExecutableDisallowed")]
    ExecutableDisallowed = 0x10be,
}

#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("7Z6JTqS5NQvxyFwTdYh9HCbwXESWy3qRP54dRnmahMGj");

pub mod constants;

mod processor;
pub(crate) use processor::*;

pub mod state;

#[program]
pub mod wormhole_mock_cpi_solana {
    use super::*;

    // core bridge

    pub fn mock_post_message(
        ctx: Context<MockPostMessage>,
        args: MockPostMessageArgs,
    ) -> Result<()> {
        processor::mock_post_message(ctx, args)
    }

    pub fn mock_post_message_unreliable(
        ctx: Context<MockPostMessageUnreliable>,
        args: MockPostMessageUnreliableArgs,
    ) -> Result<()> {
        processor::mock_post_message_unreliable(ctx, args)
    }

    pub fn mock_prepare_message_v1(
        ctx: Context<MockPrepareMessageV1>,
        args: MockPrepareMessageV1Args,
    ) -> Result<()> {
        processor::mock_prepare_message_v1(ctx, args)
    }
}

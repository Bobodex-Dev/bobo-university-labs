use anchor_lang::prelude::*;
use mpl_core::{accounts::BaseAssetV1, types::Key};

declare_id!("9hi3smXeanfs3TZzH7ia4JpcY3W36e6Yd1agpXVuFw7F");

#[program]
pub mod toolchain_probe {
    use super::*;

    pub fn read_asset(ctx: Context<ReadAsset>) -> Result<()> {
        let data = ctx.accounts.asset.try_borrow_data()?;
        require!(data.first() == Some(&(Key::AssetV1 as u8)), ProbeError::NotAnAsset);
        let asset = BaseAssetV1::from_bytes(&data).map_err(|_| ProbeError::DecodeFailed)?;
        msg!("owner {}", asset.owner);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadAsset<'info> {
    /// CHECK: owner and type are checked before decoding.
    #[account(owner = mpl_core::ID)]
    pub asset: UncheckedAccount<'info>,
}

#[error_code]
pub enum ProbeError {
    #[msg("not a Core asset")]
    NotAnAsset,
    #[msg("decode failed")]
    DecodeFailed,
}

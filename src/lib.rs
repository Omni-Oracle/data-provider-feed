use anchor_client::anchor_lang;
use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("4PrFuz5Nzqe52E2reKL6FZM9JF9dpQ4hzgP1RwN6rDBm");

#[program]
pub mod omni_oracle {
    use super::*;

    pub fn initialize_asset(ctx: Context<InitializeAsset>, assetId: Pubkey, metadata_url: String, name: String) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        let authority = ctx.accounts.authority.key();

        let clock = Clock::get()?;
        let timestamp = clock.unix_timestamp as u64;

        asset.id = assetId;
        asset.price = 0.0; // Initial price, can be set to a default or left for update
        asset.name = name;
        asset.metadata_url = metadata_url;
        asset.last_updated = timestamp; // Initial timestamp
        asset.reputation = Reputation::Low; // Default reputation
        asset.authority = authority;
        Ok(())
    }

    pub fn update_price(ctx: Context<UpdatePrice>, price: f64) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        if *ctx.accounts.authority.key != asset.authority {
            return Err(CustomError::Unauthorized.into());
        }

        let clock = Clock::get()?;
        let timestamp = clock.unix_timestamp as u64;

        asset.price = price;
        asset.last_updated = timestamp;
        Ok(())
    }

    pub fn set_reputation(ctx: Context<SetReputation>, level: Reputation) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        asset.reputation = level;
        Ok(())
    }

    pub fn get_asset_price(ctx: Context<GetAssetPrice>, assetId: Pubkey) -> Result<f64> {
        let asset = &ctx.accounts.asset;
        if asset.id != assetId {
            return Err(ProgramError::InvalidAccountData.into());
        }
        Ok(asset.price)
    }
}

#[derive(Accounts)]
#[instruction(assetId: Pubkey, name: String)]
pub struct InitializeAsset<'info> {
    #[account(
        init,
        seeds = [b"OMNI".as_ref(), assetId.as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 8 + size_of::<Asset>()
    )]
    pub asset: Account<'info, Asset>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub asset: Account<'info, Asset>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetReputation<'info> {
    #[account(mut)]
    pub asset: Account<'info, Asset>,
    pub update_authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(assetId: Pubkey)]
pub struct GetAssetPrice<'info> {
    #[account(
        seeds = [b"OMNI".as_ref(), assetId.as_ref()],
        bump,
    )]
    pub asset: Account<'info, Asset>,
}

#[account]
pub struct Asset {
    pub id: Pubkey,
    pub name: String,
    pub metadata_url: String,
    pub price: f64,
    pub last_updated: u64,
    pub reputation: Reputation,
    pub authority: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum Reputation {
    Low,
    Medium,
    High,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
}

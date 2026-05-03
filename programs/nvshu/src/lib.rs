use anchor_lang::prelude::*;

declare_id!("FuCn9zJcTa3Agq3vAVwknTiJR62Y66bDmPX4GhMMwNSo");

#[program]
pub mod nvshu {
    use super::*;

    pub fn create_contract(
        ctx: Context<CreateContract>,
        participant_wallets: Vec<Pubkey>,
        genders: Vec<Gender>,
        contract_text: String,
        duration_days: i64,
    ) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.creator = ctx.accounts.creator.key();
        contract.contract_text = contract_text;
        contract.created_at = Clock::get()?.unix_timestamp;
        contract.expires_at = if duration_days > 0 {
            contract.created_at + duration_days * 86400
        } else {
            0
        };
        contract.status = ContractStatus::Pending;

        contract.participants = vec![
            Participant {
                wallet: ctx.accounts.creator.key(),
                gender: genders[0].clone(),
                signed: true,
            }
        ];

        for i in 1..participant_wallets.len() {
            contract.participants.push(Participant {
                wallet: participant_wallets[i],
                gender: genders[i].clone(),
                signed: false,
            });
        }

        Ok(())
    }

    pub fn sign_contract(ctx: Context<SignContract>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let signer = ctx.accounts.signer.key();

        let me = contract
            .participants
            .iter_mut()
            .find(|p| p.wallet == signer)
            .ok_or(ErrorCode::NotAParticipant)?;

        require!(!me.signed, ErrorCode::AlreadySigned);
        me.signed = true;

        if contract.participants.iter().all(|p| p.signed) {
            contract.status = ContractStatus::Active;
        }

        Ok(())
    }

    pub fn annul_contract(ctx: Context<AnnulContract>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        require!(
            contract.status == ContractStatus::Active,
            ErrorCode::NotActive
        );
        contract.status = ContractStatus::Annulled;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateContract<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 200 + 8 + 8 + 1 + 1 + (4 + 10 * 34)
    )]
    pub contract: Account<'info, MarriageContract>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignContract<'info> {
    #[account(mut)]
    pub contract: Account<'info, MarriageContract>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct AnnulContract<'info> {
    #[account(mut)]
    pub contract: Account<'info, MarriageContract>,
    pub authority: Signer<'info>,
}

#[account]
pub struct MarriageContract {
    pub creator: Pubkey,
    pub participants: Vec<Participant>,
    pub contract_text: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub status: ContractStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct Participant {
    pub wallet: Pubkey,
    pub gender: Gender,
    pub signed: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Gender {
    Female,
    Male,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ContractStatus {
    Pending,
    Active,
    Expired,
    Annulled,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not a participant of this contract.")]
    NotAParticipant,
    #[msg("You have already signed this contract.")]
    AlreadySigned,
    #[msg("Contract is not active.")]
    NotActive,
}

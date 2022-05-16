use crate::error::MailError::InvalidInstruction;
use solana_program::program_error::ProgramError;


#[derive(Debug)]
pub enum MailInstruction {
    /// Initialize a new account
    /// 
    /// Accounts Expected
    /// 
    /// 1. `[writable] The AccountInfo of the account to be intialized
    InitAccount,
}

impl MailInstruction {



    // potential tags:
    // 0 => init Townhall system
    // 1 => init Account
    // 2 => Post
    // 3 => Repost
    // 4 => 
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitAccount,
            _ => return Err(InvalidInstruction.into()),
        })
    }
}


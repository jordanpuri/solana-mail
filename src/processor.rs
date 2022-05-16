use crate::error::MailError::NotWritable;
use crate::instruction::MailInstruction;
use crate::state::{Mail, MailAccount};
use borsh::BorshSerialize;
use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  msg,
  program_error::ProgramError,
  pubkey::Pubkey,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = MailInstruction::unpack(instruction_data)?;

        match instruction {
            MailInstruction::InitAccount => {
                msg!("Instruction: InitAccount");
                    Self::process_init_account(&accounts[0], program_id)
                }
        }
    }

    fn process_init_account(
        account: &AccountInfo,
        program_id: &Pubkey,
    ) -> ProgramResult {

        // check if writable enabled
        if !account.is_writable {
            return Err(NotWritable.into());
        }

        // check if account is owned by pda in question
        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let welcome_mail = Mail {
            id: String::from("00000000-0000-0000-0000-000000000000"),
            from_address: program_id.to_string(),
            to_address: account.key.to_string(),
            subject: String::from("Welcome"),
            body: String::from("This is a welcome message."),
            sent_date: String::from("5/15/2022, 10:15 AM"), // get a date library for this
        };

        let mail_account = MailAccount {
            inbox: vec![welcome_mail],
            sent: Vec::new(),
        };

        mail_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

        Ok(())
    }

}
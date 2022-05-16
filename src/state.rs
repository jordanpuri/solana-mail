// declare object models here
// this is generally what is being written to / written from

use borsh::{BorshDeserialize, BorshSerialize}; // can use serde?

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Mail {
    pub id: String,
    pub from_address: String,
    pub to_address: String,
    pub subject: String,
    pub body: String,
    pub sent_date: String,
}

// compose
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MailAccount {
    pub inbox: Vec<Mail>,
    pub sent: Vec<Mail>,
}
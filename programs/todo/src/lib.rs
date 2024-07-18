#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("87PqFzsGHbKCW2YYRwnaH74HLt8JEkSFkWDtxrC9FsNT");

#[program]
pub mod todo_prog {
    use super::*;
    pub fn initialize(_ctx: Context<InitializeUser>) -> Result<()> {
        let user_profile = &mut _ctx.accounts.user_profile;
        user_profile.authority = _ctx.accounts.authority.key();
        user_profile.last_todo = 0;
        user_profile.todo_count = 0;
        Ok(())
    }
    pub fn add_todo(_ctx: Context<AddTodo>, _content: String) -> Result<()> {
        let user_profile = &mut _ctx.accounts.user_profile;
        let todo_account = &mut _ctx.accounts.todo_account;
        todo_account.authority = _ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo;
        todo_account.content = _content;
        todo_account.checked = false;

        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();

        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        Ok(())
    }
    pub fn edit_todo(_ctx: Context<EditTodo>, todo_idx: u8, _content: String) -> Result<()> {
        let todo_account: &mut Box<Account<TodoAccount>> = &mut _ctx.accounts.todo_account;
        todo_account.authority = _ctx.accounts.authority.key();
        todo_account.content = _content;
        todo_account.checked = todo_account.checked;

        Ok(())
    }
    pub fn mark_todo(_ctx: Context<MarkTodo>, todo_idx: u8, checked: bool) -> Result<()> {
        let todo_account: &mut Box<Account<TodoAccount>> = &mut _ctx.accounts.todo_account;
        todo_account.authority = _ctx.accounts.authority.key();
        todo_account.checked = checked;

        Ok(())
    }
    pub fn remove_todo(ctx: Context<RemoveTodo>, todo_idx: u8) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap();

        Ok(())
    }
}

// ::::::::: DATA STRUCTURES ::::::::::::
#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>()
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTodo<'info> {
    #[account(
    mut,
    seeds = [USER_TAG, authority.key().as_ref()],
    bump,
    has_one = authority
  )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref(), user_profile.last_todo.to_string().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<TodoAccount>()
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct EditTodo<'info> {
    #[account(
    mut,
    seeds = [USER_TAG, authority.key().as_ref()],
    bump,
    has_one = authority,
  )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    #[account(
    mut,
    seeds = [TODO_TAG, authority.key().as_ref(), todo_idx.to_string().as_ref()],
    bump,
    has_one = authority,
  )]
    pub todo_account: Box<Account<'info, TodoAccount>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct MarkTodo<'info> {
    #[account(
    mut,
    seeds = [USER_TAG, authority.key().as_ref()],
    bump,
    has_one = authority
  )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    #[account(
    mut,
    seeds = [TODO_TAG, authority.key().as_ref(), todo_idx.to_string().as_ref()],
    bump,
    has_one = authority
  )]
    pub todo_account: Box<Account<'info, TodoAccount>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct RemoveTodo<'info> {
    #[account(
    mut,
    seeds = [USER_TAG, authority.key().as_ref()],
    bump,
    has_one = authority
  )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    #[account(
    mut,
    close = authority,
    seeds = [TODO_TAG, authority.key().as_ref(), todo_idx.to_string().as_ref()],
    bump,
    has_one = authority
  )]
    pub todo_account: Box<Account<'info, TodoAccount>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// ::::::::: CONSTANTS ::::::::::::
#[constant]
pub const USER_TAG: &[u8] = b"USER_STATE";
#[constant]
pub const TODO_TAG: &[u8] = b"TODO_STATE";

// ::::::::: STATES ::::::::::::
#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub last_todo: u8,
    pub todo_count: u8,
}
#[account]
#[derive(Default)]
pub struct TodoAccount {
    pub authority: Pubkey,
    pub idx: u8,
    pub content: String,
    pub checked: bool,
}

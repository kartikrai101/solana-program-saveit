use anchor_lang::prelude::*;

declare_id!("JDXp7QVMLbFR6QKmx8Zwhef6cqMw3nWwmrVqx8K5yB7A");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_songs.
    base_account.total_songs = 0;
    Ok(())
  }

  pub fn add_song(ctx: Context<AddSong>, song_link: String, song_name: String, song_artist: String) -> Result <()> {
    // Get a reference to the account and increment total_gifs.
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    // build the struct
    let item = ItemStruct{
        song_link: song_link.to_string(),
        song_name: song_name.to_string(),
        song_artist: song_artist.to_string(),
        user_address: *user.to_account_info().key,
    };

    // add it to the song list vector
    base_account.song_list.push(item);
    base_account.total_songs += 1;
    Ok(())
  }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddSong<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub song_link: String,
    pub song_name: String,
    pub song_artist: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_songs: u64,
    pub song_list: Vec<ItemStruct>,
}

// first we create a context of what we want to create and then we can create
// a function that references this context and processes upon the data from this 
// context.
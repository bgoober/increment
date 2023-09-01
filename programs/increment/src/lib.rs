use anchor_lang::prelude::*;

declare_id!("DAbYzXj29pr9977i7fp6CXwJMxQPb8hDHEaWQkdjDGKJ"); 

#[program]
mod increment {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        let button = &mut ctx.accounts.button;
        button.presses = 0;
        Ok(())
    }

    pub fn press(ctx: Context<Press>) -> Result<()> {
        let button = &mut ctx.accounts.button;
        button.presses += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub button: Account<'info, Button>,
    #[account(mut)] // Make user mutable
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Press<'info> {
    #[account(mut)]
    pub button: Account<'info, Button>,
}

#[account]
pub struct Button {
    pub presses: u64,
}

// // Importing necessary modules from the anchor_lang library.
// use anchor_lang::prelude::*;

// // This declares an ID for the program. It's a unique identifier for it.
// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// // This annotation marks the module as one containing the core logic of a Solana program.
// #[program]
// mod basic_2 {
//     // Importing everything from the outer scope.
//     use super::*;

//     // This function initializes a new 'Counter' account.
//     // The Context contains information about the current state and involved accounts.
//     pub fn create(ctx: Context<Create>, authority: Pubkey) -> Result<()> {
//         // Getting a mutable reference to the 'counter' account from the context.
//         let counter = &mut ctx.accounts.counter;

//         // Setting the 'authority' of the 'counter' account.
//         counter.authority = authority;
        
//         // Initializing the count of the 'counter' account to 0.
//         counter.count = 0;

//         // Returning Ok(()) indicates the function executed successfully.
//         Ok(())
//     }

//     // This function increments the 'count' of the 'Counter' account.
//     pub fn increment(ctx: Context<Increment>) -> Result<()> {
//         // Getting a mutable reference to the 'counter' account from the context.
//         let counter = &mut ctx.accounts.counter;

//         // Incrementing the count by 1.
//         counter.count += 1;

//         // Returning Ok(()) indicates the function executed successfully.
//         Ok(())
//     }
// }

// // This structure defines the accounts involved when creating a new 'Counter' account.
// #[derive(Accounts)]
// pub struct Create<'info> {
//     // The 'counter' account being initialized.
//     // 'init' means this account is being created.
//     // 'payer' indicates who will pay for the account creation.
//     // 'space' specifies the amount of space required for this account in bytes.
//     #[account(init, payer = user, space = 8 + 40)]
//     pub counter: Account<'info, Counter>,

//     // The 'user' is the signer who pays for the account creation and can modify it.
//     #[account(mut)]
//     pub user: Signer<'info>,

//     // Reference to the Solana System Program, needed for account creation.
//     pub system_program: Program<'info, System>,
// }

// // This structure defines the accounts involved when incrementing the 'Counter' account.
// #[derive(Accounts)]
// pub struct Increment<'info> {
//     // The 'counter' account that's being incremented.
//     // 'has_one' ensures the 'authority' is the owner of this account.
//     #[account(mut, has_one = authority)]
//     pub counter: Account<'info, Counter>,

//     // The 'authority' or the owner who has permission to increment the counter.
//     pub authority: Signer<'info>,
// }

// // The structure that represents the data of a 'Counter' account on-chain.
// #[account]
// pub struct Counter {
//     // The public key of the authority or owner of this account.
//     pub authority: Pubkey,

//     // The value that this counter holds.
//     pub count: u64,
// }
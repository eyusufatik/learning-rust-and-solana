use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_tic_tac_toe {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        ctx.accounts.game.start([ctx.accounts.player_one.key(), player_two]);
        Ok(())
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        let game = &mut ctx.accounts.game;

        require_keys_eq!(
            game.current_player(),
            ctx.accounts.player.key(),
            TicTacToeError::NotPlayersTurn
        );

        game.play(&tile)
    }
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum Sign {
    X,
    O,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
}

#[error_code]
pub enum TicTacToeError {
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyOver,
    NotPlayersTurn,
    GameAlreadyStarted,
}

#[account]
pub struct Game {
    players: [Pubkey; 2],          // 32 * 2       = 64
    turn: u8,                      // 1            = 01
    board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1)  = 18
    state: GameState,              // 32 + 1       = 33
}

impl Game {
    pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (9 * (1 + 1)) + (32 + 1); // Didn't understand

    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
        require_eq!(self.turn, 0, TicTacToeError::GameAlreadyStarted);

        self.players = players;
        self.turn = 1;

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }

    fn current_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    pub fn current_player(&self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        require!(self.is_active(), TicTacToeError::GameAlreadyOver);

        if tile.row > 2 || tile.column > 2 {
            return Err(TicTacToeError::TileOutOfBounds.into());
        }

        match self.board[tile.row as usize][tile.column as usize] {
            Some(_) => return Err(TicTacToeError::TileAlreadySet.into()),
            None => {
                self.board[tile.row as usize][tile.column as usize] =
                    Some(Sign::from_usize(self.current_player_index()).unwrap());
            }
        }

        self.update_state();

        // self.state is updated in the above statement
        if self.state == GameState::Active {
            self.turn += 1;
        }

        Ok(())
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;

        self.board[first.0][first.1].is_some()
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }

    fn update_state(&mut self) {
        // check:
        //          1. same row
        //          2. same column
        //          3. diagonals

        for i in 0..=2 {
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)])
                || self.is_winning_trio([(0, i), (1, i), (2, i)])
            {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
        }

        if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
            || self.is_winning_trio([(0, 2), (1, 1), (2, 0)])
        {
            self.state = GameState::Won {
                winner: self.current_player(),
            };
            return;
        }

        // if we reach here and nobody won,
        // ninth turn has been played and
        // still nobody won
        if self.turn == 9 {
            self.state = GameState::Tie;
            return;
        }
    }
}

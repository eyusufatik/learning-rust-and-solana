import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AnchorTicTacToe } from '../target/types/anchor_tic_tac_toe';
import { expect } from 'chai';

async function play(
    program: Program<AnchorTicTacToe>,
    game,
    player,
    tile,
    expectedTurn,
    expectedGameState,
    expectedBoard
) {
    await program.methods
        .play(tile)
        .accounts({
            player: player.publicKey,
            game
        })
        .signers(player instanceof (anchor.Wallet as any) ? [] : [player])
        .rpc();

    const gameState = await program.account.game.fetch(game);
    expect(gameState.turn).to.equal(expectedTurn);
    expect(gameState.state).to.eql(expectedGameState);
    expect(gameState.board).to.eql(expectedBoard);
}

describe('anchor_tic_tac_toe', () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace
        .AnchorTicTacToe as Program<AnchorTicTacToe>;

    it('setup game!', async () => {
        const gameKeypair = anchor.web3.Keypair.generate();
        const playerOne = (program.provider as anchor.AnchorProvider).wallet;
        const playerTwo = anchor.web3.Keypair.generate();

        await program.methods
            .setupGame(playerTwo.publicKey)
            .accounts({
                game: gameKeypair.publicKey,
                playerOne: playerOne.publicKey
            })
            .signers([gameKeypair])
            .rpc();

        let gameState = await program.account.game.fetch(gameKeypair.publicKey);
        expect(gameState.turn).to.equal(1);
        expect(gameState.players).to.eql([
            playerOne.publicKey,
            playerTwo.publicKey
        ]);
        expect(gameState.state).to.eql({ active: {} });
        expect(gameState.board).to.eql([
            [null, null, null],
            [null, null, null],
            [null, null, null]
        ]);
    });

    it('player one wins', async () => {
        const gameKeypair = anchor.web3.Keypair.generate();
        const playerOne = (program.provider as anchor.AnchorProvider).wallet;
        const playerTwo = anchor.web3.Keypair.generate();

        await program.methods
            .setupGame(playerTwo.publicKey)
            .accounts({
                game: gameKeypair.publicKey,
                playerOne: playerOne.publicKey
            })
            .signers([gameKeypair])
            .rpc();

        let gameState = await program.account.game.fetch(gameKeypair.publicKey);
        expect(gameState.turn).to.equal(1);
        expect(gameState.players).to.eql([
            playerOne.publicKey,
            playerTwo.publicKey
        ]);
        expect(gameState.state).to.eql({ active: {} });
        expect(gameState.board).to.eql([
            [null, null, null],
            [null, null, null],
            [null, null, null]
        ]);

        await play(
            program,
            gameKeypair.publicKey,
            playerOne,
            { row: 0, column: 0 },
            2,
            { active: {} },
            [
                [{ x: {} }, null, null],
                [null, null, null],
                [null, null, null]
            ]
        );

        await play(
            program,
            gameKeypair.publicKey,
            playerTwo,
            { row: 0, column: 1 },
            3,
            { active: {} },
            [
                [{ x: {} }, { o: {} }, null],
                [null, null, null],
                [null, null, null]
            ]
        );

        await play(
            program,
            gameKeypair.publicKey,
            playerOne,
            { row: 1, column: 0 },
            4,
            { active: {} },
            [
                [{ x: {} }, { o: {} }, null],
                [{ x: {} }, null, null],
                [null, null, null]
            ]
        );

        await play(
            program,
            gameKeypair.publicKey,
            playerTwo,
            { row: 0, column: 2 },
            5,
            { active: {} },
            [
                [{ x: {} }, { o: {} }, { o: {} }],
                [{ x: {} }, null, null],
                [null, null, null]
            ]
        );

        await play(
            program,
            gameKeypair.publicKey,
            playerOne,
            { row: 2, column: 0 },
            5,
            { won: { winner: playerOne.publicKey } },
            [
                [{ x: {} }, { o: {} }, { o: {} }],
                [{ x: {} }, null, null],
                [{ x: {} }, null, null]
            ]
        );
    });
});

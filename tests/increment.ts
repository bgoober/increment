const assert = require("assert");
const anchor = require("@coral-xyz/anchor");
const { SystemProgram } = anchor.web3;

describe("increment", () => {
  const provider = anchor.AnchorProvider.local();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // Button for the tests.
  const button = anchor.web3.Keypair.generate();

  // Program for the tests.
  const program = anchor.workspace.Increment;

  it("Creates a button", async () => {
    await program.rpc.create({
      accounts: {
        button: button.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [button, provider.wallet.payer],
      instructions: [
        await program.account.button.createInstruction(button),
      ],
    });

    const buttonAccount = await program.account.button.fetch(button.publicKey);

    // Assert that the number of presses is initially 0.
    assert.ok(buttonAccount.presses.toNumber() === 0);
  });

  it("Presses a button", async () => {
    await program.rpc.press({
      accounts: {
        button: button.publicKey,
      }
    });

    const buttonAccount = await program.account.button.fetch(button.publicKey);

    // Assert that the button has been pressed once.
    assert.ok(buttonAccount.presses.toNumber() === 1);
  });
});

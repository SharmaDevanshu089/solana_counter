import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaCounter } from "../target/types/solana_counter";

describe("solana_counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solanaCounter as Program<SolanaCounter>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
  it("Can Incriment ", async() =>{
    const program = anchor.workspace.solanaCounter;
    const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("counter"), program.provider.publicKey.toBuffer()],program.programId);
    const tx = await program.methods.increment().accounts({ counter: counterPda,}).rpc();
    console.log("Increment tx signature:", tx);
    const counterAccount = await program.account.counter.fetch(counterPda);
    console.log("Counter after increment:", counterAccount.count);
  });
});

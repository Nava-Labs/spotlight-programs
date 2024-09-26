import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { SpotlightPrograms } from "../target/types/spotlight_programs";

const ESCROW_VAULT = "EscrowVault";
const ESCROW_SOL_VAULT = "EscrowSolVault";

describe("spotlight-programs", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .SpotlightPrograms as Program<SpotlightPrograms>;

  const connection = program.provider.connection;

  const SIGNER_AUTHORITY = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const [escrowVault] = PublicKey.findProgramAddressSync(
      [Buffer.from(ESCROW_VAULT)],
      program.programId,
    );

    const [escrowSolVault] = PublicKey.findProgramAddressSync(
      [Buffer.from(ESCROW_SOL_VAULT)],
      program.programId,
    );

    const tx = await program.methods
      .initialize()
      .accounts({
        escrowVault,
        escrowSolVault,
        signerAuthority: SIGNER_AUTHORITY.publicKey,
      })
      .signers([SIGNER_AUTHORITY])
      .rpc();
    console.log("Transaction signature", tx);

    const escrowState = await program.account.escrowVault.fetch(escrowVault);
    console.log(escrowState);
  });

  it("Requested!", async () => {
    const solAmount = new BN(15 * LAMPORTS_PER_SOL); // 15 SOL
    const tx = await program.methods.request(solAmount).rpc();
    console.log("Transaction signature", tx);

    const [escrowSolVault] = PublicKey.findProgramAddressSync(
      [Buffer.from(ESCROW_SOL_VAULT)],
      program.programId,
    );

    const solBalance = await connection.getBalance(escrowSolVault);
    console.log("vault balance: ", solBalance / LAMPORTS_PER_SOL, " SOL");
  });

  it("Claim!", async () => {
    const [escrowSolVault] = PublicKey.findProgramAddressSync(
      [Buffer.from(ESCROW_SOL_VAULT)],
      program.programId,
    );

    const solAmount = new BN(12 * LAMPORTS_PER_SOL); // 12 SOL
    let receiver = anchor.web3.Keypair.generate();
    const tx = await program.methods
      .claim(solAmount)
      .accounts({
        user: receiver.publicKey,
        signerAuthority: SIGNER_AUTHORITY.publicKey,
      })
      .signers([receiver, SIGNER_AUTHORITY])
      .rpc();
    console.log("Transaction signature", tx);

    const vaultBalance = await connection.getBalance(escrowSolVault);
    console.log("vault balance: ", vaultBalance / LAMPORTS_PER_SOL, " SOL");

    const claimerBalance = await connection.getBalance(receiver.publicKey);
    console.log("claimer balance: ", claimerBalance / LAMPORTS_PER_SOL, " SOL");
  });
});

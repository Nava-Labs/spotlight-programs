import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import { IDL, SpotlightPrograms } from "../target/types/spotlight_programs";

const secretKeyDeployer = Uint8Array.from(require("../keys/deployer.json"));
const keypairDeployer = Keypair.fromSecretKey(secretKeyDeployer);
const walletDeployer = new anchor.Wallet(keypairDeployer);

const secretKeySignerAuthority = Uint8Array.from(
  require("../keys/signerAuthority.json"),
);
const keypairSignerAuthority = Keypair.fromSecretKey(secretKeySignerAuthority);

const ESCROW_VAULT = "EscrowVault";
const ESCROW_SOL_VAULT = "EscrowSolVault";

async function main() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const provider = new AnchorProvider(
    connection,
    walletDeployer,
    AnchorProvider.defaultOptions(),
  );
  anchor.setProvider(provider);

  const programId = new PublicKey(
    "CgBcBA5wtFsHaSMwDqpoTwweqVarEb8XUMYiLstNJJXo",
  );

  const program = new Program(
    IDL,
    programId,
    provider,
  ) as Program<SpotlightPrograms>;

  try {
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
        signerAuthority: keypairSignerAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([keypairDeployer, keypairSignerAuthority])
      .rpc();

    console.log("Transaction signature", tx);
  } catch (error) {
    console.error("Error initalize:", error);
  }
}

main().then(
  () => {
    console.log("Successfully run! 🚀");
    process.exit;
  },
  (err) => {
    console.log("Error", err);
  },
);

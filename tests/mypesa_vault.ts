import * as anchor from "@coral-xyz/anchor";
import { Program, Idl, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import { MypesaVault } from "../target/types/mypesa_vault";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import idl from "../target/idl/mypesa_vault.json";
import * as web3 from "@solana/web3.js";


describe("mypesa_vault", () => {
  // Configure the client to use the local cluster.
  //const { connection } = useConnection();
  //const wallet = useAnchorWallet();
  //const provider = new AnchorProvider(connection, wallet, {});
  //setProvider(provider);
  //const program = new Program(idl as MypesaVault);

  
  
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MypesaVault as Program<MypesaVault>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const mintOfToken = new web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    const pda = web3.PublicKey.findProgramAddress(program.programId);
    const tx = await program.methods.initializeVault().accounts({
        mypesaVaultAccountPda: pda,
        systemProgram: web3.SystemProgram.ProgramId,
        //tokenProgram: TOKEN_2022_PROGRAM_ID,
    }).rpc({commitment: "confirmed"});
    console.log("Your transaction signature", tx);
  });
});

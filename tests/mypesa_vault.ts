import * as anchor from "@coral-xyz/anchor";
import { Program, Idl, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import { MypesaVault } from "../target/types/mypesa_vault";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import idl from "../target/idl/idl.json";

describe("mypesa_vault", () => {
  // Configure the client to use the local cluster.
  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  const provider = new AnchorProvider(connection, wallet, {});
  setProvider(provider);
  const program = new Program(idl as MypesaVault);

  //anchor.setProvider(anchor.AnchorProvider.env());

  //const program = anchor.workspace.MypesaVault as Program<MypesaVault>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts().rpc();
    console.log("Your transaction signature", tx);
  });
});

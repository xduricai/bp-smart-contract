import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OracleSmartContract } from "../target/types/oracle_smart_contract";

describe("oracle-smart-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OracleSmartContract as Program<OracleSmartContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BundleGuard } from "../target/types/bundle_guard";

describe("bundle-guard", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BundleGuard as Program<BundleGuard>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

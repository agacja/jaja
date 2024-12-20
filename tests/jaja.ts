import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Jaja } from "../target/types/jaja";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccount,
} from "@solana/spl-token";

describe("jaja", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Jaja as Program<Jaja>;
  
  // Program and Token Constants
  const PROGRAM_ID = new PublicKey("6ar36VKC4QFyx3avJmBFg4C3121Yxu3mmTebbiJ3ASYU");
  const RETARDIO_MINT = new PublicKey("6ogzHhzdrQr9Pgv6hZ2MNze7UrzBMAFyBBWUYp1Fhitx");
  const WSOL_MINT = new PublicKey("So11111111111111111111111111111111111111112");
  const RAYDIUM_PROGRAM_ID = new PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

  // Test accounts
  let poolInfo: any;
  let userSolAccount: PublicKey;
  let userRetardioAccount: PublicKey;
  let forwardRetardioAccount: PublicKey;
  let programAuthority: PublicKey;
  let programBump: number;

  before(async () => {
    // Derive program's PDA
    [programAuthority, programBump] = await PublicKey.findProgramAddress(
      [Buffer.from("authority")],
      PROGRAM_ID
    );

    // Get pool information
    poolInfo = await getRaydiumPoolInfo();

    // Get or create user's SOL account
    userSolAccount = await getOrCreateWsolAccount(provider.wallet.publicKey);

    // Get or create user's Retardio account
    userRetardioAccount = await getOrCreateTokenAccount(
      RETARDIO_MINT,
      provider.wallet.publicKey
    );

    // Get or create forward account
    forwardRetardioAccount = await getOrCreateTokenAccount(
      RETARDIO_MINT,
      new PublicKey("YOUR_FORWARD_ADDRESS") // Replace with your forward address
    );
  });

  it("Initialize program", async () => {
    try {
      const tx = await program.methods
        .initialize(forwardRetardioAccount)
        .accounts({
          owner: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Initialization successful:", tx);
    } catch (error) {
      console.error("Error during initialization:", error);
      throw error;
    }
  });

  it("Swaps SOL for Retardio", async () => {
    try {
      const amountIn = new anchor.BN(100000000); // 0.1 SOL
      const minimumAmountOut = new anchor.BN(1000); // Adjust based on expected rate

      const tx = await program.methods
        .proxySwapBaseInput(amountIn, minimumAmountOut)
        .accounts({
          cpSwapProgram: RAYDIUM_PROGRAM_ID,
          payer: provider.wallet.publicKey,
          authority: poolInfo.authority,
          ammConfig: poolInfo.ammConfig,
          poolState: poolInfo.poolState,
          inputTokenAccount: userSolAccount,
          outputTokenAccount: userRetardioAccount,
          inputVault: poolInfo.solVault,
          outputVault: poolInfo.retardioVault,
          inputTokenProgram: TOKEN_PROGRAM_ID,
          outputTokenProgram: TOKEN_PROGRAM_ID,
          inputTokenMint: WSOL_MINT,
          outputTokenMint: RETARDIO_MINT,
          observationState: poolInfo.observationState,
        })
        .rpc();

      console.log("Swap transaction:", tx);

      // Verify the swap
      const retardioBalance = await getTokenBalance(userRetardioAccount);
      console.log("Retardio balance after swap:", retardioBalance);
    } catch (error) {
      console.error("Error during swap:", error);
      throw error;
    }
  });

  // Helper functions
  async function getRaydiumPoolInfo() {
    try {
      const response = await fetch("https://api.raydium.io/v2/main/pairs");
      const pools = await response.json();
      
      const retardioPool = pools.find((p: any) => 
        p.baseMint === RETARDIO_MINT.toString() || 
        p.quoteMint === RETARDIO_MINT.toString()
      );

      if (!retardioPool) {
        throw new Error("Retardio pool not found");
      }

      console.log("Found Retardio pool:", retardioPool);

      return {
        programId: new PublicKey(retardioPool.ammId),
        authority: new PublicKey(retardioPool.authority),
        poolState: new PublicKey(retardioPool.id),
        ammConfig: new PublicKey(retardioPool.ammConfig),
        solVault: new PublicKey(retardioPool.baseVault),
        retardioVault: new PublicKey(retardioPool.quoteVault),
        observationState: new PublicKey(retardioPool.observationId),
      };
    } catch (error) {
      console.error("Error fetching pool info:", error);
      throw error;
    }
  }

  async function getOrCreateWsolAccount(owner: PublicKey): Promise<PublicKey> {
    try {
      const ata = await getAssociatedTokenAddress(WSOL_MINT, owner);
      
      try {
        await createAssociatedTokenAccount(
          provider.connection,
          provider.wallet.payer as Keypair,
          WSOL_MINT,
          owner
        );
      } catch (e) {
        console.log("WSOL account might already exist:", e);
      }

      return ata;
    } catch (error) {
      console.error("Error creating WSOL account:", error);
      throw error;
    }
  }

  async function getOrCreateTokenAccount(
    mint: PublicKey,
    owner: PublicKey
  ): Promise<PublicKey> {
    try {
      const ata = await getAssociatedTokenAddress(mint, owner);
      
      try {
        await createAssociatedTokenAccount(
          provider.connection,
          provider.wallet.payer as Keypair,
          mint,
          owner
        );
      } catch (e) {
        console.log("Token account might already exist:", e);
      }

      return ata;
    } catch (error) {
      console.error("Error creating token account:", error);
      throw error;
    }
  }

  async function getTokenBalance(tokenAccount: PublicKey): Promise<number> {
    try {
      const accountInfo = await provider.connection.getTokenAccountBalance(tokenAccount);
      return Number(accountInfo.value.amount);
    } catch (error) {
      console.error("Error getting token balance:", error);
      throw error;
    }
  }
});
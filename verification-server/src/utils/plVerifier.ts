import { UltraHonkBackend } from "@aztec/bb.js";
import circuit from "./proof_leak_contracts.json"

type ProvingBackend = "honk" | "all";

export class ProofLeakVerifier{
   private honk?: UltraHonkBackend;

  constructor(
    /* The ACIR of the Noir circuit to prove */
    // circuit: CompiledCircuit,
    /* Define the prover backend to use */
    private provingBackend: ProvingBackend = "honk",
    /* Threads to use */
    private threads: number = 4
  ) {
    // initialize the backends
  
    if (provingBackend === "honk" || provingBackend === "all") {
      this.honk = new UltraHonkBackend(circuit.bytecode, { threads: this.threads });
    }

  }
  async generateVk() {
    switch (this.provingBackend) {
      case "honk":
        const vKey =  await this.honk?.getVerificationKey()
        
        return vKey
    }
  }

  /**
   * Verify a proof of a satisfying input to the circuit for a given proving scheme
   *
   * @param proof - the proof to verify
   * @param provingBackend - optionally provided if the class was initialized with both proving schemes
   * @returns true if the proof is valid, false otherwise
   */
  async verify(
    proof: any,
    provingBackend?: ProvingBackend
  ): Promise<boolean> {
    // determine proving backend to use
    let backend: UltraHonkBackend;
     if (
      (provingBackend === "honk" && this.honk) ||
      (this.provingBackend === "honk" && this.honk)
    ) {
      backend = this.honk;
    } else {
      throw new Error(`Proving scheme ${this.provingBackend} not initialized`);
    }
    // verify the proof
    return backend.verifyProof(proof);
  }

  /**
   * End the prover wasm instance(s) and clean up resources
   */
  async destroy() {
    if (this.honk) {
      await this.honk.destroy();
    }
  }
}
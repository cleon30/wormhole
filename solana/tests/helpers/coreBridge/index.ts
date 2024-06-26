import { Program } from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import CoreBridgeIdl from "../../../target/idl/wormhole_core_bridge_solana.json";
import { WormholeCoreBridgeSolana } from "../../../target/types/wormhole_core_bridge_solana";
import { ProgramId } from "./consts";

export * from "./consts";
export * from "./instructions";
export * from "./legacy";
export * from "./state";
export * from "./testing";
export * from "./types";
export * from "./utils";

export type CoreBridgeProgram = Program<WormholeCoreBridgeSolana>;

export function getProgramId(programId?: ProgramId): PublicKey {
  return new PublicKey(
    programId === undefined
      ? "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth" // mainnet
      : programId
  );
}

export function getAnchorProgram(connection: Connection, programId: PublicKey): CoreBridgeProgram {
  return new Program<WormholeCoreBridgeSolana>(CoreBridgeIdl as any, programId, { connection });
}

export function mainnet(): PublicKey {
  return getProgramId();
}

export function localnet(): PublicKey {
  return getProgramId("HwTijR9KsZmCipXpCnPWaScMUCQHXJYcCYS5gWAnj5gj");
}

export function testnet(): PublicKey {
  return getProgramId("3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5");
}

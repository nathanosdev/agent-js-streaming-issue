import "./fetch-polyfill.mjs";

import { HttpAgent, Actor } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { idlFactory } from "./canister-http-interface.mjs";

const GATEWAY = "https://ic0.app";
const CANISTER_ID = Principal.fromText("xrfpr-ryaaa-aaaaj-aiq7q-cai");

const agent = new HttpAgent({ host: GATEWAY });

const actor = Actor.createActor(idlFactory, {
  agent,
  canisterId: CANISTER_ID,
});

const httpRequest = {
  method: "GET",
  url: "/dashboard.js",
  headers: [],
  body: new Uint8Array(),
};

let httpResponse = await actor.http_request(httpRequest);

console.log("Headers:", httpResponse.headers);
console.log("Status Code:", httpResponse.status_code);
console.log("Streaming Strategy:", httpResponse.streaming_strategy);
console.log("Upgrade:", httpResponse.upgrade);

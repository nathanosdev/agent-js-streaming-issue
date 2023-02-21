use anyhow::Result;
use candid::Principal;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::Agent;
use ic_utils::call::SyncCall;
use ic_utils::interfaces::http_request::HeaderField;
use ic_utils::interfaces::HttpRequestCanister;

#[tokio::main]
async fn main() -> Result<()> {
    const REPLICA_ADDRESS: &str = "https://ic0.app";
    const CANISTER_ID: &str = "xrfpr-ryaaa-aaaaj-aiq7q-cai";
    const PATH: &str = "/dashboard.js";
    const HTTP_METHOD: &str = "GET";
    let headers: Vec<HeaderField> = vec![];

    let transport = ReqwestHttpReplicaV2Transport::create(REPLICA_ADDRESS)?;
    let agent = Agent::builder().with_transport(transport).build()?;
    let canister_id = Principal::from_text(CANISTER_ID)?;
    let canister_interface = HttpRequestCanister::create(&agent, canister_id);

    let (response,) = canister_interface
        .http_request(HTTP_METHOD, PATH, headers, &[])
        .call()
        .await?;

    println!("Headers: {:#?}", response.headers);
    println!("Status Code: {:?}", response.status_code);
    println!("Streaming Strategy: {:?}", response.streaming_strategy);
    println!("Upgrade: {:?}", response.upgrade);

    Ok(())
}

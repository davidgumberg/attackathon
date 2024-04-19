use std::process::Command;

use hex::ToHex;
use serde_json::Value;

struct HoldInvoice {
    hash: Vec<u8>,
    amount: u64,
}

struct Route {
    hops: Vec<String>,
    amt: u64,
}

struct GoodNode {
    id: u8,
    pubkey: String,
}

struct BadNode {
    id: u8,
    pubkey: String,
}

impl BadNode {
    fn new(id : u8) -> Self {
        // lncli0 getinfo | jq -r .identity_pubkey
        let get_info_string = String::from_utf8(
            bad_node_cmd(id)
                .arg("getinfo")
                .output()
                .expect("Failed to exec")
                .stdout
        )
        .expect("error reading stdout");

        let parsed_get_info: Value = serde_json::from_str(get_info_string.as_str())
            .expect("failed to parse");

        let pubkey:String = parsed_get_info["identity_pubkey"]
            .to_string().trim_matches('"').to_string();

        BadNode {
            id: id,
            pubkey: pubkey,
        }
    }

    fn cmd(&self) -> Command {
        bad_node_cmd(self.id)
    }

    fn open_channel(&self, pubkey: String, amt: u64, push_amt: u64 ) -> () {
        self.cmd()
            .args([pubkey, amt.to_string(), push_amt.to_string()]);
    }
}

impl GoodNode {
    fn new(id: u8) -> Self {
        let describegraph = bad_node_cmd(0)
            .args(["describegraph"]);
    }
}

impl HoldInvoice {
    fn new() -> Self {
        let hash:Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
        let hash_string:String = hash.encode_hex();
        println!("{}", hash_string);
        let output = bad_node_cmd(1)
            .arg("addholdinvoice")
            .arg(hash.encode_hex::<String>())
            .output()
            .expect("Failed to execute command");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        
        HoldInvoice {
            hash,
            amount: 1000,
        }
    }

    fn settle(&self) -> () {
        ();
    }
}

// alias lncli[index]="lncli --network=regtest --tlscertpath=/credentials/lnd[index]-tls.cert --macaroonpath=/credentials/lnd[index]-admin.macaroon --rpcserver=lightning-[index].warnet-armada"
fn bad_node_cmd(id : u8) -> Command {
    let tls_arg = format!("--tlscertpath=/credentials/lnd{id}-tls.cert");
    let macaroon_arg : String = format!("--macaroonpath=/credentials/lnd{id}-admin.macaroon");
    let rpc_arg : String = format!("--rpcserver=lightning-{id}.warnet-armada");

    let mut node = Command::new("lncli");

    node
        .arg("--network=regtest")
        .args([tls_arg, macaroon_arg, rpc_arg]);

    node
}

fn primitive_attack() -> () {
    ()
}

fn main() {
    let bad_node_0 = BadNode::new(0);
    let bad_node_1 = BadNode::new(1);
    let bad_node_2 = BadNode::new(2);

    println!("Bad 0 pk: {}", bad_node_0.pubkey);
    println!("Bad 1 pk: {}", bad_node_1.pubkey);
    println!("Bad 2 pk: {}", bad_node_2.pubkey);
}

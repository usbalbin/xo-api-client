use std::{collections::BTreeMap, env};

use async_trait::async_trait;
use xo_api_client::{Client, Vm, VmId};

const TOKEN_FILE: &str = "my_secret_token";

#[derive(Debug)]
struct TokenHandler;

#[async_trait]
impl xo_api_client::TokenHandler for TokenHandler {
    type SaveErr = std::io::Error;

    type LoadErr = std::io::Error;

    async fn save(&mut self, token: &str) -> Result<(), Self::SaveErr> {
        Ok(std::fs::write(TOKEN_FILE, token)?)
    }

    async fn load(&self) -> Result<String, Self::LoadErr> {
        std::fs::read_to_string(TOKEN_FILE)
    }
}

// We dont care about any of the data under the "other" attribute
// in this example
#[derive(serde::Deserialize)]
struct OtherInfo {}

impl xo_api_client::vm::OtherInfo for OtherInfo {}

#[tokio::main]
async fn main() {
    let url = "ws://localhost:8080/api/";

    match env::args().nth(1).as_deref() {
        Some("login") => {
            let username = String::from("admin@admin.net");
            let password = String::from("admin");
            Client::sign_in(url, username, password, &mut TokenHandler)
                .await
                .expect("Failed to sign in");
            return;
        }
        Some(_) => {
            println!("Usage: list_vms [login]");
            return;
        }
        None => (),
    }

    let con = Client::connect(url, &mut TokenHandler)
        .await
        .expect("Failed to connect to server, have you signed in?");

    let all_vms: BTreeMap<VmId, Vm<OtherInfo>> =
        con.get_vms(None, None).await.expect("Failed to list VMs");

    let test_vms = all_vms
        .iter()
        .filter(|(_id, vm)| vm.tags.iter().any(|tag| tag == "Test"));

    println!("All VMs with the tag 'Test':");
    for (id, vm) in test_vms {
        println!("ID: {:?}, Name: {}", id, vm.name_label);
    }
}

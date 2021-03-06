use std::collections::BTreeMap;
use xo_api_client::{
    api::vm::{Vm, VmId},
    credentials::EmailAndPassword,
    Client,
};

// We dont care about any of the data under the "other" attribute
// in this example
#[derive(serde::Deserialize)]
struct OtherInfo {}

impl xo_api_client::api::vm::OtherInfo for OtherInfo {}

#[tokio::main]
async fn main() {
    let url = "ws://localhost:8080/api/";
    let email = String::from("admin@admin.net");
    let password = String::from("admin");

    let con = Client::connect(url)
        .await
        .expect("Failed to connect to server");

    con.session
        .sign_in(EmailAndPassword { email, password })
        .await
        .expect("Failed to sign in");

    let all_vms: BTreeMap<VmId, Vm<OtherInfo>> = con
        .xo
        .get_objects(None, None)
        .await
        .expect("Failed to list VMs");

    let test_vms = all_vms
        .iter()
        .filter(|(_id, vm)| vm.tags.iter().any(|tag| tag == "Test"));

    println!("All VMs with the tag 'Test':");
    for (id, vm) in test_vms {
        println!("ID: {:?}, Name: {}", id, vm.name_label);
    }
}

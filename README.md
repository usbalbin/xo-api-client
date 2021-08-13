# xo-api-client

[![crates.io](https://img.shields.io/crates/v/xo-api-client.svg)](https://crates.io/crates/xo-api-client)
[![docs.rs](https://docs.rs/xo-api-client/badge.svg)](https://docs.rs/xo-api-client/)
[![dependency status](https://deps.rs/crate/xo-api-client/0.0.3/status.svg)](https://deps.rs/crate/xo-api-client/0.0.3)

<!--
TODO: Add tests before showing these badges

![Stable](https://github.com/usbalbin/xo-api-client/actions/workflows/stable.yml/badge.svg)
![Nightly](https://github.com/usbalbin/xo-api-client/actions/workflows/nightly.yml/badge.svg)
![Miri](https://github.com/usbalbin/xo-api-client/actions/workflows/miri.yml/badge.svg)
-->

Unofficial Rust crate for accessing [Xen Orchestra](https://github.com/vatesfr/xen-orchestra) through its API

## Under development
The library is still in early development, please do not use in production. The API is nowhere near complete and only covers
a very small fraction of XO's api. Lots of things might get changed and/or added in breaking ways at any time.

### Async Runtime
This library uses the tokio v1 runtime

## Example
Example of listing all VMs with the tag `Test`

```rust
use std::collections::BTreeMap;
use xo_api_client::{credentials::EmailAndPassword, Client, Vm, VmId};

// We dont care about any of the data under the "other" attribute
// in this example
#[derive(serde::Deserialize)]
struct OtherInfo {}

impl xo_api_client::vm::OtherInfo for OtherInfo {}

#[tokio::main]
async fn main() {
    let url = "ws://localhost:8080/api/";
    let email = String::from("admin@admin.net");
    let password = String::from("admin");

    let con = Client::connect(url)
        .await
        .expect("Failed to connect to server");
        
    con.sign_in(EmailAndPassword { email, password })
        .await
        .expect("Failed to sign in");

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
```

# License
`xo-api-client` is distributed under the terms of both the MIT license and
the Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in xo-api-client by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

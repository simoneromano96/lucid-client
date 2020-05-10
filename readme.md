# LucidKV Client

[![Build Status](https://cloud.drone.io/api/badges/simoneromano96/lucid-client/status.svg)](https://cloud.drone.io/simoneromano96/lucid-client)

LucidKV DB Client (Supports 0.14) based on Reqwest

For tests there must be a lucid db, you can use the preconfigured docker-compose/dockerfile.

I'll try to work on a Lucid image that support env variables, this will fix tests in CI and will help in other cases (that's the reason why the tests are failing on drone-ci).

## TODOs

* [x] Basic functionality with Reqwest
* [ ] Implement all endpoints (Execute a Specific Operation is missing)
* [ ] JWT Authentication
* [ ] Use feature flags to expose different HTTP Clients (AWC planned), this is useful to reduce bundle size for people that use already another HTTP Client in their application, reqwest has been used initially mostly for a PoC, I think that reqwest is mature and easy to use
* [ ] Add a check for DB version

Operations will be defined as an Enum to grant type safety.

Operations are:
* Lock / Unlock
* Increment / Decrement
* Define / Cancel Expiration

## Examples

Create a client:

Every operation will point to: `base_url/api/kv/` so you just need to specify the host.

By default the base_url will be `http://127.0.0.1:7020`

```rust
// Without a base URL
let lucid_client_1 = LucidKVClient::new(None);


// With a base URL
let lucid_client_2 = LucidKVClient::new(Some("http://1.2.3.4:7020".into()));
```

Operations:

```rust
use serde_derive::{Serialize};

// Serializable data
#[derive(Serialize)]
struct DemoData {
    demo: String,
}

// Demo data
let mut data = DemoData {
    demo: "Something",
};

// Client instance
let lucid_client = LucidKVClient::new(None);

// Create
let res = lucid_client.store_data("key".into(), data).await?;

assert_eq!(res.status(), reqwest::StatusCode::OK);

// Read
let res = lucid_client.get_data("key".into()).await.unwrap();

assert_eq!(res.status(), reqwest::StatusCode::OK);

// Update
data = DemoData {
    demo: "And now for something completely different",
}

let res = lucid_client.store_data("key".into(), data).await?;

assert_eq!(res.status(), reqwest::StatusCode::OK);

// Delete
// TODO

// Has key
// TODO

```

## Changelog

### 0.1

* Implementation of `new` to create a new client, optionally specify a `base_url`
* Implementation of CRUD Operations
* `is_key_present` sends `HEAD` request to lucid

### 0.2

* The only change has been `is_key_present`, this will return now a `bool`


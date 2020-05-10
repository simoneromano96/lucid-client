# LucidKV Client

LucidKV DB Client based on Reqwest

## TODOs

* [x] Basic functionality with Reqwest
* [ ] Implement all endpoints (Execute a Specific Operation is missing)
* [ ] JWT Authentication
* [ ] Use feature flags to expose different HTTP Clients (AWC planned), this is useful to reduce bundle size for people that use already another HTTP Client in their application

Operations are:
* Lock / Unlock
* Increment / Decrement
* Define / Cancel Expiration

## Examples

Create a client:

Every operation will point to: base_url/api/kv/

By default the base_url will be `http://127.0.0.1:7020`

```rust
// Without a base URL
let lucid_client_1 = LucidKVClient::new(None);


// With a base URL
let lucid_client_2 = LucidKVClient::new(Some("http://1.2.3.4:7020".into()));
```

Operations:

```rust
let lucid_client = LucidKVClient::new(None);

// Create

// Read

// Update

// Delete

// Has key

```

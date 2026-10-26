# shiprocket

Async Rust client for the [Shiprocket API](https://apidocs.shiprocket.in/).

- Covers every endpoint in the official API docs: orders, couriers, shipments, returns and exchanges, wrapper API, NDR, tracking, labels, manifests and invoices, pickup addresses, products, listings, channels, inventory, international, countries, billing and file imports.
- Logs in on demand, caches the token, refreshes it before it expires (tokens last 240 hours) and logs in again once if a request returns `401`.
- Typed requests with builders ([`bon`](https://docs.rs/bon)), so missing required fields are compile errors.
- Typed, lenient responses. Shiprocket mixes strings and numbers, sends `0`/`1` for booleans, and returns `{}` where you'd expect a list, so every response field is an `Option`. Unknown fields are kept in an `extra` map.

## Usage

```toml
[dependencies]
shiprocket = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Use the credentials of an **API user** (Settings → API → Configure in the Shiprocket panel), not your main login.

```rust,no_run
use shiprocket::{
    PaymentMethod, Shiprocket,
    couriers::AssignAwbRequest,
    orders::{CreateOrderRequest, OrderItem},
};

#[tokio::main]
async fn main() -> Result<(), shiprocket::Error> {
    let client = Shiprocket::new("api-user@example.com", "password");

    let order = client
        .orders()
        .create(
            &CreateOrderRequest::builder()
                .order_id("224-477")
                .order_date("2024-07-24 11:11")
                .pickup_location("Home")
                .billing_customer_name("Naruto")
                .billing_address("House 221B, Leaf Village")
                .billing_city("New Delhi")
                .billing_pincode("110002")
                .billing_state("Delhi")
                .billing_country("India")
                .billing_email("naruto@uzumaki.com")
                .billing_phone("9876543210")
                .shipping_is_billing(true)
                .order_items(vec![
                    OrderItem::builder()
                        .name("Kunai")
                        .sku("chakra123")
                        .units(10)
                        .selling_price(900.0)
                        .build(),
                ])
                .payment_method(PaymentMethod::Prepaid)
                .sub_total(9000.0)
                .length(10.0)
                .breadth(15.0)
                .height(20.0)
                .weight(2.5)
                .build(),
        )
        .await?;

    let shipment_id = order.shipment_id.expect("shipment id");
    let awb = client
        .couriers()
        .assign_awb(&AssignAwbRequest::builder().shipment_id(shipment_id).build())
        .await?;
    println!("{awb:?}");

    let label = client.documents().generate_label(&[shipment_id]).await?;
    println!("{label:?}");

    Ok(())
}
```

### Endpoint groups

Each group is reached through an accessor on the client:

| Accessor | Covers |
|---|---|
| `auth()` | login, logout |
| `orders()` | create (custom / channel), update, cancel, change pickup location, update address, fulfill, map products, import, list, show, export |
| `couriers()` | assign AWB, courier list, serviceability (incl. hyperlocal), generate pickup, blocked pincodes |
| `shipments()` | list, show, cancel by AWB |
| `returns()` | create return / exchange, update, list, return serviceability, return AWB |
| `wrapper()` | one-call forward and return shipments |
| `documents()` | manifests, labels, invoices |
| `ndr()` | list, show, take action |
| `tracking()` | by AWB, multiple AWBs, shipment id, order id |
| `pickup()` | list and add pickup locations |
| `account()` | wallet balance |
| `billing()` | statement, weight discrepancy |
| `products()`, `listings()`, `inventory()`, `channels()` | catalog management |
| `countries()` | countries, zones, postcode lookup |
| `imports()` | result of bulk CSV imports |
| `international()` | KYC, bank details, orders, serviceability, AWB, manifest, pickup, tracking |

### Webhooks

`shiprocket::webhook` decodes the tracking webhook body and checks the optional `x-api-key`
security token in constant time. It takes the raw header value and body, so it works with any
HTTP framework:

```rust
use shiprocket::webhook::{ShipmentStatus, TrackingWebhook, verify_api_key};

fn handle(api_key: Option<&str>, body: &[u8]) -> Result<(), &'static str> {
    if !verify_api_key(api_key, "token-configured-in-shiprocket") {
        return Err("bad token");
    }
    let event = TrackingWebhook::from_slice(body).map_err(|_| "bad body")?;
    match event.status() {
        Some(ShipmentStatus::Delivered) => { /* mark the order delivered */ }
        Some(ShipmentStatus::Unknown(code)) => { /* log `code` */ }
        _ => {}
    }
    Ok(())
}
```

Shiprocket only publishes a sample body, so every field is optional and unknown fields are kept
in `extra`. Answer the webhook with `200`.

### Tokens

```rust,no_run
# async fn run() -> Result<(), shiprocket::Error> {
use shiprocket::Shiprocket;

// Reuse a token persisted earlier; credentials allow re-login when it expires.
let client = Shiprocket::builder()
    .credentials("api-user@example.com", "password")
    .token("persisted-token")
    .build();

client.account().wallet_balance().await?;
let token_to_persist = client.token().await;
# Ok(()) }
```

`Shiprocket` is cheap to clone. Clones share the connection pool and the cached token, and concurrent requests trigger only one login. `Shiprocket::builder()` also accepts a custom `reqwest::Client` and base URLs (useful for mock servers).

### Errors

Non-2xx responses become `Error::Api` with the status, the message, the field-level validation errors (on `422` responses) and the raw body. Some endpoints (for example the wrapper API) report failures with HTTP 200 and a status flag in the body; check the response type's docs.

## Bruno collection

`bruno/` holds a [Bruno](https://www.usebruno.com/) collection of the whole API, generated from the official Postman collection. Open the folder in Bruno, select the `Production` environment, fill in `email` and `password`, and run **Authentication API → Generate Token**. The token is saved to the environment, and every other request uses it.

## License

MIT

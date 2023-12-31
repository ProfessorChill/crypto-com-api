# crypto-com-api

Asynchronous API for crypto.com.

This API is very early in development, please refer to the `TODO` section below.

## Completion Status.

### Websocket

| Feature                                      | Status             | Reason                                                                                                          |
| -------------------------------------------- | ------------------ | --------------------------------------------------------------------------------------------------------------- |
| `public/auth`                                | :white_check_mark: |                                                                                                                 |
| `public/get-instruments`                     | :white_check_mark: |                                                                                                                 |
| `private/set-cancel-on-disconnect`           | :white_check_mark: |                                                                                                                 |
| `private/get-cancel-on-disconnect`           | :white_check_mark: |                                                                                                                 |
| `private/get-withdrawal-history`             | :warning:          | Currently untested.                                                                                             |
| `user.order.{instrument_name}`               | :white_check_mark: |                                                                                                                 |
| `user.trade.{instrument_name}`               | :white_check_mark: |                                                                                                                 |
| `user.balance`                               | :warning:          | Unable to test since it requires the users balance to change and is therefore unsafe to test.                   |
| `book.{instrument_name}`                     | :white_check_mark: |                                                                                                                 |
| `ticker.{instrument_name}`                   | :white_check_mark: |                                                                                                                 |
| `trade.{instrument_name}`                    | :white_check_mark: |                                                                                                                 |
| `candlestick.{time_frame}.{instrument_name}` | :white_check_mark: |                                                                                                                 |
| `otc_book.{instrument_name}`                 | :white_check_mark: |                                                                                                                 |
| `private/get-account-summary`                | :white_check_mark: |                                                                                                                 |
| `private/create-order`                       | :warning:          | Unable to test as it requires creating an order which costs the tester money to do.                             |
| `private/cancel-order`                       | :warning:          | Same as `private/create-order`.                                                                                 |
| `private/create-order-list`                  | :warning:          | Same as `private/create-order`.                                                                                 |
| `private/cancel-order-list`                  | :warning:          | Same as `private/create-order`.                                                                                 |
| `private/get-order-history`                  | :white_check_mark: |                                                                                                                 |
| `private/get-open-orders`                    | :white_check_mark: |                                                                                                                 |
| `private/get-order-detail`                   | :warning:          | Requires a created order to test which costs the tester money. Not tested.                                      |
| `private/get-trades`                         | :white_check_mark: |                                                                                                                 |
| Sub-account API                              | :x:                | I do not have sub-accounts to test with so I could not test and add this to the API.                            |
| OTC Trading API                              | :x:                | I have never used the OTC Trading API and do not understand it so adding it would be unreasonable for me to do. |

### REST

This is a mainly incomplete feature as I saw Websocket to be of more importance.
Consider using the REST API to be unsafe.

REST will be completed at a later date as some of the routes are fairly
necessary.

| Feature                          | Status             | Reason                               |
| -------------------------------- | ------------------ | ------------------------------------ |
| `public/get-instruments`         | :white_check_mark: |                                      |
| `public/get-book`                | :white_check_mark: |                                      |
| `public/get-candlestick`         | :white_check_mark: |                                      |
| `public/get-ticker`              | :white_check_mark: |                                      |
| `public/get-trades`              | :white_check_mark: |                                      |
| `private/create-withdrawal`      | :warning:          | Requires the tester to pay per test. |
| `private/get-currency-networks`  | :warning:          | Untested.                            |
| `private/get-withdrawal-history` | :warning:          | Untested.                            |
| `private/get-deposit-history`    | :warning:          | Untested.                            |
| `private/get-deposit-address`    | :warning:          | Untested.                            |
| `private/get-account-summary`    | :warning:          | Untested.                            |
| `private/create-order`           | :x:                |                                      |
| `private/cancel-order`           | :x:                |                                      |
| `private/create-order-list`      | :x:                |                                      |
| `private/cancel-order-list`      | :x:                |                                      |
| `private/cancel-all-orders`      | :x:                |                                      |
| `private/get-order-history`      | :x:                |                                      |
| `private/get-open-orders`        | :x:                |                                      |
| `private/get-order-detail`       | :x:                |                                      |
| `private/get-trades`             | :x:                |                                      |

## Usage

This library is very early in development and should only be used as a proof of
concept right now.

With that said, refer to `tests` to see examples of usage until `examples` are
created.

## Testing

Tests will only run `websocket_basic` without the feature flag `test_authorized`
and a `.env` file containing:

I highly recommend running tests with the command
`cargo test -- --test-threads 1` to prevent flooding crypto.com with too many
requests as each test runs its own controller and creates a fresh connection.

```
WEBSOCKET_USER_API_ROOT_V2=wss://stream.crypto.com/v2/user
WEBSOCKET_MARKET_DATA_ROOT_V2=wss://stream.crypto.com/v2/market
API_KEY=YOUR_API_KEY
SECRET_KEY=YOUR_SECRET_KEY
```

## TODO

Merge duplicate tests.

Create the REST routes, add tests, and finalize that section.

Add tests that should panic to data types.

# crypto-com-api

Asynchronous API for crypto.com.

This API is very early in development, please refer to the `TODO` section below.

## Completion Status.

### Websocket

| Feature                                      | Status             | Reason |
| -------------------------------------------- | ------------------ | ------ |
| `public/auth`                                | :white_check_mark: |        |
| `public/get-instruments`                     | :x:                | Not developed yet, public routes are typically `REST` routes so I most likely missed this one. |
| `private/set-cancel-on-disconnect`           | :x:                | To be developed. |
| `private/get-cancel-on-disconnect`           | :x:                | To be developed. |
| `private/get-withdrawal-history`             | :warning:          | Currently untested. |
| `user.order.{instrument_name}`               | :white_check_mark: |        |
| `user.trade.{instrument_name}`               | :white_check_mark: |        |
| `user.balance`                               | :warning:          | Unable to test since it requires the users balance to change and is therefore unsafe to test. |
| `book.{instrument_name}`                     | :white_check_mark: |        |
| `ticker.{instrument_name}`                   | :white_check_mark: |        |
| `trade.{instrument_name}`                    | :white_check_mark: |        |
| `candlestick.{time_frame}.{instrument_name}` | :white_check_mark: |        |
| `otc_book.{instrument_name}`                 | :white_check_mark: |        |
| `private/get-account-summary`                | :white_check_mark: |        |
| `private/create-order`                       | :warning:          | Unable to test as it requires creating an order which costs the tester money to do. |
| `private/cancel-order`                       | :warning:          | Same as `private/create-order`. |
| `private/create-order-list`                  | :warning:          | Same as `private/create-order`. |
| `private/cancel-order-list`                  | :warning:          | Same as `private/create-order`. |
| `private/get-order-history`                  | :white_check_mark: |        |
| `private/get-open-orders`                    | :white_check_mark: |        |
| `private/get-order-detail`                   | :warning:          | Requires a created order to test which costs the tester money. Not tested. |
| `private/get-trades`                         | :white_check_mark: |        |
| Sub-account API                              | :x:                | I do not have sub-accounts to test with so I could not test and add this to the API. |
| OTC Trading API                              | :x:                | I have never used the OTC Trading API and do not understand it so adding it would be unreasonable for me to do. |

### REST

This is a mainly incomplete feature as I saw Websocket to be of more importance. Consider using the REST API to be unsafe.

REST will be completed at a later date as some of the routes are fairly necessary.

| Feature                          | Status             | Reason |
| -------------------------------- | ------------------ | ------ |
| `public/get-instruments`         | :white_check_mark: |        |
| `public/get-book`                | :white_check_mark: |        |
| `public/get-candlestick`         | :white_check_mark: |        |
| `public/get-ticker`              | :white_check_mark: |        |
| `public/get-trades`              | :white_check_mark: |        |
| `private/create-withdrawal`      | :warning:          | Requires the tester to pay per test. |
| `private/get-currency-networks`  | :x:                |        |
| `private/get-withdrawal-history` | :x:                |        |
| `private/get-deposit-history`    | :x:                |        |
| `private/get-deposit-address`    | :x:                |        |
| `private/get-account-summary`    | :x:                |        |
| `private/create-order`           | :x:                |        |
| `private/cancel-order`           | :x:                |        |
| `private/create-order-list`      | :x:                |        |
| `private/cancel-order-list`      | :x:                |        |
| `private/cancel-all-orders`      | :x:                |        |
| `private/get-order-history`      | :x:                |        |
| `private/get-open-orders`        | :x:                |        |
| `private/get-order-detail`       | :x:                |        |
| `private/get-trades`             | :x:                |        |

## Usage

This library is very early in development and should only be used as a proof of concept right now.

With that said, refer to `tests` to see examples of usage until `examples` are created.

## Testing

Tests will only run `websocket_basic` without the feature flag `test_authorized` as a `.env` file containing:

```
WEBSOCKET_USER_API_ROOT_V2=wss://stream.crypto.com/v2/user
WEBSOCKET_MARKET_DATA_ROOT_V2=wss://stream.crypto.com/v2/market
API_KEY=YOUR_API_KEY
SECRET_KEY=YOUR_SECRET_KEY
```

## TODO

Make initialization of `tokio` threads return a join handle to make it possible to have crashing and
non-crashing errors. As of right now, if `serde_json` were to encounter an error deserializing the
value from a websocket response the thread would never crash and it would forever continue looping
without handling the error.

Make `process_user` and `process_market` exit the thread upon encountering a critical error instead
of simply waiting for the next piece of data from the stream and only storing the error to be mapped
after the thread closes.

Add proper error handling to types, as an example, in `src/websocket/data/book.rs` we use
From instead of TryFrom despite it being possible for the incoming data to be inconsistent
and cause a failure.

Handle and return error codes sent from crypto.com in the form of `(Option<transaction_id>, ApiError::Code)`
or something of the likes instead of just ignoring the error.

Add tests that should panic to data types.

Finish creating the missing routes and adding tests where I reasonably can.

Create the REST routes, add tests, and finalize that section.

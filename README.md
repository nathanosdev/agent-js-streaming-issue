This repo is a reproduction of an issue with differing behaviours between agent-js and agent-rs.

It contains two apps, both of which will make a request for the `/dashboard.js` file of the `xrfpr-ryaaa-aaaaj-aiq7q-cai` canister on mainnet.

To run the RS version:

```
$ cd agent-rs
$ cargo run
```

To run the JS version:

```
$ cd agent-js
$ npm i
$ npm start
```


When running the RS version, the streaming strategy is returned as `None`.
When running the JS version, the streaming strategy is returned as `[ { Callback: { token: [Object], callback: [Array] } } ]`.

This causes the file to streamed when it is requested through the service worker, but not through ICX Proxy.

# Quick Start
- Generate a public private api key pair and store save in files located at 
`<project_root>/keys/kalshi-key-pub.pem` and `<project_root>/keys/kalshi-key.pem`

- Next, build the binaries with the command 
    ```{Bash}
    cargo build --release
    ```

- To recreate a kalshi orderbook live, run the command
    ```{Bash}
    target/release/kalshi_live_orderbook <MARKET_TICKER>
    ```

- To stream live data from a channel, run the command
    ```{Bash}
    target/release/kalshi_stream <MARKET_TICKER> <CHANNEL_NAME>
    ```
    Possible channels are 
    - orderbook_delta : sends an orderbook snapshot and then sends incremental changes
    - trade : send a message every time a trade is executed

- To stream live data and continuously append it to a file, run the command
    ```{Bash}
    target/release/kalshi_record <MARKET_TICKER> <CHANNEL_NAME> <PATH_TO_WRITE_FILE>
    ```
    Again, possible channels are 
    - orderbook_delta : sends an orderbook snapshot and then sends incremental changes
    - trade : send a message every time a trade is executed


# WebSocket Client


# Rest Client



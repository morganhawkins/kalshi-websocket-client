# Quick Start 

## Kalshi
 - Generate a public private api key pair and store save in files located at `<project_root>/keys/kalshi-key.pem` and `<project_root>/keys/kalshi-key.pem`

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
    possible channels are 
     - orderbook_delta : sends an orderbook snapshot and then sends incremental changes
     - trade : send a message every time a trade is executed
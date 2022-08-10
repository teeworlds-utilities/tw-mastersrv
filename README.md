# tw-mastersrv

## How to build and run ?
1. Install the dependencies
    - `cargo`

## Usage example

```rust
use tw_mastersrv::mastersrv::TwMasterServer;

fn main() {
    let url = "https://master1.ddnet.tw/ddnet/15/servers.json";    
    let master_srv = TwMasterServer::new(url);

    // Search server by name
    match master_srv.search_server_by_name("fng") {
        Some(servers) => {
            for server in servers {
                println!("{:?}", server.info.name)
            }
        },
        None => return
    };
}
```

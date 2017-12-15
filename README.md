# netflow_v9
A simple parser for the Netflow v9 protocol.
It collects the flows and outputs each in a JSON format.

## Install
In your Cargo.toml file:
```
netflow_v9 = { git = "https://github.com/Caspinol/netflow_v9.git" }
```

## Example
```
let mut parser = Parser::new();

if let Ok(sets) = parser.parse_netflow_packet(&packet_1) {
    for set in sets {
        let s = set.to_json();
        println!("{}",s);
    }
}
```

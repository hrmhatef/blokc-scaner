## Summary
This Repo contains two separate services:
    
    #### The **Indexer** will connect to the provided RPC node then store the **Transfer** event from the **USDT** contract address on the EHT network.
    
    #### The **Server** provides a graphql playground to make quries form the DB. 

## Build
```
    cargo build
```

## Config File
```
```

## Migration

    ### Generate the DB.


### Make file
    You can use the following command for simplicities:
| Command    | Descripton |
| -------- | ------- |
| make build  | Build the project    |
| make format | Run the cargo fmt to edit the codebase     |

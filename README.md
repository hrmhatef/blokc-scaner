## Summary
This repo contains two separate services:
	

 1. The **indexer** which is responsible to connect to a RPC node the store the event (Transfer) in the sqlite DB.
 2. **Server** provides a graphql playground to fetch the information from the DB.

## Config file
In [config](https://github.com/hrmhatef/blokc-scaner/blob/development/config/app.yaml) file you can find the default values for the app.


### Make file
The are some shortcut commands in the [Makefile](https://github.com/hrmhatef/blokc-scaner/blob/development/Makefile) to build and run the services also prepare the DB

### [Migration](https://github.com/hrmhatef/blokc-scaner/tree/development/migration)
Prepare the DB and run the schema you can use check the migration folder.


### Build & Run
Please install [Rust](https://www.rust-lang.org/tools/install) and [Sqlite](https://www.sqlite.org/download.html) based on your OS then execute  the following commands in your terminal to run the services.
```bash
	# create a database on the root folder
	sqlite3 indexer.sqlite
	# install the sea-orm-cli to manage the DB migration
	cargo  install sea-orm-cli
	# to make sure the DB path is correct and it is available
	make migrate.status
	# run the schema on the created database.
	# then you chan check the result which is "Applying all pending migrations" with the name of the files and tables.
	make migrate.apply
	# build be bin files
	make build
	# run the indexer
	make run-indexer
	# from another terminal tab or pan run the server
	make run-server
```

Now the indexer will add the data into the DB and check [playground](http://localhost:9080/api/graphql) to make any queries based on the Docs section on the playground.

### Graphql
#### Schema:
```
type BlockResult {
  blockNumber: Int!
  eventInfo: EventInfo!
  blockHash: String!
  timestamp: Int
  tag: String!
  transactionHash: String!
  tagIndex: Int!
  logIndex: Int!
  isRemoved: Boolean!
}

type BlocksInfo {
  # The list of `Shows` returned for the current page
  data: [BlockResult!]!
}

type EventInfo {
  from: String!
  to: String!
  value: String!
}

type Query {
  blocks(blockNumber: Int!): BlocksInfo!
  blocksByAddress(address: String!): BlocksInfo!
  circularTrnasactions(address: String!): BlocksInfo!
  totalBlocks: Int!
  totalEvents: Int!
}

```
#### Query example
##### request
```bash
{
	query{
	  totalEvents
	  totalBlocks
	  blocks(blockNumber:23191371){
	    data{
	      blockHash
	    }
	  }
	}
}
```

##### response

<details>
 
```json
{
  "data": {
    "totalEvents": 1864,
    "totalBlocks": 31,
    "blocks": {
      "data": [
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        },
        {
          "blockHash": "0xae5f91f03118be121f70058301690f5be9a9521dd143a8817af7df553ebdcd9b"
        }
      ]
    }
  }
}
```

 
</details>


#### List of queries
```c++
	// returns all events based on the provided blockNumber
	blocks(blockNumber: int) BlocksInfo
	// returns all events which is the address is equal with From or To
	blocksByAddress(address: string) BlocksInfo
	// returns all events which is the address is in same place of From and To 
	circularTransactions(address: string) BlocksInfo
	// return total blocks stored on the DB
	totalBlocks() int
	// returns total events of the DB 
	totalEvents() int 
```


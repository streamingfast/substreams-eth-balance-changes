# Substreams ETH Balance Changes
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Overview

The goal of this Substreams project is to extract all ETH transfers from Ethereum events for the full chain.

The `map_balance_changes` module will output messages of type `eth.types.v1.BalanceChange` defined by: 

```proto
message BalanceChanges {
  repeated BalanceChange balance_changes = 1;
}

message BalanceChange {
  string address = 1;
  string old_value = 2; // in wei
  string new_value = 3; // in wei
  string reason = 4;
  uint64 ordinal = 5;

  string transaction = 6; //if the change is done in a transaction
}
```

## Build

To build the .spkg file, run:

```bash
make pack
```

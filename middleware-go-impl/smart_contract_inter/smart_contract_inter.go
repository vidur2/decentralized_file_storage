package smartcontractinter

import (
	"context"

	"github.com/ethereum/go-ethereum/rpc"
	api "github.com/textileio/near-api-go"
	"github.com/textileio/near-api-go/keys"
	"github.com/textileio/near-api-go/transaction"
	"github.com/textileio/near-api-go/types"
)

const PRIVATE_KEY = "ed25519:5LkcnjVhkApabobMfp9671pPdu4KM6bCGh4V7MnHMKXnD9WaPEjcqoex788xAMaCjDD9CHNUAhkHt8ijRQDboft6"
const ACCT_ID = "filechain.testnet"

func HandleAddFileTransaction(tokens_transferred float64, timestamp uint64, from_account_id string) bool {
	client := initWalletConn()
	ctx := context.Background()

	res, err := client.Account(ACCT_ID).FunctionCall(
		ctx,
		ACCT_ID,
		"add_txn",
		transaction.FunctionCallWithArgs(map[string]interface{}{
			"tokens_transferred": tokens_transferred,
			"timestamp":          timestamp,
			"from_account_id":    from_account_id,
			"to_account_id":      "network",
		}),
	)

	_, status := res.GetStatus()

	if err != nil {
		return false
	} else if status {
		return true
	} else {
		return false
	}
}

func initWalletConn() *api.Client {
	rpcClient, err := rpc.DialContext(context.Background(), "https://rpc.testnet.near.org")

	if err != nil {
		return nil
	}

	keyPair, err := keys.NewKeyPairFromString(PRIVATE_KEY)

	if err != nil {
		return nil
	}

	config := &types.Config{
		RPCClient: rpcClient,
		Signer:    keyPair,
		NetworkID: "testnet",
	}

	client, err := api.NewClient(config)

	if err != nil {
		return nil
	}

	return client
}

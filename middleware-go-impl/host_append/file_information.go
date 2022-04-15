package hostappend

import (
	"crypto/ed25519"
	"math/rand"
	"strconv"
	"time"
	"vidur2/middleware/util"
)

// Struct for serialization and deserialization purposes
//
// Used for testing whether a node functions the appropriate way
type FileInformation struct {
	Data              string   `json:"data"`
	LinkedUri         string   `json:"linked_uri"`
	Creator           []uint16 `json:"creator"`
	Version           string   `json:"version"`
	FileType          string   `json:"file_type"`
	TokensTransferred int64    `json:"tokens_transferred"`
	ToAcctId          string   `json:"to_acct_id"`
	Signature         []uint16 `json:"signature"`
	Timestamp         int64    `json:"timestamp"`
}

// Constructor for FileInformation
//
// Constructs a random FileInformation with random feilds
func generateRandomFileInformation() FileInformation {
	timestamp := time.Now().Unix()
	publicKey, privateKey, _ := ed25519.GenerateKey(rand.New(rand.NewSource(timestamp)))
	linked_uri := util.RandSeq(8)
	signature := ed25519.Sign(privateKey, []byte(linked_uri+strconv.FormatInt(timestamp, 10)+"0"))
	publicKeyBytes := []byte(publicKey)
	publicKeyNums := make([]uint16, 0)
	for _, char := range publicKeyBytes {
		publicKeyNums = append(publicKeyNums, uint16(char))
	}

	signatureNums := make([]uint16, 0)
	for _, char := range signature {
		signatureNums = append(signatureNums, uint16(char))
	}
	return FileInformation{
		Data:              util.RandSeq(8),
		LinkedUri:         linked_uri,
		Creator:           publicKeyNums,
		Version:           util.RandSeq(8),
		FileType:          "Frontend",
		TokensTransferred: 0,
		ToAcctId:          "shitpost",
		Signature:         signatureNums,
		Timestamp:         timestamp,
	}
}

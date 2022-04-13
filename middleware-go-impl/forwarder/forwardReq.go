package forwarder

import (
	"encoding/json"
	"fmt"
	"math/rand"
	"strconv"
	"strings"
	"time"
	smartcontractinter "vidur2/middleware/smart_contract_inter"
	"vidur2/middleware/util"

	"github.com/libp2p/go-libp2p-core/crypto"
	"github.com/valyala/fasthttp"
	"golang.org/x/crypto/bcrypt"
)

type FileInformation struct {
	Data      string `json:"data"`
	LinkedUri string `json:"linked_uri"`
	Creator   string `json:"creator"`
	Version   string `json:"version"`
	FileType  string `json:"file_type"`
}

type FileMessage struct {
	Data      FileInformation `json:"data"`
	Timestamp int64           `json:"timestamp"`
}

/*
Forwards a request from the reverse proxy to a linked node

ctx: The context of the recieved request from the reverse proxy
validated: A list of active nodes
*/
func ForwardOperation(ctx *fasthttp.RequestCtx, validated []string) {

	// Original getting of variables
	var clientReqBody string

	// Variables for file storage information
	var file []byte
	var time int64

	body := ctx.Request.Body()

	// Handling different request paths
	if string(ctx.Path()) != "/store_information" {
		clientReqBody = string(body)
	} else {
		// Storing information in variables
		file, time = TransformFile(body)
		clientReqBody = string(file)
	}

	serverErr, ipAddr, idx := getAvailableServer(validated)
	res, err := _handleFileOperation(ctx, "http://"+ipAddr, clientReqBody)

	// Keeps going until either an active server is found or no servers remain
	for err != nil && serverErr == "" {
		validated = util.Remove(validated, idx)
		serverErr, ipAddr, idx = getAvailableServer(validated)
		res, err = _handleFileOperation(ctx, "http://"+ipAddr, clientReqBody)
	}

	// If there is no server err return content
	if serverErr == "" {
		ctx.SetStatusCode(fasthttp.StatusOK)

		// Interaction with the smart contract
		if string(ctx.Path()) == "/store_information" {
			var fileInformation FileInformation
			err := json.Unmarshal(body, &fileInformation)

			if err == nil {

				// Gets the other parameters needed in the smart contract call
				creator := getPublicKey(fileInformation.Creator)

				if creator != "" {
					tokOwed := float64(len([]byte(fileInformation.Data))) / 1_000_000_000.0
					smartcontractinter.HandleAddFileTransaction(tokOwed, uint64(time), creator)
				}
			}
		}

		// Appends response
		body := string(res.Body())
		ctx.Response.AppendBodyString(body)
	} else {
		ctx.SetStatusCode(fasthttp.StatusServiceUnavailable)
		ctx.Response.AppendBodyString("All nodes are inactive right now")
	}

	util.ValidatedChannel <- validated
}

// Parses public key from private key
func getPublicKey(secret string) string {
	privateKey, err := crypto.UnmarshalEd25519PrivateKey([]byte(secret))

	if err != nil {
		publicKey, _ := crypto.MarshalPublicKey(privateKey.GetPublic())
		return string(publicKey)
	} else {
		return ""
	}
}

func HandleGetPeers(ctx *fasthttp.RequestCtx, validated []string) {
	util.ValidatedChannel <- validated
	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.Response.AppendBodyString(serializeValidated(validated))
}

func TransformFile(body []byte) ([]byte, int64) {
	var parsed FileInformation
	err := json.Unmarshal(body, &parsed)

	if err != nil {
		return nil, 0
	}

	timestamp := time.Now().Unix()

	fullBody := FileMessage{
		Timestamp: timestamp,
		Data:      parsed,
	}

	hashed, err := bcrypt.GenerateFromPassword([]byte(fullBody.Data.Creator+strconv.Itoa(int(fullBody.Timestamp))), 10)

	if err != nil {
		return nil, 0
	}

	fullBody.Data.Creator = string(hashed)

	newBody, err := json.Marshal(fullBody)

	if err != nil {
		return nil, 0
	}

	return newBody, timestamp
}

func serializeValidated(validated []string) string {
	retString := "["
	for idx, server := range validated {
		server = strings.Replace(server, ":8002", ":8003", 1)
		if idx != len(validated)-1 {
			retString += server + ","
		} else {
			retString += server + "]"
		}
	}

	return retString
}

// Helper function to act as a request client
func _handleFileOperation(ctx *fasthttp.RequestCtx, ipAddr string, clientReqBody string) (fasthttp.Response, error) {
	req := fasthttp.AcquireRequest()

	if string(ctx.Path()) != "/get_blocks" {
		req.Header.SetMethod(fasthttp.MethodPost)
		req.AppendBodyString(clientReqBody)
	} else {
		req.Header.SetMethod(fasthttp.MethodGet)
	}

	req.SetRequestURI(ipAddr + string(ctx.Path()))

	res := fasthttp.AcquireResponse()

	err := util.Client.Do(req, res)

	if err != nil {
		fmt.Println(err)
	}

	return *res, err
}

// Gets an active server at random
func getAvailableServer(validated []string) (string, string, int) {
	var chosenServer string
	var randomIndex int
	var err string
	err = ""

	if len(validated) > 1 {
		randomIndex = rand.Intn(len(validated) - 1)
		chosenServer = validated[randomIndex]
	} else if len(validated) == 1 {
		chosenServer = validated[0]
	} else {
		err = "No active servers right now"
	}

	return err, chosenServer, randomIndex
}

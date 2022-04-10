package forwarder

import (
	"encoding/json"
	"fmt"
	"math/rand"
	"strings"
	"time"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
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
func ForwardOperation(ctx *fasthttp.RequestCtx, validated []string) []string {

	// Original getting of variables
	var clientReqBody string

	if string(ctx.Path()) != "/store_information" {
		clientReqBody = string(ctx.Request.Body())
	} else {
		clientReqBody = string(HandleAddFile(ctx.Request.Body()))
	}
	serverErr, ipAddr, idx := getAvailableServer(validated)
	fmt.Println(ipAddr)
	err, res := _handleFileOperation(ctx, "http://"+ipAddr, clientReqBody)

	// Keeps going until either an active server is found or no servers remain
	for err != nil && serverErr == "" {
		validated = util.Remove(validated, idx)
		serverErr, ipAddr, idx = getAvailableServer(validated)
		err, res = _handleFileOperation(ctx, "http://"+ipAddr, clientReqBody)
	}

	// If there is no server err return content
	if serverErr == "" {
		ctx.SetStatusCode(fasthttp.StatusOK)
		body := string(res.Body())
		ctx.Response.AppendBodyString(body)
	} else {
		ctx.SetStatusCode(fasthttp.StatusServiceUnavailable)
		ctx.Response.AppendBodyString("All nodes are inactive right now")
	}

	return validated
}

func HandleGetPeers(ctx *fasthttp.RequestCtx, validated []string) {
	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.Response.AppendBodyString(serializeValidated(validated))
}

func HandleAddFile(body []byte) []byte {
	var parsed FileInformation
	err := json.Unmarshal(body, &parsed)

	if err != nil {
		return nil
	}

	fullBody := FileMessage{
		Timestamp: time.Now().Unix(),
		Data:      parsed,
	}

	newBody, err := json.Marshal(fullBody)

	if err != nil {
		return nil
	}

	return newBody
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
func _handleFileOperation(ctx *fasthttp.RequestCtx, ipAddr string, clientReqBody string) (error, fasthttp.Response) {
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

	fmt.Println(string(res.Body()))

	if err != nil {
		fmt.Println(err)
	}

	return err, *res
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

package forwarder

import (
	"fmt"
	"math/rand"
	"strings"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

/*
Forwards a request from the reverse proxy to a linked node

ctx: The context of the recieved request from the reverse proxy
validated: A list of active nodes
*/
func ForwardOperation(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {

	clientReqBody := string(ctx.Request.Body())

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
		body := string(res.Body())
		ctx.Response.AppendBodyString(body)
	} else {
		ctx.SetStatusCode(fasthttp.StatusServiceUnavailable)
		ctx.Response.AppendBodyString("All nodes are inactive right now")
	}

	util.ValidatedChannel <- validated
}

func HandleGetPeers(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {
	util.ValidatedChannel <- validated
	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.Response.AppendBodyString(serializeValidated(validated))
}

func serializeValidated(validated []util.AddressInformation) string {
	retString := "["
	for idx, server := range validated {
		server.HttpAddr = strings.Replace(server.HttpAddr, ":8002", ":8003", 1)
		if idx != len(validated)-1 {
			retString += server.HttpAddr + ","
		} else {
			retString += server.HttpAddr + "]"
		}
	}

	return retString
}

// Helper function to act as a request client
func _handleFileOperation(ctx *fasthttp.RequestCtx, ipAddr string, clientReqBody string) (fasthttp.Response, error) {
	req := fasthttp.AcquireRequest()
	fmt.Println(string(ctx.Path()))

	if string(ctx.Path()) != "/get_blocks" && string(ctx.Path()) != "/get_pool_amt" {
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
func getAvailableServer(validated []util.AddressInformation) (string, string, int) {
	var chosenServer string
	var randomIndex int
	var err string
	err = ""

	if len(validated) > 1 {
		randomIndex = rand.Intn(len(validated) - 1)
		chosenServer = validated[randomIndex].HttpAddr
	} else if len(validated) == 1 {
		chosenServer = validated[0].HttpAddr
	} else {
		err = "No active servers right now"
	}

	return err, chosenServer, randomIndex
}

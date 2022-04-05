package forwarder

import (
	"fmt"
	"math/rand"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

/*
Forwards a request from the reverse proxy to a linked node

ctx: The context of the recieved request from the reverse proxy
validated: A list of active nodes
*/
func ForwardOperation(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {

	// Original getting of variables
	uri := string(ctx.Request.Body())
	serverErr, ipAddr, idx := getAvailableServer(validated)
	err, res := _handleFileOperation(ctx, ipAddr.HttpAddr, uri)

	// Keeps going until either an active server is found or no servers remain
	for err != nil && serverErr == "" {
		validated = util.Remove(validated, idx)
		serverErr, ipAddr, idx = getAvailableServer(validated)
		err, res = _handleFileOperation(ctx, ipAddr.HttpAddr, uri)
	}

	// If there is no server err return content
	if serverErr == "" {
		ctx.SetStatusCode(fasthttp.StatusOK)
		ctx.Response.AppendBodyString(string(res.Body()))
	} else {
		ctx.SetStatusCode(fasthttp.StatusServiceUnavailable)
		ctx.Response.AppendBodyString("All nodes are inactive right now")
	}

	util.ValidatedChannel <- validated
}

// Helper function to act as a request client
func _handleFileOperation(ctx *fasthttp.RequestCtx, ipAddr string, uri string) (error, fasthttp.Response) {

	req := fasthttp.AcquireRequest()

	if string(ctx.Path()) != "/get_blocks" {
		req.Header.SetMethod(fasthttp.MethodPost)
	} else {
		req.Header.SetMethod(fasthttp.MethodGet)
	}

	req.AppendBodyString(uri)
	req.SetRequestURI(ipAddr + string(ctx.Path()))

	res := fasthttp.AcquireResponse()

	err := util.Client.Do(req, res)

	if err != nil {
		fmt.Println(err)
	}

	return err, *res
}

// Gets an active server at random
func getAvailableServer(validated []util.AddressInformation) (string, util.AddressInformation, int) {
	var chosenServer util.AddressInformation
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

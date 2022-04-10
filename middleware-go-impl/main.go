package main

import (
	"fmt"
	"vidur2/middleware/forwarder"
	hostappend "vidur2/middleware/host_append"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

var validated []string

func handler(ctx *fasthttp.RequestCtx) {

	switch string(ctx.Path()) {

	case "/get_peers":
		go forwarder.HandleGetPeers(ctx, validated)
		validated = <-util.ValidatedChannel

	case "/add_self_as_peer":
		go hostappend.HandleAddSelf(ctx, validated)
		validated = <-util.ValidatedChannel

	case "/get_information_by_url":
		go forwarder.ForwardOperation(ctx, validated)
		validated = <-util.ValidatedChannel

	case "/store_information":
		go forwarder.ForwardOperation(ctx, validated)
		validated = <-util.ValidatedChannel

	case "/get_blocks":
		go forwarder.ForwardOperation(ctx, validated)
		validated = <-util.ValidatedChannel

	default:
		ctx.Response.SetStatusCode(fasthttp.StatusNotFound)
		ctx.Response.AppendBodyString("Invalid Path")
	}

}

func main() {
	util.InitClient()
	util.InitChannels()
	fmt.Println("Server listening on 'http://localhost:8080'")
	fasthttp.ListenAndServe(":8080", handler)
}

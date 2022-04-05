package main

import (
	"fmt"
	clientside "vidur2/middleware/client_side"
	hostappend "vidur2/middleware/host_append"
	peercheck "vidur2/middleware/peer_check"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

var validated []util.AddressInformation

func handler(ctx *fasthttp.RequestCtx) {

	switch string(ctx.Path()) {

	case "/get_peers":
		peercheck.HandleGetPeers(ctx, validated)
		fmt.Println(validated)

	case "/add_self_as_peer":
		validated = hostappend.HandleAddSelf(ctx, validated)

	case "/get_information_by_url":
		validated = clientside.HandleFileOperation(ctx, validated)

	case "/store_information":
		validated = clientside.HandleFileOperation(ctx, validated)

	case "/get_blocks":
		validated = clientside.HandleFileOperation(ctx, validated)

	default:
		ctx.Response.SetStatusCode(fasthttp.StatusNotFound)
		ctx.Response.AppendBodyString("Invalid Path")
	}

}

func main() {
	util.InitClient()
	fmt.Println("Server listening on 'http://localhost:8080'")
	fasthttp.ListenAndServe(":8080", handler)
}

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
	util.InitValidatedRecv()

	switch string(ctx.Path()) {
	case "/get_peers":
		go peercheck.HandleGetPeers(ctx, validated)

	case "/add_self_as_peer":
		go hostappend.HandleAddSelf(ctx, validated)
		validated = <-util.ValidatedRecv

	case "/get_information_by_url":
		go clientside.HandleFileOperation(ctx, validated)
		validated = <-util.ValidatedRecv
	case "/store_information":
		go clientside.HandleFileOperation(ctx, validated)
		validated = <-util.ValidatedRecv

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

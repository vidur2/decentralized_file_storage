package main

import (
	"fmt"
	"vidur2/middleware/forwarder"
	gatewayconn "vidur2/middleware/gateway_conn"
	hostappend "vidur2/middleware/host_append"
	peercheck "vidur2/middleware/peer_check"
	"vidur2/middleware/util"

	"github.com/dgrr/fastws"
	"github.com/valyala/fasthttp"
)

var validated []util.AddressInformation

func handler(ctx *fasthttp.RequestCtx) {

	switch string(ctx.Path()) {

	case "/get_peers":
		go peercheck.HandleGetPeersSocket(ctx)

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
		fmt.Println(validated)

	default:
		ctx.Response.SetStatusCode(fasthttp.StatusNotFound)
		ctx.Response.AppendBodyString("Invalid Path")
	}

}

func wsHandler(conn *fastws.Conn) {
	go gatewayconn.HandleNewWs(conn)
}

func main() {
	util.InitClient()
	util.InitChannel()
	go fasthttp.ListenAndServe(":8081", fastws.Upgrade(wsHandler))
	go fasthttp.ListenAndServe(":8080", handler)
	fmt.Println("Http server listening on 'ws://localhost:8080'")
	fmt.Println("Socket server listening on 'http://localhost:8081'")
	for {
	}
}

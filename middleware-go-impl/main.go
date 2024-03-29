package main

import (
	"fmt"
	"strconv"
	"time"
	"vidur2/middleware/forwarder"
	gatewayconn "vidur2/middleware/gateway_conn"
	hostappend "vidur2/middleware/host_append"
	removehost "vidur2/middleware/remove_host"
	"vidur2/middleware/util"

	"github.com/dgrr/fastws"
	"github.com/valyala/fasthttp"
)

var validated []util.AddressInformation

func handler(ctx *fasthttp.RequestCtx) {

	switch string(ctx.Path()) {

	case "/get_peers":
		go forwarder.HandleGetPeers(ctx, validated)
		validated = <-util.ValidatedChannel
		fmt.Println(validated)

	case "/add_self_as_peer":
		go hostappend.HandleAddSelf(ctx, validated)
		validated = <-util.ValidatedChannel
		fmt.Println(validated)

	case "/get_information_by_url":
		go forwarder.ForwardOperation(ctx, validated)
		validated = <-util.ValidatedChannel
		fmt.Println(validated)

	case "/store_information":
		go forwarder.ForwardOperation(ctx, validated)
		validated = <-util.ValidatedChannel
		fmt.Println(validated)

	case "/get_blocks":
		go forwarder.ForwardOperation(ctx, validated)
		validated = <-util.ValidatedChannel
		fmt.Println(validated)

	case "/get_balance":
		go forwarder.ForwardOperation(ctx, validated)
		validated = <-util.ValidatedChannel
		fmt.Println(validated)

	case "/get_amt_nodes":
		ctx.Response.AppendBodyString(strconv.FormatInt(int64(len(validated)), 10))

	case "/get_public_keys":
		forwarder.HandleGetPublicKeys(ctx, validated)

	case "/remove_ip_addr":
		removehost.HandleRemoveHost(ctx, validated)
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
	go util.InitChannel()
	time.Sleep(1000000)
	go fasthttp.ListenAndServe(":8081", fastws.Upgrade(wsHandler))
	go fasthttp.ListenAndServe(":8080", handler)
	fmt.Println("Http server listening on 'ws://localhost:8080'")
	fmt.Println("Socket server listening on 'http://localhost:8081'")
	for {
	}
}

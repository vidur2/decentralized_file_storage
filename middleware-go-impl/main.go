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
		validated = forwarder.ForwardOperation(ctx, validated)
		fmt.Println(validated)

	case "/add_self_as_peer":
		validated = hostappend.HandleAddSelf(ctx, validated)

	case "/get_information_by_url":
		validated = forwarder.ForwardOperation(ctx, validated)
		fmt.Println(validated)

	case "/store_information":
		validated = forwarder.ForwardOperation(ctx, validated)
		fmt.Println(validated)

	case "/get_blocks":
		validated = forwarder.ForwardOperation(ctx, validated)
		fmt.Println(validated)

	default:
		ctx.Response.SetStatusCode(fasthttp.StatusNotFound)
		ctx.Response.AppendBodyString("Invalid Path")
	}

}

func main() {
	var mode string
	util.InitClient()
	fmt.Println("Enter the mode you are running the webserver on (local/global): ")
	fmt.Scanln(&mode)
	if mode == "local" {
		util.Port = ":8002"
	} else {
		util.Port = ""
	}
	fmt.Println("Server listening on 'http://localhost:8080'")
	fasthttp.ListenAndServe(":8080", handler)
}

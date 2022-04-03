package main

import (
	"fmt"
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

	case "/add_self_as_peer":
		validated = hostappend.HandleAddSelf(ctx, validated)
	}

}

func main() {
	fmt.Println("Server listening on 'http://localhost:8080'")
	fasthttp.ListenAndServe(":8080", handler)
}

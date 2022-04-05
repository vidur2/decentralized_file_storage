package peercheck

import (
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

// Gives requestser a slice of type AddressInformation containing all addresses in blockchain
//
// Note one must run a node to be part of this list
func HandleGetPeersSocket(ctx *fasthttp.RequestCtx) {
	validated := <-util.ValidatedChannel
	hostname := string(ctx.Request.Body())
	valid := checkIfValid(validated, hostname)

	if valid {
		stringified := serializeList(validated, hostname)
		ctx.SetStatusCode(fasthttp.StatusOK)
		ctx.Response.AppendBodyString(stringified)
	} else {
		ctx.SetStatusCode(fasthttp.StatusBadRequest)
	}
}

// Checks if requester is in list
func checkIfValid(validated []util.AddressInformation, hostname string) bool {
	inList := false

	for _, host := range validated {
		if host.SocketAddr == hostname {
			inList = true
			break
		}
	}

	return inList
}

// Serializes list as string to be returned through a response
func serializeList(validated []util.AddressInformation, currentHostname string) string {
	returnType := ""

	for idx, host := range validated {
		if host.SocketAddr == currentHostname {
			continue
		} else if idx != len(validated)-1 {
			returnType += host.SocketAddr + ","
		} else {
			returnType += host.SocketAddr
		}
	}

	return returnType
}

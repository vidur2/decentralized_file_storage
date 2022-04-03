package peercheck

import (
	"fmt"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

func HandleGetPeers(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {
	hostname := string(ctx.Request.Body())
	valid := checkIfValid(validated, hostname)

	if valid {
		stringified := serializeList(validated, hostname)
		ctx.SetStatusCode(fasthttp.StatusOK)
		fmt.Println(stringified)
		ctx.Response.AppendBodyString(stringified)
	} else {
		ctx.SetStatusCode(fasthttp.StatusBadRequest)
	}
}

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

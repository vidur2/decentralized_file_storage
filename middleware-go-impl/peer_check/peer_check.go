package peercheck

import (
	"github.com/valyala/fasthttp"
)

func checkIfValid(ctx *fasthttp.RequestCtx, validated []string) bool {
	hostname := string(ctx.Request.Header.Host())
	inList := false

	for _, host := range validated {
		if host == hostname {
			inList = true
			break
		}
	}

	return inList
}

func HandleGetPeers(ctx *fasthttp.RequestCtx, validated []string) {
	valid := checkIfValid(ctx, validated)

	if valid {
		ctx.SetStatusCode(fasthttp.StatusOK)
		ctx.Response.AppendBody([]byte(serializeList(validated)))
	} else {
		ctx.SetStatusCode(fasthttp.StatusBadRequest)
	}
}

func serializeList(validated []string) string {
	returnType := "["

	for _, host := range validated {
		returnType += host
	}

	returnType += "]"

	return returnType
}

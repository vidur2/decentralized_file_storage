package peercheck

import (
	"vidur2/middleware/util"

	"encoding/json"

	"github.com/valyala/fasthttp"
)

func HandleGetPeersHttpAndConnect(ctx *fasthttp.RequestCtx, validated util.GatwayInitInfor) {
	asJson, err := json.Marshal(validated)

	if err != nil {
		ctx.SetStatusCode(fasthttp.StatusOK)
		ctx.Response.AppendBody(asJson)
	} else {
		ctx.SetStatusCode(fasthttp.StatusBadRequest)
	}
}

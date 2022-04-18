package get_node

import (
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

func HandleGetNode(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {
	stringified := serializeValidated(validated)
	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.Response.AppendBodyString(stringified)
}

func serializeValidated(validated []util.AddressInformation) string {
	retString := "["
	for idx, server := range validated {
		if idx != len(validated)-1 {
			retString += server.PublicKey + ","
		} else {
			retString += server.PublicKey + "]"
		}
	}
	return retString
}

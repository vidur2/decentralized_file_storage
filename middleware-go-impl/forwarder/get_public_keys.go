package forwarder

import (
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

func HandleGetPublicKeys(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {
	retValue := serializePublicKeys(validated)
	ctx.Response.AppendBodyString(retValue)
}

func serializePublicKeys(validated []util.AddressInformation) string {
	retString := "["
	for idx, server := range validated {
		public_key := server.PublicKey
		if idx != len(validated)-1 {
			retString += public_key + ","
		} else {
			retString += public_key + "]"
		}
	}

	return retString
}

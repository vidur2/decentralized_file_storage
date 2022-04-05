package gatewayconn

import (
	"github.com/dgrr/fastws"
	"github.com/valyala/fasthttp"
)

func HandleWsRequest(ctx *fasthttp.RequestCtx) {
	handler := fastws.Upgrade( , ctx)
}

package hostappend

import (
	"fmt"
	"strconv"
	"vidur2/middleware/util"

	realip "github.com/Ferluci/fast-realip"
	"github.com/valyala/fasthttp"
)

// Adds ip addr to list if it passes the nessescary testing
func HandleAddSelf(ctx *fasthttp.RequestCtx, validated []string) []string {
	clientIp := realip.FromRequest(ctx)
	fmt.Println(clientIp)

	valid := testHost("http://" + clientIp + util.Port)

	if valid {
		validated = append(validated, clientIp+util.Port)
	}

	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.Response.AppendBodyString(strconv.FormatBool(valid))

	return validated
}

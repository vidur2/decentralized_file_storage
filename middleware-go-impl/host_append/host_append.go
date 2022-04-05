package hostappend

import (
	"encoding/json"
	"fmt"
	"strconv"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

// Adds ip addr to list if it passes the nessescary testing
func HandleAddSelf(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {
	var ipInformation util.AddressInformation
	err := json.Unmarshal(ctx.Request.Body(), &ipInformation)

	if err == nil {
		fmt.Println(ipInformation)
		valid := TestHost(ipInformation.HttpAddr)

		if valid {
			validated = append(validated, ipInformation)
		}
		fmt.Println(strconv.FormatBool(valid))
		ctx.SetStatusCode(fasthttp.StatusOK)
		ctx.Response.AppendBodyString(strconv.FormatBool(valid))
		util.ValidatedChannel <- validated
	}
}

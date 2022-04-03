package hostappend

import (
	"encoding/json"
	"fmt"
	"strconv"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

func HandleAddSelf(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) []util.AddressInformation {
	var ipInformation util.AddressInformation
	err := json.Unmarshal(ctx.Request.Body(), &ipInformation)

	if err == nil {
		fmt.Println(ipInformation)
		valid := testHost(ipInformation.HttpAddr)

		if valid {
			validated = append(validated, ipInformation)
		}

		ctx.Response.AppendBodyString(strconv.FormatBool(valid))

		return validated
	} else {
		return nil
	}
}

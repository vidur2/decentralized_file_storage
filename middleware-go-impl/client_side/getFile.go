package clientside

import (
	"math/rand"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

func HandleFileOperation(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) []util.AddressInformation {
	uri := string(ctx.Request.Body())
	ipAddr, idx := getAvailableServer(validated)
	err, res := _handleFileOperation(ctx, ipAddr.HttpAddr, uri)

	for err != nil {
		validated = util.Remove(validated, idx)
		ipAddr, idx = getAvailableServer(validated)
		err, res = _handleFileOperation(ctx, ipAddr.HttpAddr, uri)
	}

	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.Response.AppendBodyString(string(res.Body()))

	return validated
}

func _handleFileOperation(ctx *fasthttp.RequestCtx, ipAddr string, uri string) (error, fasthttp.Response) {

	req := fasthttp.AcquireRequest()
	req.Header.SetMethod(fasthttp.MethodPost)
	req.AppendBodyString(uri)
	req.SetRequestURI(ipAddr + string(ctx.Path()))

	res := fasthttp.AcquireResponse()

	err := util.Client.Do(req, res)

	return err, *res
}

func getAvailableServer(validated []util.AddressInformation) (util.AddressInformation, int) {

	var chosenServer util.AddressInformation
	var randomIndex int

	if len(validated) > 1 {
		randomIndex = rand.Intn(len(validated) - 1)
		chosenServer = validated[randomIndex]
	} else {
		chosenServer = validated[0]
	}

	return chosenServer, randomIndex
}

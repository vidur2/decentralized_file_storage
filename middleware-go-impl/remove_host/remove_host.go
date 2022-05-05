package removehost

import (
	"crypto/ed25519"
	"encoding/json"
	"fmt"
	"strconv"
	"strings"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

type RequestFormat struct {
	Timestamp string `json:"timestamp"`
	Signature string `json:"signature"`
	PublicKey string `json:"public_key"`
}

func HandleRemoveHost(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {
	var parsedBody RequestFormat
	fmt.Println(string(ctx.Request.Body()))
	err := json.Unmarshal(ctx.Request.Body(), &parsedBody)

	if err == nil {
		ctx.Response.SetStatusCode(fasthttp.StatusOK)
		valid := ed25519.Verify(stringToArray(parsedBody.PublicKey), []byte(parsedBody.Timestamp), stringToArray(parsedBody.Signature))
		if valid {
			validated = removeBasedOnPk(parsedBody.PublicKey, validated)
			fmt.Println(validated)
		}
		ctx.Response.AppendBodyString("good")
	} else {
		ctx.Response.SetStatusCode(fasthttp.StatusBadRequest)
		ctx.Response.AppendBodyString("bad")
	}

	util.ValidatedChannel <- validated
}

func stringToArray(str string) []byte {
	str = strings.Replace(str, "[", "", 1)
	str = strings.Replace(str, "]", "", 1)
	str = strings.ReplaceAll(str, " ", "")
	stringArr := strings.Split(str, ",")
	byteArr := make([]uint8, len(stringArr))

	for idx, strNum := range stringArr {
		num, err := strconv.Atoi(strNum)
		if err == nil {
			byteArr[idx] = uint8(num)
		}
	}

	return byteArr
}

func removeBasedOnPk(publicKey string, validated []util.AddressInformation) []util.AddressInformation {
	for idx, pk := range validated {
		if pk.PublicKey == publicKey {
			validated[idx] = validated[len(validated)-1]
			return validated[:len(validated)-1]
		}
	}

	return validated
}

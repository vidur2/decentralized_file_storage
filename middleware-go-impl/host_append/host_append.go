package hostappend

import (
	"encoding/json"
	"fmt"
	"strconv"
	"time"
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

var client *fasthttp.Client

type FileInformation struct {
	Data      string `json:"data"`
	LinkedUri string `json:"linked_uri"`
	Creator   string `json:"creator"`
	Version   string `json:"version"`
	FileType  string `json:"file_type"`
}

func generateRandomFileInformation() FileInformation {
	return FileInformation{
		Data:      util.RandSeq(8),
		LinkedUri: util.RandSeq(8),
		Creator:   util.RandSeq(8),
		Version:   util.RandSeq(8),
		FileType:  "Frontend",
	}
}

func initClient() {
	readTimeout, _ := time.ParseDuration("500ms")
	writeTimeout, _ := time.ParseDuration("500ms")
	maxIdleConnDuration, _ := time.ParseDuration("1h")

	client = &fasthttp.Client{
		ReadTimeout:                   readTimeout,
		WriteTimeout:                  writeTimeout,
		MaxIdleConnDuration:           maxIdleConnDuration,
		NoDefaultUserAgentHeader:      true, // Don't send: User-Agent: fasthttp
		DisableHeaderNamesNormalizing: true, // If you set the case on your headers correctly you can enable this
		DisablePathNormalizing:        true,
		// increase DNS cache time to an hour instead of default minute
		Dial: (&fasthttp.TCPDialer{
			Concurrency:      4096,
			DNSCacheDuration: time.Hour,
		}).Dial,
	}
}

func testHost(url string) bool {
	initClient()
	req := fasthttp.AcquireRequest()
	req.Header.SetMethod(fasthttp.MethodPost)
	req.SetRequestURI(url + "/store_information")
	checkedFileInf := generateRandomFileInformation()
	fileInfAsString, _ := json.Marshal(checkedFileInf)
	req.AppendBody(fileInfAsString)
	res := fasthttp.AcquireResponse()

	err := client.Do(req, res)

	if err != nil {
		fmt.Println(err)
		return false
	} else {
		req.SetRequestURI(url + "/get_information_by_url")
		req.SetBody([]byte(checkedFileInf.LinkedUri))
		res := fasthttp.AcquireResponse()

		err := client.Do(req, res)

		if err != nil {
			return false
		} else {
			if checkedFileInf.Data == string(res.Body()) {
				return true
			} else {
				return false
			}
		}
	}
}

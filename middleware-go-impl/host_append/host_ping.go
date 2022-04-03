package hostappend

import (
	"encoding/json"
	"fmt"
	"time"

	"github.com/valyala/fasthttp"
)

var client *fasthttp.Client

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

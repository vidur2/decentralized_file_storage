package hostappend

import (
	"encoding/json"
	"fmt"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

// Tests thte host for functional api routes
func TestHost(url string) bool {

	// Builds the request
	req := fasthttp.AcquireRequest()
	req.Header.SetMethod(fasthttp.MethodPost)
	req.SetRequestURI(url + "/store_information")
	checkedFileInf := generateRandomFileInformation()
	fileInfAsString, _ := json.Marshal(checkedFileInf)
	fmt.Println(string(fileInfAsString))
	req.AppendBody(fileInfAsString)
	res := fasthttp.AcquireResponse()

	// Makes request
	err := util.Client.Do(req, res)

	// Parses response
	if err != nil {
		return false
	} else {
		req.SetRequestURI(url + "/get_information_by_url")
		req.SetBody([]byte(checkedFileInf.LinkedUri))
		res := fasthttp.AcquireResponse()

		err := util.Client.Do(req, res)

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

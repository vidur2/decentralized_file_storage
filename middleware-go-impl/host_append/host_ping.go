package hostappend

import (
	"encoding/json"
	"fmt"
	"vidur2/middleware/util"

	"github.com/valyala/fasthttp"
)

func testHost(url string) bool {
	req := fasthttp.AcquireRequest()
	req.Header.SetMethod(fasthttp.MethodPost)
	req.SetRequestURI(url + "/store_information")
	checkedFileInf := generateRandomFileInformation()
	fileInfAsString, _ := json.Marshal(checkedFileInf)
	req.AppendBody(fileInfAsString)
	res := fasthttp.AcquireResponse()

	err := util.Client.Do(req, res)

	if err != nil {
		fmt.Println(err)
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

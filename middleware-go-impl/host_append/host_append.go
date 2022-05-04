package hostappend

import (
	"encoding/json"
	"fmt"
	"strconv"
	"vidur2/middleware/util"

	realip "github.com/Ferluci/fast-realip"
	"github.com/valyala/fasthttp"
)

// Represents a websocket message
//
// Used for de/serialization
//
// Fields
//  * Path: used to identify which handler to use
//    - "/add_node"
//    - "/add_gateway"
type MessageType struct {
	Path          string
	IpInformation string
}

// Adds ip addr to list if it passes the nessescary testing
//
// Params:
//  * ctx: fasthttp context object
//  * validated: slice containing all active nodes
func HandleAddSelf(ctx *fasthttp.RequestCtx, validated []util.AddressInformation) {
	ipInformation := realip.FromRequest(ctx) + ":8002"
	check1 := !checkIfInList(validated, ipInformation)
	fmt.Println(ipInformation)
	check2 := TestHost("http://" + ipInformation)
	fmt.Println(check1 && check2)
	if check1 && check2 {
		validated = append(validated, util.AddressInformation{
			HttpAddr:  ipInformation,
			PublicKey: string(ctx.Request.Body()),
		})
		handleAdd(ipInformation)
	} else {
		fmt.Println("SHITTTT!")
	}
	fmt.Println(strconv.FormatBool(check2 && check1))
	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.Response.AppendBodyString(strconv.FormatBool(check2 && check1))
	util.ValidatedChannel <- validated

}

// Broadcasts the adding of a node over ip
func handleAdd(ipInformation string) {
	nonblocking := <-util.SocketsNonNil

	if nonblocking {
		sockets := <-util.SocketsChannel
		message, err := json.Marshal(MessageType{
			Path:          "/add_node",
			IpInformation: ipInformation,
		})

		if err == nil {
			for _, socket := range sockets {
				socket.Write(message)
			}
		}
	}
}

func checkIfInList(validated []util.AddressInformation, url string) bool {
	for _, server := range validated {
		if server.HttpAddr == url {
			return true
		}
	}
	return false
}

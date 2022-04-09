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
func HandleAddSelf(ctx *fasthttp.RequestCtx, validated []string) {
	ipInformation := realip.FromRequest(ctx) + ":8002"

	fmt.Println(ipInformation)
	valid := TestHost("http://" + ipInformation)
	if valid {
		validated = append(validated, ipInformation)
		handleAdd(ipInformation)
	}
	fmt.Println(strconv.FormatBool(valid))
	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.Response.AppendBodyString(strconv.FormatBool(valid))
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

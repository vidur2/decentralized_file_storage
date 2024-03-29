package gatewayconn

import (
	"encoding/json"
	"vidur2/middleware/util"

	"github.com/dgrr/fastws"
)

func HandleNewWs(conn *fastws.Conn) {
	nonblocking := <-util.SocketsNonNil
	var sockets []*fastws.Conn

	if !nonblocking {
		util.SocketsNonNil <- true
		sockets = make([]*fastws.Conn, 1)
	} else {
		sockets = <-util.SocketsChannel
	}
	broadcastAddGateway(sockets, *conn)
	sockets = append(sockets, conn)
	util.SocketsChannel <- sockets
	var messageInfor MessageType
	var validated []util.AddressInformation

	for {
		_, msg, err := conn.ReadMessage(nil)

		if err == nil {
			err := json.Unmarshal(msg, &messageInfor)

			if err == nil {
				if messageInfor.Path == "/add_gateway" {
					conn, err := fastws.Dial(messageInfor.IpInformation.HttpAddr)

					if err == nil {
						go HandleNewWs(conn)
					}
				} else if messageInfor.Path == "/add_node" {
					valid := true

					if valid {
						validated = <-util.ValidatedChannel
						validated = append(validated, messageInfor.IpInformation)
						util.ValidatedChannel <- validated
					}
				}
			}
		}
	}
}

func broadcastAddGateway(sockets []*fastws.Conn, newConn fastws.Conn) {
	for _, socket := range sockets {
		msg, err := json.Marshal(MessageType{
			Path: "/add_gateway",
			IpInformation: util.AddressInformation{
				HttpAddr: newConn.RemoteAddr().String(),
			},
		})

		if err == nil {
			socket.Write(msg)
		}
	}
}

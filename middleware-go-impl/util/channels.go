package util

import "github.com/dgrr/fastws"

var ValidatedChannel chan ([]AddressInformation)
var SocketsChannel chan ([]*fastws.Conn)

func InitChannel() {
	ValidatedChannel = make(chan []AddressInformation)
	SocketsChannel = make(chan []*fastws.Conn)
}

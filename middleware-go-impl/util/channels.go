package util

import (
	"github.com/dgrr/fastws"
)

var ValidatedChannel chan ([]AddressInformation)
var SocketsChannel chan ([]*fastws.Conn)
var SocketsNonNil chan (bool)

func InitChannel() {
	ValidatedChannel = make(chan []AddressInformation)
	SocketsChannel = make(chan []*fastws.Conn)
	SocketsNonNil = make(chan bool)
	SocketsNonNil <- false
}

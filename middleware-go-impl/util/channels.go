package util

import "github.com/dgrr/fastws"

var ValidatedChannel chan ([]string)
var SocketsChannel chan ([]*fastws.Conn)
var SocketsNonNil chan (bool)

func InitChannel() {
	ValidatedChannel = make(chan []string)
	SocketsChannel = make(chan []*fastws.Conn)
	SocketsNonNil = make(chan bool)
	SocketsNonNil <- false
}

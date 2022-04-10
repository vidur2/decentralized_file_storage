package util

var ValidatedChannel chan []string

func InitChannels() {
	ValidatedChannel = make(chan []string)
}

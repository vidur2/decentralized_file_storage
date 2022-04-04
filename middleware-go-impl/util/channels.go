package util

var ValidatedRecv chan ([]AddressInformation)

func InitValidatedRecv() {
	ValidatedRecv = make(chan []AddressInformation)
}

package util

// Contains information about ip addr
type AddressInformation struct {
	SocketAddr string `json:"socket_addr"`
	HttpAddr   string `json:"http_addr"`
}

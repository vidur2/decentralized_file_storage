package util

func Remove(slice []AddressInformation, idx int) []AddressInformation {
	slice[idx] = slice[len(slice)-1]
	slice[len(slice)-1] = AddressInformation{}
	slice = slice[:len(slice)-1]
	return slice
}

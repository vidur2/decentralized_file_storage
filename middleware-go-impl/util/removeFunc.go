package util

// Function to remove an element from a list
func Remove(slice []string, idx int) []string {
	slice[idx] = slice[len(slice)-1]
	slice[len(slice)-1] = ""
	slice = slice[:len(slice)-1]
	return slice
}

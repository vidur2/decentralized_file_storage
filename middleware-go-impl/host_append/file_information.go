package hostappend

import "vidur2/middleware/util"

type FileInformation struct {
	Data      string `json:"data"`
	LinkedUri string `json:"linked_uri"`
	Creator   string `json:"creator"`
	Version   string `json:"version"`
	FileType  string `json:"file_type"`
}

func generateRandomFileInformation() FileInformation {
	return FileInformation{
		Data:      util.RandSeq(8),
		LinkedUri: util.RandSeq(8),
		Creator:   util.RandSeq(8),
		Version:   util.RandSeq(8),
		FileType:  "Frontend",
	}
}

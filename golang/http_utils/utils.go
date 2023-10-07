package utils

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
)

func HandleResponse(response *http.Response, v any) error {
	defer response.Body.Close()
	if response.StatusCode > 299 {
		content, err := io.ReadAll(response.Body)
		if err != nil {
			return err
		}
		if len(content) == 0 {
			return errors.New(fmt.Sprintf("Response: %v+", response))
		}
		return errors.New(string(content))
	}
	return json.NewDecoder(response.Body).Decode(v)
}

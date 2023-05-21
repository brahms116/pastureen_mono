package prpc_core

import (
  "encoding/json"
  "testing"
)

func TestPRPCRequest(t *testing.T) {
  jsonStr := `{"auth":null,"command":"test","params":["hello","world"]}`
  var req PRPCRequest[[]string]
  json.Unmarshal([]byte(jsonStr), &req)
  
  if req.Command != "test" {
    t.Fatalf("Command is not test")
  }

  if req.Auth != nil {
    t.Fatalf("Auth is not nil")
  }

  if len(req.Params) != 2 {
    t.Fatalf("Params length is not 2")
  }

  if req.Params[0] != "hello" {
    t.Fatalf("Params[0] is not hello")
  }

  if req.Params[1] != "world" {
    t.Fatalf("Params[1] is not world")
  }
}

func TestPRPCResponseResult(t *testing.T) {
  results := []string{"hello", "world"}
  res := PRPCResponse[[]string]{
    Result: &results,
    Error: nil,
  }

  jsonStr, err := json.Marshal(res)

  if err != nil {
    t.Fatalf("Marshal error: %s", err)
  }

  if string(jsonStr) != `{"result":["hello","world"],"error":null}` {
    t.Fatalf("Marshal result error: %s", string(jsonStr))
  }
}

func TestPRPCResponseError(t *testing.T) {
  res := PRPCResponse[[]string]{
    Result: nil,
    Error: &PRPCError{
      Code: PRPC_ERROR_CODE_UNKNOWN,
      Message: "test",
    },
  }

  jsonStr, err := json.Marshal(res)

  if err != nil {
    t.Fatalf("Marshal error: %s", err)
  }

  if string(jsonStr) != `{"result":null,"error":{"code":1,"message":"test"}}` {
    t.Fatalf("Marshal error error: %s", string(jsonStr))
  }
}

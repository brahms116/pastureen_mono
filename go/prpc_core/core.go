package prpc_core

const (
  PRPC_ERROR_CODE_CANCELLED = 1
  PRPC_ERROR_CODE_UNKNOWN = 2
  PRPC_ERROR_CODE_INVALID_ARGUMENT = 3
  PRPC_ERROR_CODE_DEADLINE_EXCEEDED = 4
  PRPC_ERROR_CODE_NOT_FOUND = 5
  PRPC_ERROR_CODE_ALREADY_EXISTS = 6
  PRPC_ERROR_CODE_PERMISSION_DENIED = 7
  PRPC_ERROR_CODE_RESOURCE_EXHAUSTED = 8
  PRPC_ERROR_CODE_FAILED_PRECONDITION = 9
  PRPC_ERROR_CODE_ABORTED = 10
  PRPC_ERROR_CODE_OUT_OF_RANGE = 11
  PRPC_ERROR_CODE_UNIMPLEMENTED = 12
  PRPC_ERROR_CODE_INTERNAL = 13
  PRPC_ERROR_CODE_UNAVAILABLE = 14
  PRPC_ERROR_CODE_DATA_LOSS = 15
  PRPC_ERROR_CODE_UNAUTHENTICATED = 16
)



type PRPCRequest[T any] struct {
	Auth    *string `json:"auth"`
	Command string `json:"command"`
	Params  T      `json:"params"`
}

type PRPCError struct {
	Code    uint8 `json:"code"`
	Message string `json:"message"`
}

type PRPCResponse[T any] struct {
	Result *T       `json:"result"`
	Error  *PRPCError `json:"error"`
}


func (response *PRPCResponse[T]) IsValid() bool {
  return (response.Error != nil || response.Result != nil)  
}

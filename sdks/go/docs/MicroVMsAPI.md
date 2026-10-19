# \MicroVMsAPI

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteApiV1VmmByMachineName**](MicroVMsAPI.md#DeleteApiV1VmmByMachineName) | **Delete** /api/v1/vmm/{machine_name} | Delete a microVM
[**GetApiV1Vmm**](MicroVMsAPI.md#GetApiV1Vmm) | **Get** /api/v1/vmm | List microVMs
[**GetApiV1VmmByMachineName**](MicroVMsAPI.md#GetApiV1VmmByMachineName) | **Get** /api/v1/vmm/{machine_name} | Get a microVM
[**PostApiV1VmmByMachineName**](MicroVMsAPI.md#PostApiV1VmmByMachineName) | **Post** /api/v1/vmm/{machine_name} | Create a microVM
[**PostApiV1VmmByMachineNameClone**](MicroVMsAPI.md#PostApiV1VmmByMachineNameClone) | **Post** /api/v1/vmm/{machine_name}/clone | Clone a microVM
[**PutApiV1VmmByMachineName**](MicroVMsAPI.md#PutApiV1VmmByMachineName) | **Put** /api/v1/vmm/{machine_name} | Resize a microVM
[**PutApiV1VmmByMachineNamePause**](MicroVMsAPI.md#PutApiV1VmmByMachineNamePause) | **Put** /api/v1/vmm/{machine_name}/pause | Pause a microVM
[**PutApiV1VmmByMachineNameRestart**](MicroVMsAPI.md#PutApiV1VmmByMachineNameRestart) | **Put** /api/v1/vmm/{machine_name}/restart | Restart a microVM
[**PutApiV1VmmByMachineNameRestore**](MicroVMsAPI.md#PutApiV1VmmByMachineNameRestore) | **Put** /api/v1/vmm/{machine_name}/restore | Restore a microVM from a snapshot
[**PutApiV1VmmByMachineNameShutdown**](MicroVMsAPI.md#PutApiV1VmmByMachineNameShutdown) | **Put** /api/v1/vmm/{machine_name}/shutdown | Shut down a microVM
[**PutApiV1VmmByMachineNameSnapshot**](MicroVMsAPI.md#PutApiV1VmmByMachineNameSnapshot) | **Put** /api/v1/vmm/{machine_name}/snapshot | Snapshot a microVM
[**PutApiV1VmmByMachineNameStart**](MicroVMsAPI.md#PutApiV1VmmByMachineNameStart) | **Put** /api/v1/vmm/{machine_name}/start | Start a microVM



## DeleteApiV1VmmByMachineName

> ApiSuccess DeleteApiV1VmmByMachineName(ctx, machineName).Execute()

Delete a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.DeleteApiV1VmmByMachineName(context.Background(), machineName).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.DeleteApiV1VmmByMachineName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteApiV1VmmByMachineName`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.DeleteApiV1VmmByMachineName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteApiV1VmmByMachineNameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetApiV1Vmm

> []ApiVm GetApiV1Vmm(ctx).Execute()

List microVMs



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.GetApiV1Vmm(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.GetApiV1Vmm``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1Vmm`: []ApiVm
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.GetApiV1Vmm`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1VmmRequest struct via the builder pattern


### Return type

[**[]ApiVm**](ApiVm.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetApiV1VmmByMachineName

> ApiVmGetResponse GetApiV1VmmByMachineName(ctx, machineName).Execute()

Get a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.GetApiV1VmmByMachineName(context.Background(), machineName).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.GetApiV1VmmByMachineName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1VmmByMachineName`: ApiVmGetResponse
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.GetApiV1VmmByMachineName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1VmmByMachineNameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiVmGetResponse**](ApiVmGetResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1VmmByMachineName

> ApiVmCreateResponse PostApiV1VmmByMachineName(ctx, machineName).ApiVmCreateRequest(apiVmCreateRequest).Execute()

Create a microVM



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.
	apiVmCreateRequest := *openapiclient.NewApiVmCreateRequest([]string{"CloudinitUserData_example"}, "PublicKey_example", int32(123), int32(123), int32(123)) // ApiVmCreateRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PostApiV1VmmByMachineName(context.Background(), machineName).ApiVmCreateRequest(apiVmCreateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PostApiV1VmmByMachineName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1VmmByMachineName`: ApiVmCreateResponse
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PostApiV1VmmByMachineName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1VmmByMachineNameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiVmCreateRequest** | [**ApiVmCreateRequest**](ApiVmCreateRequest.md) |  | 

### Return type

[**ApiVmCreateResponse**](ApiVmCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1VmmByMachineNameClone

> ApiVmCloneResponse PostApiV1VmmByMachineNameClone(ctx, machineName).ApiVmCloneRequest(apiVmCloneRequest).Execute()

Clone a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.
	apiVmCloneRequest := *openapiclient.NewApiVmCloneRequest("NewName_example") // ApiVmCloneRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PostApiV1VmmByMachineNameClone(context.Background(), machineName).ApiVmCloneRequest(apiVmCloneRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PostApiV1VmmByMachineNameClone``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1VmmByMachineNameClone`: ApiVmCloneResponse
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PostApiV1VmmByMachineNameClone`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1VmmByMachineNameCloneRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiVmCloneRequest** | [**ApiVmCloneRequest**](ApiVmCloneRequest.md) |  | 

### Return type

[**ApiVmCloneResponse**](ApiVmCloneResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1VmmByMachineName

> ApiSuccess PutApiV1VmmByMachineName(ctx, machineName).ApiVmUpdateRequest(apiVmUpdateRequest).Execute()

Resize a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.
	apiVmUpdateRequest := *openapiclient.NewApiVmUpdateRequest() // ApiVmUpdateRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PutApiV1VmmByMachineName(context.Background(), machineName).ApiVmUpdateRequest(apiVmUpdateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PutApiV1VmmByMachineName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1VmmByMachineName`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PutApiV1VmmByMachineName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1VmmByMachineNameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiVmUpdateRequest** | [**ApiVmUpdateRequest**](ApiVmUpdateRequest.md) |  | 

### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1VmmByMachineNamePause

> ApiSuccess PutApiV1VmmByMachineNamePause(ctx, machineName).Execute()

Pause a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PutApiV1VmmByMachineNamePause(context.Background(), machineName).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PutApiV1VmmByMachineNamePause``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1VmmByMachineNamePause`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PutApiV1VmmByMachineNamePause`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1VmmByMachineNamePauseRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1VmmByMachineNameRestart

> ApiSuccess PutApiV1VmmByMachineNameRestart(ctx, machineName).Execute()

Restart a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PutApiV1VmmByMachineNameRestart(context.Background(), machineName).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PutApiV1VmmByMachineNameRestart``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1VmmByMachineNameRestart`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PutApiV1VmmByMachineNameRestart`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1VmmByMachineNameRestartRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1VmmByMachineNameRestore

> ApiVmRestoreResponse PutApiV1VmmByMachineNameRestore(ctx, machineName).ApiVmRestoreRequest(apiVmRestoreRequest).Execute()

Restore a microVM from a snapshot



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.
	apiVmRestoreRequest := *openapiclient.NewApiVmRestoreRequest("SourceUrl_example") // ApiVmRestoreRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PutApiV1VmmByMachineNameRestore(context.Background(), machineName).ApiVmRestoreRequest(apiVmRestoreRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PutApiV1VmmByMachineNameRestore``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1VmmByMachineNameRestore`: ApiVmRestoreResponse
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PutApiV1VmmByMachineNameRestore`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1VmmByMachineNameRestoreRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiVmRestoreRequest** | [**ApiVmRestoreRequest**](ApiVmRestoreRequest.md) |  | 

### Return type

[**ApiVmRestoreResponse**](ApiVmRestoreResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1VmmByMachineNameShutdown

> ApiSuccess PutApiV1VmmByMachineNameShutdown(ctx, machineName).Execute()

Shut down a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PutApiV1VmmByMachineNameShutdown(context.Background(), machineName).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PutApiV1VmmByMachineNameShutdown``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1VmmByMachineNameShutdown`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PutApiV1VmmByMachineNameShutdown`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1VmmByMachineNameShutdownRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1VmmByMachineNameSnapshot

> ApiVmSnapshotResponse PutApiV1VmmByMachineNameSnapshot(ctx, machineName).ApiVmSnapshotRequest(apiVmSnapshotRequest).Execute()

Snapshot a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.
	apiVmSnapshotRequest := *openapiclient.NewApiVmSnapshotRequest() // ApiVmSnapshotRequest |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PutApiV1VmmByMachineNameSnapshot(context.Background(), machineName).ApiVmSnapshotRequest(apiVmSnapshotRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PutApiV1VmmByMachineNameSnapshot``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1VmmByMachineNameSnapshot`: ApiVmSnapshotResponse
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PutApiV1VmmByMachineNameSnapshot`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1VmmByMachineNameSnapshotRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiVmSnapshotRequest** | [**ApiVmSnapshotRequest**](ApiVmSnapshotRequest.md) |  | 

### Return type

[**ApiVmSnapshotResponse**](ApiVmSnapshotResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1VmmByMachineNameStart

> ApiSuccess PutApiV1VmmByMachineNameStart(ctx, machineName).Execute()

Start a microVM

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	machineName := "machineName_example" // string | Name of the microVM.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.MicroVMsAPI.PutApiV1VmmByMachineNameStart(context.Background(), machineName).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MicroVMsAPI.PutApiV1VmmByMachineNameStart``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1VmmByMachineNameStart`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `MicroVMsAPI.PutApiV1VmmByMachineNameStart`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**machineName** | **string** | Name of the microVM. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1VmmByMachineNameStartRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


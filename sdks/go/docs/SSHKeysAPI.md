# \SSHKeysAPI

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteApiV1KeysById**](SSHKeysAPI.md#DeleteApiV1KeysById) | **Delete** /api/v1/keys/{id} | Delete an SSH key
[**GetApiV1Keys**](SSHKeysAPI.md#GetApiV1Keys) | **Get** /api/v1/keys | List SSH keys
[**PostApiV1Keys**](SSHKeysAPI.md#PostApiV1Keys) | **Post** /api/v1/keys | Add an SSH key
[**PutApiV1KeysById**](SSHKeysAPI.md#PutApiV1KeysById) | **Put** /api/v1/keys/{id} | Rename an SSH key



## DeleteApiV1KeysById

> ApiSuccess DeleteApiV1KeysById(ctx, id).Execute()

Delete an SSH key

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
	id := "id_example" // string | Id of the SSH key.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SSHKeysAPI.DeleteApiV1KeysById(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SSHKeysAPI.DeleteApiV1KeysById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteApiV1KeysById`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `SSHKeysAPI.DeleteApiV1KeysById`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Id of the SSH key. | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteApiV1KeysByIdRequest struct via the builder pattern


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


## GetApiV1Keys

> []ApiSshKey GetApiV1Keys(ctx).Execute()

List SSH keys

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
	resp, r, err := apiClient.SSHKeysAPI.GetApiV1Keys(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SSHKeysAPI.GetApiV1Keys``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1Keys`: []ApiSshKey
	fmt.Fprintf(os.Stdout, "Response from `SSHKeysAPI.GetApiV1Keys`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1KeysRequest struct via the builder pattern


### Return type

[**[]ApiSshKey**](ApiSshKey.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1Keys

> ApiSshKeyCreateResponse PostApiV1Keys(ctx).ApiSshKeyCreateRequest(apiSshKeyCreateRequest).Execute()

Add an SSH key



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
	apiSshKeyCreateRequest := *openapiclient.NewApiSshKeyCreateRequest("Type_example") // ApiSshKeyCreateRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SSHKeysAPI.PostApiV1Keys(context.Background()).ApiSshKeyCreateRequest(apiSshKeyCreateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SSHKeysAPI.PostApiV1Keys``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1Keys`: ApiSshKeyCreateResponse
	fmt.Fprintf(os.Stdout, "Response from `SSHKeysAPI.PostApiV1Keys`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1KeysRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **apiSshKeyCreateRequest** | [**ApiSshKeyCreateRequest**](ApiSshKeyCreateRequest.md) |  | 

### Return type

[**ApiSshKeyCreateResponse**](ApiSshKeyCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1KeysById

> ApiSuccess PutApiV1KeysById(ctx, id).ApiSshKeyUpdateRequest(apiSshKeyUpdateRequest).Execute()

Rename an SSH key



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
	id := "id_example" // string | Id of the SSH key.
	apiSshKeyUpdateRequest := *openapiclient.NewApiSshKeyUpdateRequest() // ApiSshKeyUpdateRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SSHKeysAPI.PutApiV1KeysById(context.Background(), id).ApiSshKeyUpdateRequest(apiSshKeyUpdateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SSHKeysAPI.PutApiV1KeysById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1KeysById`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `SSHKeysAPI.PutApiV1KeysById`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Id of the SSH key. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1KeysByIdRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiSshKeyUpdateRequest** | [**ApiSshKeyUpdateRequest**](ApiSshKeyUpdateRequest.md) |  | 

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


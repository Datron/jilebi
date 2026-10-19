# \APIKeysAPI

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteApiV1ApiKeysById**](APIKeysAPI.md#DeleteApiV1ApiKeysById) | **Delete** /api/v1/api-keys/{id} | Revoke an API key
[**GetApiV1ApiKeys**](APIKeysAPI.md#GetApiV1ApiKeys) | **Get** /api/v1/api-keys | List API keys
[**PostApiV1ApiKeys**](APIKeysAPI.md#PostApiV1ApiKeys) | **Post** /api/v1/api-keys | Create an API key



## DeleteApiV1ApiKeysById

> ApiSuccess DeleteApiV1ApiKeysById(ctx, id).Execute()

Revoke an API key

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
	id := "id_example" // string | Id of the API key.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.APIKeysAPI.DeleteApiV1ApiKeysById(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `APIKeysAPI.DeleteApiV1ApiKeysById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteApiV1ApiKeysById`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `APIKeysAPI.DeleteApiV1ApiKeysById`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Id of the API key. | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteApiV1ApiKeysByIdRequest struct via the builder pattern


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


## GetApiV1ApiKeys

> []ApiKey GetApiV1ApiKeys(ctx).Execute()

List API keys

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
	resp, r, err := apiClient.APIKeysAPI.GetApiV1ApiKeys(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `APIKeysAPI.GetApiV1ApiKeys``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1ApiKeys`: []ApiKey
	fmt.Fprintf(os.Stdout, "Response from `APIKeysAPI.GetApiV1ApiKeys`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1ApiKeysRequest struct via the builder pattern


### Return type

[**[]ApiKey**](ApiKey.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1ApiKeys

> ApiKeyCreateResponse PostApiV1ApiKeys(ctx).ApiKeyCreateRequest(apiKeyCreateRequest).Execute()

Create an API key



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
	apiKeyCreateRequest := *openapiclient.NewApiKeyCreateRequest("Name_example", []string{"Scopes_example"}) // ApiKeyCreateRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.APIKeysAPI.PostApiV1ApiKeys(context.Background()).ApiKeyCreateRequest(apiKeyCreateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `APIKeysAPI.PostApiV1ApiKeys``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1ApiKeys`: ApiKeyCreateResponse
	fmt.Fprintf(os.Stdout, "Response from `APIKeysAPI.PostApiV1ApiKeys`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1ApiKeysRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **apiKeyCreateRequest** | [**ApiKeyCreateRequest**](ApiKeyCreateRequest.md) |  | 

### Return type

[**ApiKeyCreateResponse**](ApiKeyCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


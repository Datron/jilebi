# \SoftwareCatalogAPI

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteApiV1SoftwareById**](SoftwareCatalogAPI.md#DeleteApiV1SoftwareById) | **Delete** /api/v1/software/{id} | Delete software
[**GetApiV1Software**](SoftwareCatalogAPI.md#GetApiV1Software) | **Get** /api/v1/software | List software
[**GetApiV1SoftwareById**](SoftwareCatalogAPI.md#GetApiV1SoftwareById) | **Get** /api/v1/software/{id} | Get software
[**PostApiV1Software**](SoftwareCatalogAPI.md#PostApiV1Software) | **Post** /api/v1/software | Add software
[**PutApiV1SoftwareById**](SoftwareCatalogAPI.md#PutApiV1SoftwareById) | **Put** /api/v1/software/{id} | Update software



## DeleteApiV1SoftwareById

> ApiSuccess DeleteApiV1SoftwareById(ctx, id).Execute()

Delete software

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
	id := "id_example" // string | Id of the catalog entry.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SoftwareCatalogAPI.DeleteApiV1SoftwareById(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SoftwareCatalogAPI.DeleteApiV1SoftwareById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteApiV1SoftwareById`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `SoftwareCatalogAPI.DeleteApiV1SoftwareById`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Id of the catalog entry. | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteApiV1SoftwareByIdRequest struct via the builder pattern


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


## GetApiV1Software

> []ApiSoftware GetApiV1Software(ctx).Execute()

List software

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
	resp, r, err := apiClient.SoftwareCatalogAPI.GetApiV1Software(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SoftwareCatalogAPI.GetApiV1Software``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1Software`: []ApiSoftware
	fmt.Fprintf(os.Stdout, "Response from `SoftwareCatalogAPI.GetApiV1Software`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1SoftwareRequest struct via the builder pattern


### Return type

[**[]ApiSoftware**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetApiV1SoftwareById

> ApiSoftware GetApiV1SoftwareById(ctx, id).Execute()

Get software

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
	id := "id_example" // string | Id of the catalog entry.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SoftwareCatalogAPI.GetApiV1SoftwareById(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SoftwareCatalogAPI.GetApiV1SoftwareById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1SoftwareById`: ApiSoftware
	fmt.Fprintf(os.Stdout, "Response from `SoftwareCatalogAPI.GetApiV1SoftwareById`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Id of the catalog entry. | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1SoftwareByIdRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiSoftware**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1Software

> ApiSoftware PostApiV1Software(ctx).ApiSoftwareCreateRequest(apiSoftwareCreateRequest).Execute()

Add software

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
	apiSoftwareCreateRequest := *openapiclient.NewApiSoftwareCreateRequest("Id_example", "Name_example", "DisplayName_example", "InstallCommand_example") // ApiSoftwareCreateRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SoftwareCatalogAPI.PostApiV1Software(context.Background()).ApiSoftwareCreateRequest(apiSoftwareCreateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SoftwareCatalogAPI.PostApiV1Software``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1Software`: ApiSoftware
	fmt.Fprintf(os.Stdout, "Response from `SoftwareCatalogAPI.PostApiV1Software`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1SoftwareRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **apiSoftwareCreateRequest** | [**ApiSoftwareCreateRequest**](ApiSoftwareCreateRequest.md) |  | 

### Return type

[**ApiSoftware**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1SoftwareById

> ApiSuccess PutApiV1SoftwareById(ctx, id).ApiSoftwareUpdateRequest(apiSoftwareUpdateRequest).Execute()

Update software

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
	id := "id_example" // string | Id of the catalog entry.
	apiSoftwareUpdateRequest := *openapiclient.NewApiSoftwareUpdateRequest() // ApiSoftwareUpdateRequest |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SoftwareCatalogAPI.PutApiV1SoftwareById(context.Background(), id).ApiSoftwareUpdateRequest(apiSoftwareUpdateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SoftwareCatalogAPI.PutApiV1SoftwareById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1SoftwareById`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `SoftwareCatalogAPI.PutApiV1SoftwareById`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Id of the catalog entry. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1SoftwareByIdRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiSoftwareUpdateRequest** | [**ApiSoftwareUpdateRequest**](ApiSoftwareUpdateRequest.md) |  | 

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


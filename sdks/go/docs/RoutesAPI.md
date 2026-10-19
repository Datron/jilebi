# \RoutesAPI

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteApiV1RoutesByName**](RoutesAPI.md#DeleteApiV1RoutesByName) | **Delete** /api/v1/routes/{name} | Delete a route
[**GetApiV1Routes**](RoutesAPI.md#GetApiV1Routes) | **Get** /api/v1/routes | List routes
[**PostApiV1Routes**](RoutesAPI.md#PostApiV1Routes) | **Post** /api/v1/routes | Create a route
[**PutApiV1RoutesByName**](RoutesAPI.md#PutApiV1RoutesByName) | **Put** /api/v1/routes/{name} | Update a route



## DeleteApiV1RoutesByName

> ApiSuccess DeleteApiV1RoutesByName(ctx, name).Execute()

Delete a route

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
	name := "name_example" // string | Name of the route.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.RoutesAPI.DeleteApiV1RoutesByName(context.Background(), name).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `RoutesAPI.DeleteApiV1RoutesByName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteApiV1RoutesByName`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `RoutesAPI.DeleteApiV1RoutesByName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**name** | **string** | Name of the route. | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteApiV1RoutesByNameRequest struct via the builder pattern


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


## GetApiV1Routes

> ApiRouteListResponse GetApiV1Routes(ctx).Execute()

List routes



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
	resp, r, err := apiClient.RoutesAPI.GetApiV1Routes(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `RoutesAPI.GetApiV1Routes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1Routes`: ApiRouteListResponse
	fmt.Fprintf(os.Stdout, "Response from `RoutesAPI.GetApiV1Routes`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1RoutesRequest struct via the builder pattern


### Return type

[**ApiRouteListResponse**](ApiRouteListResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1Routes

> ApiRouteCreateResponse PostApiV1Routes(ctx).ApiRouteCreateRequest(apiRouteCreateRequest).Execute()

Create a route

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
	apiRouteCreateRequest := *openapiclient.NewApiRouteCreateRequest("Name_example", "MicrovmId_example", int32(123), "Privacy_example") // ApiRouteCreateRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.RoutesAPI.PostApiV1Routes(context.Background()).ApiRouteCreateRequest(apiRouteCreateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `RoutesAPI.PostApiV1Routes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1Routes`: ApiRouteCreateResponse
	fmt.Fprintf(os.Stdout, "Response from `RoutesAPI.PostApiV1Routes`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1RoutesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **apiRouteCreateRequest** | [**ApiRouteCreateRequest**](ApiRouteCreateRequest.md) |  | 

### Return type

[**ApiRouteCreateResponse**](ApiRouteCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1RoutesByName

> ApiSuccess PutApiV1RoutesByName(ctx, name).ApiRouteUpdateRequest(apiRouteUpdateRequest).Execute()

Update a route

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
	name := "name_example" // string | Name of the route.
	apiRouteUpdateRequest := *openapiclient.NewApiRouteUpdateRequest() // ApiRouteUpdateRequest |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.RoutesAPI.PutApiV1RoutesByName(context.Background(), name).ApiRouteUpdateRequest(apiRouteUpdateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `RoutesAPI.PutApiV1RoutesByName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1RoutesByName`: ApiSuccess
	fmt.Fprintf(os.Stdout, "Response from `RoutesAPI.PutApiV1RoutesByName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**name** | **string** | Name of the route. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1RoutesByNameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiRouteUpdateRequest** | [**ApiRouteUpdateRequest**](ApiRouteUpdateRequest.md) |  | 

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


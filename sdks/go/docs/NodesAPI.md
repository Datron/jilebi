# \NodesAPI

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteApiV1NodesByName**](NodesAPI.md#DeleteApiV1NodesByName) | **Delete** /api/v1/nodes/{name} | Delete a node
[**GetApiV1Nodes**](NodesAPI.md#GetApiV1Nodes) | **Get** /api/v1/nodes | List nodes
[**GetApiV1NodesByName**](NodesAPI.md#GetApiV1NodesByName) | **Get** /api/v1/nodes/{name} | Get a node
[**PostApiV1Nodes**](NodesAPI.md#PostApiV1Nodes) | **Post** /api/v1/nodes | Register a node
[**PostApiV1NodesByNameHealth**](NodesAPI.md#PostApiV1NodesByNameHealth) | **Post** /api/v1/nodes/{name}/health | Check node health
[**PostApiV1NodesByNameRefresh**](NodesAPI.md#PostApiV1NodesByNameRefresh) | **Post** /api/v1/nodes/{name}/refresh | Refresh node resources
[**PostApiV1NodesByNameReinitialize**](NodesAPI.md#PostApiV1NodesByNameReinitialize) | **Post** /api/v1/nodes/{name}/reinitialize | Reinitialize a node
[**PutApiV1NodesByName**](NodesAPI.md#PutApiV1NodesByName) | **Put** /api/v1/nodes/{name} | Update a node



## DeleteApiV1NodesByName

> ApiNode DeleteApiV1NodesByName(ctx, name).Execute()

Delete a node

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
	name := "name_example" // string | Name of the node.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.NodesAPI.DeleteApiV1NodesByName(context.Background(), name).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `NodesAPI.DeleteApiV1NodesByName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteApiV1NodesByName`: ApiNode
	fmt.Fprintf(os.Stdout, "Response from `NodesAPI.DeleteApiV1NodesByName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**name** | **string** | Name of the node. | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteApiV1NodesByNameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetApiV1Nodes

> []ApiNode GetApiV1Nodes(ctx).Execute()

List nodes

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
	resp, r, err := apiClient.NodesAPI.GetApiV1Nodes(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `NodesAPI.GetApiV1Nodes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1Nodes`: []ApiNode
	fmt.Fprintf(os.Stdout, "Response from `NodesAPI.GetApiV1Nodes`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1NodesRequest struct via the builder pattern


### Return type

[**[]ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetApiV1NodesByName

> ApiNode GetApiV1NodesByName(ctx, name).Execute()

Get a node

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
	name := "name_example" // string | Name of the node.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.NodesAPI.GetApiV1NodesByName(context.Background(), name).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `NodesAPI.GetApiV1NodesByName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1NodesByName`: ApiNode
	fmt.Fprintf(os.Stdout, "Response from `NodesAPI.GetApiV1NodesByName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**name** | **string** | Name of the node. | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1NodesByNameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1Nodes

> ApiNode PostApiV1Nodes(ctx).ApiNodeCreateRequest(apiNodeCreateRequest).Execute()

Register a node



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
	apiNodeCreateRequest := *openapiclient.NewApiNodeCreateRequest("Name_example", "Url_example") // ApiNodeCreateRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.NodesAPI.PostApiV1Nodes(context.Background()).ApiNodeCreateRequest(apiNodeCreateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `NodesAPI.PostApiV1Nodes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1Nodes`: ApiNode
	fmt.Fprintf(os.Stdout, "Response from `NodesAPI.PostApiV1Nodes`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1NodesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **apiNodeCreateRequest** | [**ApiNodeCreateRequest**](ApiNodeCreateRequest.md) |  | 

### Return type

[**ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1NodesByNameHealth

> ApiNodeHealthResponse PostApiV1NodesByNameHealth(ctx, name).Execute()

Check node health

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
	name := "name_example" // string | Name of the node.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.NodesAPI.PostApiV1NodesByNameHealth(context.Background(), name).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `NodesAPI.PostApiV1NodesByNameHealth``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1NodesByNameHealth`: ApiNodeHealthResponse
	fmt.Fprintf(os.Stdout, "Response from `NodesAPI.PostApiV1NodesByNameHealth`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**name** | **string** | Name of the node. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1NodesByNameHealthRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiNodeHealthResponse**](ApiNodeHealthResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1NodesByNameRefresh

> ApiNodeRefreshResponse PostApiV1NodesByNameRefresh(ctx, name).Execute()

Refresh node resources



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
	name := "name_example" // string | Name of the node.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.NodesAPI.PostApiV1NodesByNameRefresh(context.Background(), name).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `NodesAPI.PostApiV1NodesByNameRefresh``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1NodesByNameRefresh`: ApiNodeRefreshResponse
	fmt.Fprintf(os.Stdout, "Response from `NodesAPI.PostApiV1NodesByNameRefresh`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**name** | **string** | Name of the node. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1NodesByNameRefreshRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiNodeRefreshResponse**](ApiNodeRefreshResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1NodesByNameReinitialize

> ApiNodeReinitializeResponse PostApiV1NodesByNameReinitialize(ctx, name).Execute()

Reinitialize a node



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
	name := "name_example" // string | Name of the node.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.NodesAPI.PostApiV1NodesByNameReinitialize(context.Background(), name).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `NodesAPI.PostApiV1NodesByNameReinitialize``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1NodesByNameReinitialize`: ApiNodeReinitializeResponse
	fmt.Fprintf(os.Stdout, "Response from `NodesAPI.PostApiV1NodesByNameReinitialize`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**name** | **string** | Name of the node. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1NodesByNameReinitializeRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiNodeReinitializeResponse**](ApiNodeReinitializeResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutApiV1NodesByName

> ApiNode PutApiV1NodesByName(ctx, name).ApiNodeUpdateRequest(apiNodeUpdateRequest).Execute()

Update a node

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
	name := "name_example" // string | 
	apiNodeUpdateRequest := *openapiclient.NewApiNodeUpdateRequest() // ApiNodeUpdateRequest |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.NodesAPI.PutApiV1NodesByName(context.Background(), name).ApiNodeUpdateRequest(apiNodeUpdateRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `NodesAPI.PutApiV1NodesByName``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutApiV1NodesByName`: ApiNode
	fmt.Fprintf(os.Stdout, "Response from `NodesAPI.PutApiV1NodesByName`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**name** | **string** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiPutApiV1NodesByNameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **apiNodeUpdateRequest** | [**ApiNodeUpdateRequest**](ApiNodeUpdateRequest.md) |  | 

### Return type

[**ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


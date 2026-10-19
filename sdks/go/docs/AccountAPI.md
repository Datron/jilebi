# \AccountAPI

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetApiMe**](AccountAPI.md#GetApiMe) | **Get** /api/me | Get the current user and session



## GetApiMe

> ApiMeResponse GetApiMe(ctx).Execute()

Get the current user and session

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
	resp, r, err := apiClient.AccountAPI.GetApiMe(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AccountAPI.GetApiMe``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiMe`: ApiMeResponse
	fmt.Fprintf(os.Stdout, "Response from `AccountAPI.GetApiMe`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiMeRequest struct via the builder pattern


### Return type

[**ApiMeResponse**](ApiMeResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


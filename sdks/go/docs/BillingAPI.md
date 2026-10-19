# \BillingAPI

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetApiV1BillingAdminUsers**](BillingAPI.md#GetApiV1BillingAdminUsers) | **Get** /api/v1/billing/admin/users | List billable users
[**PostApiV1BillingAdminGenerateByUsername**](BillingAPI.md#PostApiV1BillingAdminGenerateByUsername) | **Post** /api/v1/billing/admin/generate/{username} | Generate an invoice
[**PostApiV1BillingAdminResetDodoByUsername**](BillingAPI.md#PostApiV1BillingAdminResetDodoByUsername) | **Post** /api/v1/billing/admin/reset-dodo/{username} | Reset a user&#39;s Dodo data
[**PostApiV1BillingPayByBillId**](BillingAPI.md#PostApiV1BillingPayByBillId) | **Post** /api/v1/billing/pay/{billId} | Pay an invoice



## GetApiV1BillingAdminUsers

> ApiBillingUsersResponse GetApiV1BillingAdminUsers(ctx).Execute()

List billable users

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
	resp, r, err := apiClient.BillingAPI.GetApiV1BillingAdminUsers(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `BillingAPI.GetApiV1BillingAdminUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiV1BillingAdminUsers`: ApiBillingUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `BillingAPI.GetApiV1BillingAdminUsers`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiV1BillingAdminUsersRequest struct via the builder pattern


### Return type

[**ApiBillingUsersResponse**](ApiBillingUsersResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1BillingAdminGenerateByUsername

> ApiBillingGenerateResponse PostApiV1BillingAdminGenerateByUsername(ctx, username).Start(start).End(end).Execute()

Generate an invoice



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/Datron/jilebi/sdks/go"
)

func main() {
	username := "username_example" // string | 
	start := time.Now() // string | Window start (YYYY-MM-DD). (optional)
	end := time.Now() // string | Window end (YYYY-MM-DD). (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.BillingAPI.PostApiV1BillingAdminGenerateByUsername(context.Background(), username).Start(start).End(end).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `BillingAPI.PostApiV1BillingAdminGenerateByUsername``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1BillingAdminGenerateByUsername`: ApiBillingGenerateResponse
	fmt.Fprintf(os.Stdout, "Response from `BillingAPI.PostApiV1BillingAdminGenerateByUsername`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**username** | **string** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1BillingAdminGenerateByUsernameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **start** | **string** | Window start (YYYY-MM-DD). | 
 **end** | **string** | Window end (YYYY-MM-DD). | 

### Return type

[**ApiBillingGenerateResponse**](ApiBillingGenerateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1BillingAdminResetDodoByUsername

> ApiBillingResetDodoResponse PostApiV1BillingAdminResetDodoByUsername(ctx, username).Execute()

Reset a user's Dodo data

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
	username := "username_example" // string | Username of the account to reset.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.BillingAPI.PostApiV1BillingAdminResetDodoByUsername(context.Background(), username).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `BillingAPI.PostApiV1BillingAdminResetDodoByUsername``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostApiV1BillingAdminResetDodoByUsername`: ApiBillingResetDodoResponse
	fmt.Fprintf(os.Stdout, "Response from `BillingAPI.PostApiV1BillingAdminResetDodoByUsername`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**username** | **string** | Username of the account to reset. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1BillingAdminResetDodoByUsernameRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApiBillingResetDodoResponse**](ApiBillingResetDodoResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostApiV1BillingPayByBillId

> PostApiV1BillingPayByBillId(ctx, billId).Execute()

Pay an invoice



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
	billId := "billId_example" // string | Id of the invoice to pay.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.BillingAPI.PostApiV1BillingPayByBillId(context.Background(), billId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `BillingAPI.PostApiV1BillingPayByBillId``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**billId** | **string** | Id of the invoice to pay. | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostApiV1BillingPayByBillIdRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


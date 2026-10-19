# AccountApi

All URIs are relative to *https://jilebi.ai*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getApiMe**](AccountApi.md#getapime) | **GET** /api/me | Get the current user and session |



## getApiMe

> ApiMeResponse getApiMe()

Get the current user and session

### Example

```ts
import {
  Configuration,
  AccountApi,
} from '@jilebi/sdk';
import type { GetApiMeRequest } from '@jilebi/sdk';

async function example() {
  console.log("🚀 Testing @jilebi/sdk SDK...");
  const config = new Configuration({ 
    // To configure API key authorization: apiKey
    apiKey: "YOUR API KEY",
    // To configure API key authorization: sessionCookie
    apiKey: "YOUR API KEY",
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new AccountApi(config);

  try {
    const data = await api.getApiMe();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**ApiMeResponse**](ApiMeResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The authenticated user and their session. |  -  |
| **401** | Unauthorized |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


# APIKeysApi

All URIs are relative to *https://jilebi.ai*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**deleteApiV1ApiKeysById**](APIKeysApi.md#deleteapiv1apikeysbyid) | **DELETE** /api/v1/api-keys/{id} | Revoke an API key |
| [**getApiV1ApiKeys**](APIKeysApi.md#getapiv1apikeys) | **GET** /api/v1/api-keys | List API keys |
| [**postApiV1ApiKeys**](APIKeysApi.md#postapiv1apikeys) | **POST** /api/v1/api-keys | Create an API key |



## deleteApiV1ApiKeysById

> ApiSuccess deleteApiV1ApiKeysById(id)

Revoke an API key

### Example

```ts
import {
  Configuration,
  APIKeysApi,
} from '@jilebi/sdk';
import type { DeleteApiV1ApiKeysByIdRequest } from '@jilebi/sdk';

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
  const api = new APIKeysApi(config);

  const body = {
    // string | Id of the API key.
    id: id_example,
  } satisfies DeleteApiV1ApiKeysByIdRequest;

  try {
    const data = await api.deleteApiV1ApiKeysById(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | `string` | Id of the API key. | [Defaults to `undefined`] |

### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Revoked. |  -  |
| **401** | Unauthorized |  -  |
| **404** | Key not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1ApiKeys

> Array&lt;ApiKey&gt; getApiV1ApiKeys()

List API keys

### Example

```ts
import {
  Configuration,
  APIKeysApi,
} from '@jilebi/sdk';
import type { GetApiV1ApiKeysRequest } from '@jilebi/sdk';

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
  const api = new APIKeysApi(config);

  try {
    const data = await api.getApiV1ApiKeys();
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

[**Array&lt;ApiKey&gt;**](ApiKey.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The user\&#39;s API keys. |  -  |
| **401** | Unauthorized |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1ApiKeys

> ApiKeyCreateResponse postApiV1ApiKeys(apiKeyCreateRequest)

Create an API key

The plaintext key is returned only once.

### Example

```ts
import {
  Configuration,
  APIKeysApi,
} from '@jilebi/sdk';
import type { PostApiV1ApiKeysRequest } from '@jilebi/sdk';

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
  const api = new APIKeysApi(config);

  const body = {
    // ApiKeyCreateRequest
    apiKeyCreateRequest: ...,
  } satisfies PostApiV1ApiKeysRequest;

  try {
    const data = await api.postApiV1ApiKeys(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **apiKeyCreateRequest** | [ApiKeyCreateRequest](ApiKeyCreateRequest.md) |  | |

### Return type

[**ApiKeyCreateResponse**](ApiKeyCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Key created. |  -  |
| **400** | Invalid name, scopes or expiry |  -  |
| **401** | Unauthorized |  -  |
| **403** | Requested a scope only node admins can grant |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


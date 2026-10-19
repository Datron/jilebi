# SSHKeysApi

All URIs are relative to *https://jilebi.ai*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**deleteApiV1KeysById**](SSHKeysApi.md#deleteapiv1keysbyid) | **DELETE** /api/v1/keys/{id} | Delete an SSH key |
| [**getApiV1Keys**](SSHKeysApi.md#getapiv1keys) | **GET** /api/v1/keys | List SSH keys |
| [**postApiV1Keys**](SSHKeysApi.md#postapiv1keys) | **POST** /api/v1/keys | Add an SSH key |
| [**putApiV1KeysById**](SSHKeysApi.md#putapiv1keysbyid) | **PUT** /api/v1/keys/{id} | Rename an SSH key |



## deleteApiV1KeysById

> ApiSuccess deleteApiV1KeysById(id)

Delete an SSH key

### Example

```ts
import {
  Configuration,
  SSHKeysApi,
} from '@jilebi/sdk';
import type { DeleteApiV1KeysByIdRequest } from '@jilebi/sdk';

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
  const api = new SSHKeysApi(config);

  const body = {
    // string | Id of the SSH key.
    id: id_example,
  } satisfies DeleteApiV1KeysByIdRequest;

  try {
    const data = await api.deleteApiV1KeysById(body);
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
| **id** | `string` | Id of the SSH key. | [Defaults to `undefined`] |

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
| **200** | Deleted. |  -  |
| **401** | Unauthorized |  -  |
| **404** | Key not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1Keys

> Array&lt;ApiSshKey&gt; getApiV1Keys()

List SSH keys

### Example

```ts
import {
  Configuration,
  SSHKeysApi,
} from '@jilebi/sdk';
import type { GetApiV1KeysRequest } from '@jilebi/sdk';

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
  const api = new SSHKeysApi(config);

  try {
    const data = await api.getApiV1Keys();
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

[**Array&lt;ApiSshKey&gt;**](ApiSshKey.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The user\&#39;s SSH keys. |  -  |
| **401** | Unauthorized |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1Keys

> ApiSshKeyCreateResponse postApiV1Keys(apiSshKeyCreateRequest)

Add an SSH key

Upload an existing public key or generate a new ed25519 key pair. Generated private keys are returned once.

### Example

```ts
import {
  Configuration,
  SSHKeysApi,
} from '@jilebi/sdk';
import type { PostApiV1KeysRequest } from '@jilebi/sdk';

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
  const api = new SSHKeysApi(config);

  const body = {
    // ApiSshKeyCreateRequest
    apiSshKeyCreateRequest: ...,
  } satisfies PostApiV1KeysRequest;

  try {
    const data = await api.postApiV1Keys(body);
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
| **apiSshKeyCreateRequest** | [ApiSshKeyCreateRequest](ApiSshKeyCreateRequest.md) |  | |

### Return type

[**ApiSshKeyCreateResponse**](ApiSshKeyCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Key created. |  -  |
| **400** | Missing key or invalid type |  -  |
| **401** | Unauthorized |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1KeysById

> ApiSuccess putApiV1KeysById(id, apiSshKeyUpdateRequest)

Rename an SSH key

Update the label of an SSH key. Send an empty string to clear it.

### Example

```ts
import {
  Configuration,
  SSHKeysApi,
} from '@jilebi/sdk';
import type { PutApiV1KeysByIdRequest } from '@jilebi/sdk';

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
  const api = new SSHKeysApi(config);

  const body = {
    // string | Id of the SSH key.
    id: id_example,
    // ApiSshKeyUpdateRequest
    apiSshKeyUpdateRequest: ...,
  } satisfies PutApiV1KeysByIdRequest;

  try {
    const data = await api.putApiV1KeysById(body);
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
| **id** | `string` | Id of the SSH key. | [Defaults to `undefined`] |
| **apiSshKeyUpdateRequest** | [ApiSshKeyUpdateRequest](ApiSshKeyUpdateRequest.md) |  | |

### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Updated. |  -  |
| **400** | Invalid name |  -  |
| **401** | Unauthorized |  -  |
| **404** | Key not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


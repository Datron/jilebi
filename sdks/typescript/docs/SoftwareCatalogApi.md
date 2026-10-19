# SoftwareCatalogApi

All URIs are relative to *https://jilebi.ai*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**deleteApiV1SoftwareById**](SoftwareCatalogApi.md#deleteapiv1softwarebyid) | **DELETE** /api/v1/software/{id} | Delete software |
| [**getApiV1Software**](SoftwareCatalogApi.md#getapiv1software) | **GET** /api/v1/software | List software |
| [**getApiV1SoftwareById**](SoftwareCatalogApi.md#getapiv1softwarebyid) | **GET** /api/v1/software/{id} | Get software |
| [**postApiV1Software**](SoftwareCatalogApi.md#postapiv1software) | **POST** /api/v1/software | Add software |
| [**putApiV1SoftwareById**](SoftwareCatalogApi.md#putapiv1softwarebyid) | **PUT** /api/v1/software/{id} | Update software |



## deleteApiV1SoftwareById

> ApiSuccess deleteApiV1SoftwareById(id)

Delete software

### Example

```ts
import {
  Configuration,
  SoftwareCatalogApi,
} from '@jilebi/sdk';
import type { DeleteApiV1SoftwareByIdRequest } from '@jilebi/sdk';

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
  const api = new SoftwareCatalogApi(config);

  const body = {
    // string | Id of the catalog entry.
    id: id_example,
  } satisfies DeleteApiV1SoftwareByIdRequest;

  try {
    const data = await api.deleteApiV1SoftwareById(body);
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
| **id** | `string` | Id of the catalog entry. | [Defaults to `undefined`] |

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
| **403** | Forbidden |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1Software

> Array&lt;ApiSoftware&gt; getApiV1Software()

List software

### Example

```ts
import {
  Configuration,
  SoftwareCatalogApi,
} from '@jilebi/sdk';
import type { GetApiV1SoftwareRequest } from '@jilebi/sdk';

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
  const api = new SoftwareCatalogApi(config);

  try {
    const data = await api.getApiV1Software();
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

[**Array&lt;ApiSoftware&gt;**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The software catalog. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1SoftwareById

> ApiSoftware getApiV1SoftwareById(id)

Get software

### Example

```ts
import {
  Configuration,
  SoftwareCatalogApi,
} from '@jilebi/sdk';
import type { GetApiV1SoftwareByIdRequest } from '@jilebi/sdk';

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
  const api = new SoftwareCatalogApi(config);

  const body = {
    // string | Id of the catalog entry.
    id: id_example,
  } satisfies GetApiV1SoftwareByIdRequest;

  try {
    const data = await api.getApiV1SoftwareById(body);
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
| **id** | `string` | Id of the catalog entry. | [Defaults to `undefined`] |

### Return type

[**ApiSoftware**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The catalog entry. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **404** | Not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1Software

> ApiSoftware postApiV1Software(apiSoftwareCreateRequest)

Add software

### Example

```ts
import {
  Configuration,
  SoftwareCatalogApi,
} from '@jilebi/sdk';
import type { PostApiV1SoftwareRequest } from '@jilebi/sdk';

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
  const api = new SoftwareCatalogApi(config);

  const body = {
    // ApiSoftwareCreateRequest
    apiSoftwareCreateRequest: ...,
  } satisfies PostApiV1SoftwareRequest;

  try {
    const data = await api.postApiV1Software(body);
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
| **apiSoftwareCreateRequest** | [ApiSoftwareCreateRequest](ApiSoftwareCreateRequest.md) |  | |

### Return type

[**ApiSoftware**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Created. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1SoftwareById

> ApiSuccess putApiV1SoftwareById(id, apiSoftwareUpdateRequest)

Update software

### Example

```ts
import {
  Configuration,
  SoftwareCatalogApi,
} from '@jilebi/sdk';
import type { PutApiV1SoftwareByIdRequest } from '@jilebi/sdk';

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
  const api = new SoftwareCatalogApi(config);

  const body = {
    // string | Id of the catalog entry.
    id: id_example,
    // ApiSoftwareUpdateRequest (optional)
    apiSoftwareUpdateRequest: ...,
  } satisfies PutApiV1SoftwareByIdRequest;

  try {
    const data = await api.putApiV1SoftwareById(body);
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
| **id** | `string` | Id of the catalog entry. | [Defaults to `undefined`] |
| **apiSoftwareUpdateRequest** | [ApiSoftwareUpdateRequest](ApiSoftwareUpdateRequest.md) |  | [Optional] |

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
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


# RoutesApi

All URIs are relative to *https://jilebi.ai*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**deleteApiV1RoutesByName**](RoutesApi.md#deleteapiv1routesbyname) | **DELETE** /api/v1/routes/{name} | Delete a route |
| [**getApiV1Routes**](RoutesApi.md#getapiv1routes) | **GET** /api/v1/routes | List routes |
| [**postApiV1Routes**](RoutesApi.md#postapiv1routes) | **POST** /api/v1/routes | Create a route |
| [**putApiV1RoutesByName**](RoutesApi.md#putapiv1routesbyname) | **PUT** /api/v1/routes/{name} | Update a route |



## deleteApiV1RoutesByName

> ApiSuccess deleteApiV1RoutesByName(name)

Delete a route

### Example

```ts
import {
  Configuration,
  RoutesApi,
} from '@jilebi/sdk';
import type { DeleteApiV1RoutesByNameRequest } from '@jilebi/sdk';

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
  const api = new RoutesApi(config);

  const body = {
    // string | Name of the route.
    name: name_example,
  } satisfies DeleteApiV1RoutesByNameRequest;

  try {
    const data = await api.deleteApiV1RoutesByName(body);
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
| **name** | `string` | Name of the route. | [Defaults to `undefined`] |

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
| **404** | Route not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1Routes

> ApiRouteListResponse getApiV1Routes()

List routes

Routes owned by the user plus routes shared with them.

### Example

```ts
import {
  Configuration,
  RoutesApi,
} from '@jilebi/sdk';
import type { GetApiV1RoutesRequest } from '@jilebi/sdk';

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
  const api = new RoutesApi(config);

  try {
    const data = await api.getApiV1Routes();
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

[**ApiRouteListResponse**](ApiRouteListResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Owned and shared routes. |  -  |
| **401** | Unauthorized |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1Routes

> ApiRouteCreateResponse postApiV1Routes(apiRouteCreateRequest)

Create a route

### Example

```ts
import {
  Configuration,
  RoutesApi,
} from '@jilebi/sdk';
import type { PostApiV1RoutesRequest } from '@jilebi/sdk';

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
  const api = new RoutesApi(config);

  const body = {
    // ApiRouteCreateRequest
    apiRouteCreateRequest: ...,
  } satisfies PostApiV1RoutesRequest;

  try {
    const data = await api.postApiV1Routes(body);
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
| **apiRouteCreateRequest** | [ApiRouteCreateRequest](ApiRouteCreateRequest.md) |  | |

### Return type

[**ApiRouteCreateResponse**](ApiRouteCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Created. |  -  |
| **400** | Invalid name, port or emails |  -  |
| **401** | Unauthorized |  -  |
| **404** | MicroVM not found |  -  |
| **409** | Route name already used |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1RoutesByName

> ApiSuccess putApiV1RoutesByName(name, apiRouteUpdateRequest)

Update a route

### Example

```ts
import {
  Configuration,
  RoutesApi,
} from '@jilebi/sdk';
import type { PutApiV1RoutesByNameRequest } from '@jilebi/sdk';

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
  const api = new RoutesApi(config);

  const body = {
    // string | Name of the route.
    name: name_example,
    // ApiRouteUpdateRequest (optional)
    apiRouteUpdateRequest: ...,
  } satisfies PutApiV1RoutesByNameRequest;

  try {
    const data = await api.putApiV1RoutesByName(body);
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
| **name** | `string` | Name of the route. | [Defaults to `undefined`] |
| **apiRouteUpdateRequest** | [ApiRouteUpdateRequest](ApiRouteUpdateRequest.md) |  | [Optional] |

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
| **400** | Invalid port, privacy or emails |  -  |
| **401** | Unauthorized |  -  |
| **404** | Route not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


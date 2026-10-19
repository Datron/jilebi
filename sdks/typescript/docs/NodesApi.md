# NodesApi

All URIs are relative to *https://jilebi.ai*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**deleteApiV1NodesByName**](NodesApi.md#deleteapiv1nodesbyname) | **DELETE** /api/v1/nodes/{name} | Delete a node |
| [**getApiV1Nodes**](NodesApi.md#getapiv1nodes) | **GET** /api/v1/nodes | List nodes |
| [**getApiV1NodesByName**](NodesApi.md#getapiv1nodesbyname) | **GET** /api/v1/nodes/{name} | Get a node |
| [**postApiV1Nodes**](NodesApi.md#postapiv1nodes) | **POST** /api/v1/nodes | Register a node |
| [**postApiV1NodesByNameHealth**](NodesApi.md#postapiv1nodesbynamehealth) | **POST** /api/v1/nodes/{name}/health | Check node health |
| [**postApiV1NodesByNameRefresh**](NodesApi.md#postapiv1nodesbynamerefresh) | **POST** /api/v1/nodes/{name}/refresh | Refresh node resources |
| [**postApiV1NodesByNameReinitialize**](NodesApi.md#postapiv1nodesbynamereinitialize) | **POST** /api/v1/nodes/{name}/reinitialize | Reinitialize a node |
| [**putApiV1NodesByName**](NodesApi.md#putapiv1nodesbyname) | **PUT** /api/v1/nodes/{name} | Update a node |



## deleteApiV1NodesByName

> ApiNode deleteApiV1NodesByName(name)

Delete a node

### Example

```ts
import {
  Configuration,
  NodesApi,
} from '@jilebi/sdk';
import type { DeleteApiV1NodesByNameRequest } from '@jilebi/sdk';

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
  const api = new NodesApi(config);

  const body = {
    // string | Name of the node.
    name: name_example,
  } satisfies DeleteApiV1NodesByNameRequest;

  try {
    const data = await api.deleteApiV1NodesByName(body);
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
| **name** | `string` | Name of the node. | [Defaults to `undefined`] |

### Return type

[**ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Deleted node. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **404** | Node not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1Nodes

> Array&lt;ApiNode&gt; getApiV1Nodes()

List nodes

### Example

```ts
import {
  Configuration,
  NodesApi,
} from '@jilebi/sdk';
import type { GetApiV1NodesRequest } from '@jilebi/sdk';

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
  const api = new NodesApi(config);

  try {
    const data = await api.getApiV1Nodes();
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

[**Array&lt;ApiNode&gt;**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | All nodes. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1NodesByName

> ApiNode getApiV1NodesByName(name)

Get a node

### Example

```ts
import {
  Configuration,
  NodesApi,
} from '@jilebi/sdk';
import type { GetApiV1NodesByNameRequest } from '@jilebi/sdk';

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
  const api = new NodesApi(config);

  const body = {
    // string | Name of the node.
    name: name_example,
  } satisfies GetApiV1NodesByNameRequest;

  try {
    const data = await api.getApiV1NodesByName(body);
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
| **name** | `string` | Name of the node. | [Defaults to `undefined`] |

### Return type

[**ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The node. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **404** | Node not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1Nodes

> ApiNode postApiV1Nodes(apiNodeCreateRequest)

Register a node

Probes the node\&#39;s /system endpoint and registers it with its control plane.

### Example

```ts
import {
  Configuration,
  NodesApi,
} from '@jilebi/sdk';
import type { PostApiV1NodesRequest } from '@jilebi/sdk';

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
  const api = new NodesApi(config);

  const body = {
    // ApiNodeCreateRequest
    apiNodeCreateRequest: ...,
  } satisfies PostApiV1NodesRequest;

  try {
    const data = await api.postApiV1Nodes(body);
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
| **apiNodeCreateRequest** | [ApiNodeCreateRequest](ApiNodeCreateRequest.md) |  | |

### Return type

[**ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Registered. |  -  |
| **400** | Missing name or url, or probe failed |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **422** | Node unreachable |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1NodesByNameHealth

> ApiNodeHealthResponse postApiV1NodesByNameHealth(name)

Check node health

### Example

```ts
import {
  Configuration,
  NodesApi,
} from '@jilebi/sdk';
import type { PostApiV1NodesByNameHealthRequest } from '@jilebi/sdk';

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
  const api = new NodesApi(config);

  const body = {
    // string | Name of the node.
    name: name_example,
  } satisfies PostApiV1NodesByNameHealthRequest;

  try {
    const data = await api.postApiV1NodesByNameHealth(body);
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
| **name** | `string` | Name of the node. | [Defaults to `undefined`] |

### Return type

[**ApiNodeHealthResponse**](ApiNodeHealthResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Per-check results. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **404** | Node not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1NodesByNameRefresh

> ApiNodeRefreshResponse postApiV1NodesByNameRefresh(name)

Refresh node resources

Re-probes the node and recomputes available capacity from active microVM allocations.

### Example

```ts
import {
  Configuration,
  NodesApi,
} from '@jilebi/sdk';
import type { PostApiV1NodesByNameRefreshRequest } from '@jilebi/sdk';

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
  const api = new NodesApi(config);

  const body = {
    // string | Name of the node.
    name: name_example,
  } satisfies PostApiV1NodesByNameRefreshRequest;

  try {
    const data = await api.postApiV1NodesByNameRefresh(body);
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
| **name** | `string` | Name of the node. | [Defaults to `undefined`] |

### Return type

[**ApiNodeRefreshResponse**](ApiNodeRefreshResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Refreshed node. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **404** | Node not found |  -  |
| **422** | Node unreachable |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1NodesByNameReinitialize

> ApiNodeReinitializeResponse postApiV1NodesByNameReinitialize(name)

Reinitialize a node

Pushes the stored host info back to the node\&#39;s control plane.

### Example

```ts
import {
  Configuration,
  NodesApi,
} from '@jilebi/sdk';
import type { PostApiV1NodesByNameReinitializeRequest } from '@jilebi/sdk';

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
  const api = new NodesApi(config);

  const body = {
    // string | Name of the node.
    name: name_example,
  } satisfies PostApiV1NodesByNameReinitializeRequest;

  try {
    const data = await api.postApiV1NodesByNameReinitialize(body);
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
| **name** | `string` | Name of the node. | [Defaults to `undefined`] |

### Return type

[**ApiNodeReinitializeResponse**](ApiNodeReinitializeResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Reinitialized. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **404** | Node not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1NodesByName

> ApiNode putApiV1NodesByName(name, apiNodeUpdateRequest)

Update a node

### Example

```ts
import {
  Configuration,
  NodesApi,
} from '@jilebi/sdk';
import type { PutApiV1NodesByNameRequest } from '@jilebi/sdk';

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
  const api = new NodesApi(config);

  const body = {
    // string
    name: name_example,
    // ApiNodeUpdateRequest (optional)
    apiNodeUpdateRequest: ...,
  } satisfies PutApiV1NodesByNameRequest;

  try {
    const data = await api.putApiV1NodesByName(body);
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
| **name** | `string` |  | [Defaults to `undefined`] |
| **apiNodeUpdateRequest** | [ApiNodeUpdateRequest](ApiNodeUpdateRequest.md) |  | [Optional] |

### Return type

[**ApiNode**](ApiNode.md)

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
| **404** | Node not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


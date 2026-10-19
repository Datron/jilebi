# MicroVMsApi

All URIs are relative to *https://jilebi.ai*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**deleteApiV1VmmByMachineName**](MicroVMsApi.md#deleteapiv1vmmbymachinename) | **DELETE** /api/v1/vmm/{machine_name} | Delete a microVM |
| [**getApiV1Vmm**](MicroVMsApi.md#getapiv1vmm) | **GET** /api/v1/vmm | List microVMs |
| [**getApiV1VmmByMachineName**](MicroVMsApi.md#getapiv1vmmbymachinename) | **GET** /api/v1/vmm/{machine_name} | Get a microVM |
| [**postApiV1VmmByMachineName**](MicroVMsApi.md#postapiv1vmmbymachinename) | **POST** /api/v1/vmm/{machine_name} | Create a microVM |
| [**postApiV1VmmByMachineNameClone**](MicroVMsApi.md#postapiv1vmmbymachinenameclone) | **POST** /api/v1/vmm/{machine_name}/clone | Clone a microVM |
| [**putApiV1VmmByMachineName**](MicroVMsApi.md#putapiv1vmmbymachinename) | **PUT** /api/v1/vmm/{machine_name} | Resize a microVM |
| [**putApiV1VmmByMachineNamePause**](MicroVMsApi.md#putapiv1vmmbymachinenamepause) | **PUT** /api/v1/vmm/{machine_name}/pause | Pause a microVM |
| [**putApiV1VmmByMachineNameRestart**](MicroVMsApi.md#putapiv1vmmbymachinenamerestart) | **PUT** /api/v1/vmm/{machine_name}/restart | Restart a microVM |
| [**putApiV1VmmByMachineNameRestore**](MicroVMsApi.md#putapiv1vmmbymachinenamerestore) | **PUT** /api/v1/vmm/{machine_name}/restore | Restore a microVM from a snapshot |
| [**putApiV1VmmByMachineNameShutdown**](MicroVMsApi.md#putapiv1vmmbymachinenameshutdown) | **PUT** /api/v1/vmm/{machine_name}/shutdown | Shut down a microVM |
| [**putApiV1VmmByMachineNameSnapshot**](MicroVMsApi.md#putapiv1vmmbymachinenamesnapshot) | **PUT** /api/v1/vmm/{machine_name}/snapshot | Snapshot a microVM |
| [**putApiV1VmmByMachineNameStart**](MicroVMsApi.md#putapiv1vmmbymachinenamestart) | **PUT** /api/v1/vmm/{machine_name}/start | Start a microVM |



## deleteApiV1VmmByMachineName

> ApiSuccess deleteApiV1VmmByMachineName(machineName)

Delete a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { DeleteApiV1VmmByMachineNameRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
  } satisfies DeleteApiV1VmmByMachineNameRequest;

  try {
    const data = await api.deleteApiV1VmmByMachineName(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |

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
| **404** | MicroVM not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1Vmm

> Array&lt;ApiVm&gt; getApiV1Vmm()

List microVMs

List every microVM owned by the authenticated user.

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { GetApiV1VmmRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  try {
    const data = await api.getApiV1Vmm();
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

[**Array&lt;ApiVm&gt;**](ApiVm.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The user\&#39;s microVMs. |  -  |
| **401** | Unauthorized |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getApiV1VmmByMachineName

> ApiVmGetResponse getApiV1VmmByMachineName(machineName)

Get a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { GetApiV1VmmByMachineNameRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
  } satisfies GetApiV1VmmByMachineNameRequest;

  try {
    const data = await api.getApiV1VmmByMachineName(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |

### Return type

[**ApiVmGetResponse**](ApiVmGetResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The microVM with live hypervisor info. |  -  |
| **401** | Unauthorized |  -  |
| **404** | MicroVM not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1VmmByMachineName

> ApiVmCreateResponse postApiV1VmmByMachineName(machineName, apiVmCreateRequest)

Create a microVM

Provisions a new microVM on a node with sufficient free capacity.

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PostApiV1VmmByMachineNameRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
    // ApiVmCreateRequest
    apiVmCreateRequest: ...,
  } satisfies PostApiV1VmmByMachineNameRequest;

  try {
    const data = await api.postApiV1VmmByMachineName(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |
| **apiVmCreateRequest** | [ApiVmCreateRequest](ApiVmCreateRequest.md) |  | |

### Return type

[**ApiVmCreateResponse**](ApiVmCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Created. |  -  |
| **400** | Invalid size or over quota |  -  |
| **401** | Unauthorized |  -  |
| **402** | Overdue unpaid invoice |  -  |
| **409** | Name taken or no node available |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1VmmByMachineNameClone

> ApiVmCloneResponse postApiV1VmmByMachineNameClone(machineName, apiVmCloneRequest)

Clone a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PostApiV1VmmByMachineNameCloneRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
    // ApiVmCloneRequest
    apiVmCloneRequest: ...,
  } satisfies PostApiV1VmmByMachineNameCloneRequest;

  try {
    const data = await api.postApiV1VmmByMachineNameClone(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |
| **apiVmCloneRequest** | [ApiVmCloneRequest](ApiVmCloneRequest.md) |  | |

### Return type

[**ApiVmCloneResponse**](ApiVmCloneResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Clone created. |  -  |
| **400** | Invalid name or missing SSH key |  -  |
| **401** | Unauthorized |  -  |
| **404** | Source microVM not found |  -  |
| **409** | Name already exists |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1VmmByMachineName

> ApiSuccess putApiV1VmmByMachineName(machineName, apiVmUpdateRequest)

Resize a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PutApiV1VmmByMachineNameRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
    // ApiVmUpdateRequest
    apiVmUpdateRequest: ...,
  } satisfies PutApiV1VmmByMachineNameRequest;

  try {
    const data = await api.putApiV1VmmByMachineName(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |
| **apiVmUpdateRequest** | [ApiVmUpdateRequest](ApiVmUpdateRequest.md) |  | |

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
| **400** | Invalid size |  -  |
| **401** | Unauthorized |  -  |
| **404** | MicroVM not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1VmmByMachineNamePause

> ApiSuccess putApiV1VmmByMachineNamePause(machineName)

Pause a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PutApiV1VmmByMachineNamePauseRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
  } satisfies PutApiV1VmmByMachineNamePauseRequest;

  try {
    const data = await api.putApiV1VmmByMachineNamePause(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |

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
| **200** | Paused. |  -  |
| **400** | MicroVM is not active |  -  |
| **401** | Unauthorized |  -  |
| **404** | MicroVM not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1VmmByMachineNameRestart

> ApiSuccess putApiV1VmmByMachineNameRestart(machineName)

Restart a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PutApiV1VmmByMachineNameRestartRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
  } satisfies PutApiV1VmmByMachineNameRestartRequest;

  try {
    const data = await api.putApiV1VmmByMachineNameRestart(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |

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
| **200** | Restarted. |  -  |
| **400** | MicroVM is not active |  -  |
| **401** | Unauthorized |  -  |
| **404** | MicroVM not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1VmmByMachineNameRestore

> ApiVmRestoreResponse putApiV1VmmByMachineNameRestore(machineName, apiVmRestoreRequest)

Restore a microVM from a snapshot

Restores in place, or creates a new microVM when &#x60;new_name&#x60; is set.

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PutApiV1VmmByMachineNameRestoreRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
    // ApiVmRestoreRequest
    apiVmRestoreRequest: ...,
  } satisfies PutApiV1VmmByMachineNameRestoreRequest;

  try {
    const data = await api.putApiV1VmmByMachineNameRestore(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |
| **apiVmRestoreRequest** | [ApiVmRestoreRequest](ApiVmRestoreRequest.md) |  | |

### Return type

[**ApiVmRestoreResponse**](ApiVmRestoreResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Restored in place. |  -  |
| **201** | Restored into a new microVM. |  -  |
| **400** | Invalid name |  -  |
| **401** | Unauthorized |  -  |
| **404** | MicroVM not found |  -  |
| **409** | Name already exists |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1VmmByMachineNameShutdown

> ApiSuccess putApiV1VmmByMachineNameShutdown(machineName)

Shut down a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PutApiV1VmmByMachineNameShutdownRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
  } satisfies PutApiV1VmmByMachineNameShutdownRequest;

  try {
    const data = await api.putApiV1VmmByMachineNameShutdown(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |

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
| **200** | Shut down. |  -  |
| **400** | MicroVM is not active |  -  |
| **401** | Unauthorized |  -  |
| **404** | MicroVM not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1VmmByMachineNameSnapshot

> ApiVmSnapshotResponse putApiV1VmmByMachineNameSnapshot(machineName, apiVmSnapshotRequest)

Snapshot a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PutApiV1VmmByMachineNameSnapshotRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
    // ApiVmSnapshotRequest (optional)
    apiVmSnapshotRequest: ...,
  } satisfies PutApiV1VmmByMachineNameSnapshotRequest;

  try {
    const data = await api.putApiV1VmmByMachineNameSnapshot(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |
| **apiVmSnapshotRequest** | [ApiVmSnapshotRequest](ApiVmSnapshotRequest.md) |  | [Optional] |

### Return type

[**ApiVmSnapshotResponse**](ApiVmSnapshotResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Snapshot created. |  -  |
| **401** | Unauthorized |  -  |
| **404** | MicroVM not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## putApiV1VmmByMachineNameStart

> ApiSuccess putApiV1VmmByMachineNameStart(machineName)

Start a microVM

### Example

```ts
import {
  Configuration,
  MicroVMsApi,
} from '@jilebi/sdk';
import type { PutApiV1VmmByMachineNameStartRequest } from '@jilebi/sdk';

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
  const api = new MicroVMsApi(config);

  const body = {
    // string | Name of the microVM.
    machineName: machineName_example,
  } satisfies PutApiV1VmmByMachineNameStartRequest;

  try {
    const data = await api.putApiV1VmmByMachineNameStart(body);
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
| **machineName** | `string` | Name of the microVM. | [Defaults to `undefined`] |

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
| **200** | Started, or already active. |  -  |
| **401** | Unauthorized |  -  |
| **402** | Overdue unpaid invoice |  -  |
| **404** | MicroVM not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


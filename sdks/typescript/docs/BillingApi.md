# BillingApi

All URIs are relative to *https://jilebi.ai*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getApiV1BillingAdminUsers**](BillingApi.md#getapiv1billingadminusers) | **GET** /api/v1/billing/admin/users | List billable users |
| [**postApiV1BillingAdminGenerateByUsername**](BillingApi.md#postapiv1billingadmingeneratebyusername) | **POST** /api/v1/billing/admin/generate/{username} | Generate an invoice |
| [**postApiV1BillingAdminResetDodoByUsername**](BillingApi.md#postapiv1billingadminresetdodobyusername) | **POST** /api/v1/billing/admin/reset-dodo/{username} | Reset a user\&#39;s Dodo data |
| [**postApiV1BillingPayByBillId**](BillingApi.md#postapiv1billingpaybybillid) | **POST** /api/v1/billing/pay/{billId} | Pay an invoice |



## getApiV1BillingAdminUsers

> ApiBillingUsersResponse getApiV1BillingAdminUsers()

List billable users

### Example

```ts
import {
  Configuration,
  BillingApi,
} from '@jilebi/sdk';
import type { GetApiV1BillingAdminUsersRequest } from '@jilebi/sdk';

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
  const api = new BillingApi(config);

  try {
    const data = await api.getApiV1BillingAdminUsers();
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

[**ApiBillingUsersResponse**](ApiBillingUsersResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Usernames and contact details. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1BillingAdminGenerateByUsername

> ApiBillingGenerateResponse postApiV1BillingAdminGenerateByUsername(username, start, end)

Generate an invoice

Generates (and emails) an invoice for a user over an optional time window.

### Example

```ts
import {
  Configuration,
  BillingApi,
} from '@jilebi/sdk';
import type { PostApiV1BillingAdminGenerateByUsernameRequest } from '@jilebi/sdk';

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
  const api = new BillingApi(config);

  const body = {
    // string
    username: username_example,
    // Date | Window start (YYYY-MM-DD). (optional)
    start: 2013-10-20,
    // Date | Window end (YYYY-MM-DD). (optional)
    end: 2013-10-20,
  } satisfies PostApiV1BillingAdminGenerateByUsernameRequest;

  try {
    const data = await api.postApiV1BillingAdminGenerateByUsername(body);
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
| **username** | `string` |  | [Defaults to `undefined`] |
| **start** | `Date` | Window start (YYYY-MM-DD). | [Optional] [Defaults to `undefined`] |
| **end** | `Date` | Window end (YYYY-MM-DD). | [Optional] [Defaults to `undefined`] |

### Return type

[**ApiBillingGenerateResponse**](ApiBillingGenerateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Invoice generation summary. |  -  |
| **400** | Invalid date window |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1BillingAdminResetDodoByUsername

> ApiBillingResetDodoResponse postApiV1BillingAdminResetDodoByUsername(username)

Reset a user\&#39;s Dodo data

### Example

```ts
import {
  Configuration,
  BillingApi,
} from '@jilebi/sdk';
import type { PostApiV1BillingAdminResetDodoByUsernameRequest } from '@jilebi/sdk';

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
  const api = new BillingApi(config);

  const body = {
    // string | Username of the account to reset.
    username: username_example,
  } satisfies PostApiV1BillingAdminResetDodoByUsernameRequest;

  try {
    const data = await api.postApiV1BillingAdminResetDodoByUsername(body);
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
| **username** | `string` | Username of the account to reset. | [Defaults to `undefined`] |

### Return type

[**ApiBillingResetDodoResponse**](ApiBillingResetDodoResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Reset done. |  -  |
| **401** | Unauthorized |  -  |
| **403** | Forbidden |  -  |
| **404** | User not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postApiV1BillingPayByBillId

> postApiV1BillingPayByBillId(billId)

Pay an invoice

HTMX endpoint. Charges the saved mandate, or redirects to a Dodo checkout through the &#x60;HX-Redirect&#x60; response header (empty body). Failures return an HTML fragment targeting &#x60;#pay-error&#x60;.

### Example

```ts
import {
  Configuration,
  BillingApi,
} from '@jilebi/sdk';
import type { PostApiV1BillingPayByBillIdRequest } from '@jilebi/sdk';

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
  const api = new BillingApi(config);

  const body = {
    // string | Id of the invoice to pay.
    billId: billId_example,
  } satisfies PostApiV1BillingPayByBillIdRequest;

  try {
    const data = await api.postApiV1BillingPayByBillId(body);
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
| **billId** | `string` | Id of the invoice to pay. | [Defaults to `undefined`] |

### Return type

`void` (Empty response body)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`, `text/html`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Payment started (see the &#x60;HX-Redirect&#x60; header) or already settled. |  -  |
| **400** | Zero-amount invoice |  -  |
| **401** | Unauthorized |  -  |
| **402** | Payment declined |  -  |
| **404** | Invoice not found |  -  |
| **502** | Payment provider error |  -  |
| **503** | Payments not configured |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


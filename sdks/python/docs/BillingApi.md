# jilebi_sdk.BillingApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_v1_billing_admin_users**](BillingApi.md#get_api_v1_billing_admin_users) | **GET** /api/v1/billing/admin/users | List billable users
[**post_api_v1_billing_admin_generate_by_username**](BillingApi.md#post_api_v1_billing_admin_generate_by_username) | **POST** /api/v1/billing/admin/generate/{username} | Generate an invoice
[**post_api_v1_billing_admin_reset_dodo_by_username**](BillingApi.md#post_api_v1_billing_admin_reset_dodo_by_username) | **POST** /api/v1/billing/admin/reset-dodo/{username} | Reset a user&#39;s Dodo data
[**post_api_v1_billing_pay_by_bill_id**](BillingApi.md#post_api_v1_billing_pay_by_bill_id) | **POST** /api/v1/billing/pay/{billId} | Pay an invoice


# **get_api_v1_billing_admin_users**
> ApiBillingUsersResponse get_api_v1_billing_admin_users()

List billable users

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_billing_users_response import ApiBillingUsersResponse
from jilebi_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://jilebi.ai
# See configuration.py for a list of all supported configuration parameters.
configuration = jilebi_sdk.Configuration(
    host = "https://jilebi.ai"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: apiKey
configuration.api_key['apiKey'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['apiKey'] = 'Bearer'

# Configure API key authorization: sessionCookie
configuration.api_key['sessionCookie'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['sessionCookie'] = 'Bearer'

# Configure Bearer authorization (API key): bearerAuth
configuration = jilebi_sdk.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with jilebi_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = jilebi_sdk.BillingApi(api_client)

    try:
        # List billable users
        api_response = api_instance.get_api_v1_billing_admin_users()
        print("The response of BillingApi->get_api_v1_billing_admin_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling BillingApi->get_api_v1_billing_admin_users: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**ApiBillingUsersResponse**](ApiBillingUsersResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Usernames and contact details. |  -  |
**401** | Unauthorized |  -  |
**403** | Forbidden |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_api_v1_billing_admin_generate_by_username**
> ApiBillingGenerateResponse post_api_v1_billing_admin_generate_by_username(username, start=start, end=end)

Generate an invoice

Generates (and emails) an invoice for a user over an optional time window.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_billing_generate_response import ApiBillingGenerateResponse
from jilebi_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://jilebi.ai
# See configuration.py for a list of all supported configuration parameters.
configuration = jilebi_sdk.Configuration(
    host = "https://jilebi.ai"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: apiKey
configuration.api_key['apiKey'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['apiKey'] = 'Bearer'

# Configure API key authorization: sessionCookie
configuration.api_key['sessionCookie'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['sessionCookie'] = 'Bearer'

# Configure Bearer authorization (API key): bearerAuth
configuration = jilebi_sdk.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with jilebi_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = jilebi_sdk.BillingApi(api_client)
    username = 'username_example' # str | 
    start = '2013-10-20' # date | Window start (YYYY-MM-DD). (optional)
    end = '2013-10-20' # date | Window end (YYYY-MM-DD). (optional)

    try:
        # Generate an invoice
        api_response = api_instance.post_api_v1_billing_admin_generate_by_username(username, start=start, end=end)
        print("The response of BillingApi->post_api_v1_billing_admin_generate_by_username:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling BillingApi->post_api_v1_billing_admin_generate_by_username: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **username** | **str**|  | 
 **start** | **date**| Window start (YYYY-MM-DD). | [optional] 
 **end** | **date**| Window end (YYYY-MM-DD). | [optional] 

### Return type

[**ApiBillingGenerateResponse**](ApiBillingGenerateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Invoice generation summary. |  -  |
**400** | Invalid date window |  -  |
**401** | Unauthorized |  -  |
**403** | Forbidden |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_api_v1_billing_admin_reset_dodo_by_username**
> ApiBillingResetDodoResponse post_api_v1_billing_admin_reset_dodo_by_username(username)

Reset a user's Dodo data

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_billing_reset_dodo_response import ApiBillingResetDodoResponse
from jilebi_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://jilebi.ai
# See configuration.py for a list of all supported configuration parameters.
configuration = jilebi_sdk.Configuration(
    host = "https://jilebi.ai"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: apiKey
configuration.api_key['apiKey'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['apiKey'] = 'Bearer'

# Configure API key authorization: sessionCookie
configuration.api_key['sessionCookie'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['sessionCookie'] = 'Bearer'

# Configure Bearer authorization (API key): bearerAuth
configuration = jilebi_sdk.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with jilebi_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = jilebi_sdk.BillingApi(api_client)
    username = 'username_example' # str | Username of the account to reset.

    try:
        # Reset a user's Dodo data
        api_response = api_instance.post_api_v1_billing_admin_reset_dodo_by_username(username)
        print("The response of BillingApi->post_api_v1_billing_admin_reset_dodo_by_username:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling BillingApi->post_api_v1_billing_admin_reset_dodo_by_username: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **username** | **str**| Username of the account to reset. | 

### Return type

[**ApiBillingResetDodoResponse**](ApiBillingResetDodoResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Reset done. |  -  |
**401** | Unauthorized |  -  |
**403** | Forbidden |  -  |
**404** | User not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_api_v1_billing_pay_by_bill_id**
> post_api_v1_billing_pay_by_bill_id(bill_id)

Pay an invoice

HTMX endpoint. Charges the saved mandate, or redirects to a Dodo checkout through the `HX-Redirect` response header (empty body). Failures return an HTML fragment targeting `#pay-error`.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://jilebi.ai
# See configuration.py for a list of all supported configuration parameters.
configuration = jilebi_sdk.Configuration(
    host = "https://jilebi.ai"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: apiKey
configuration.api_key['apiKey'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['apiKey'] = 'Bearer'

# Configure API key authorization: sessionCookie
configuration.api_key['sessionCookie'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['sessionCookie'] = 'Bearer'

# Configure Bearer authorization (API key): bearerAuth
configuration = jilebi_sdk.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with jilebi_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = jilebi_sdk.BillingApi(api_client)
    bill_id = 'bill_id_example' # str | Id of the invoice to pay.

    try:
        # Pay an invoice
        api_instance.post_api_v1_billing_pay_by_bill_id(bill_id)
    except Exception as e:
        print("Exception when calling BillingApi->post_api_v1_billing_pay_by_bill_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **bill_id** | **str**| Id of the invoice to pay. | 

### Return type

void (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/html

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Payment started (see the &#x60;HX-Redirect&#x60; header) or already settled. |  -  |
**400** | Zero-amount invoice |  -  |
**401** | Unauthorized |  -  |
**402** | Payment declined |  -  |
**404** | Invoice not found |  -  |
**502** | Payment provider error |  -  |
**503** | Payments not configured |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


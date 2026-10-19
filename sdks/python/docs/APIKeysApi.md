# jilebi_sdk.APIKeysApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_api_keys_by_id**](APIKeysApi.md#delete_api_v1_api_keys_by_id) | **DELETE** /api/v1/api-keys/{id} | Revoke an API key
[**get_api_v1_api_keys**](APIKeysApi.md#get_api_v1_api_keys) | **GET** /api/v1/api-keys | List API keys
[**post_api_v1_api_keys**](APIKeysApi.md#post_api_v1_api_keys) | **POST** /api/v1/api-keys | Create an API key


# **delete_api_v1_api_keys_by_id**
> ApiSuccess delete_api_v1_api_keys_by_id(id)

Revoke an API key

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_success import ApiSuccess
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
    api_instance = jilebi_sdk.APIKeysApi(api_client)
    id = 'id_example' # str | Id of the API key.

    try:
        # Revoke an API key
        api_response = api_instance.delete_api_v1_api_keys_by_id(id)
        print("The response of APIKeysApi->delete_api_v1_api_keys_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling APIKeysApi->delete_api_v1_api_keys_by_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| Id of the API key. | 

### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Revoked. |  -  |
**401** | Unauthorized |  -  |
**404** | Key not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_api_v1_api_keys**
> List[ApiKey] get_api_v1_api_keys()

List API keys

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_key import ApiKey
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
    api_instance = jilebi_sdk.APIKeysApi(api_client)

    try:
        # List API keys
        api_response = api_instance.get_api_v1_api_keys()
        print("The response of APIKeysApi->get_api_v1_api_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling APIKeysApi->get_api_v1_api_keys: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[ApiKey]**](ApiKey.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The user&#39;s API keys. |  -  |
**401** | Unauthorized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_api_v1_api_keys**
> ApiKeyCreateResponse post_api_v1_api_keys(api_key_create_request)

Create an API key

The plaintext key is returned only once.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_key_create_request import ApiKeyCreateRequest
from jilebi_sdk.models.api_key_create_response import ApiKeyCreateResponse
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
    api_instance = jilebi_sdk.APIKeysApi(api_client)
    api_key_create_request = jilebi_sdk.ApiKeyCreateRequest() # ApiKeyCreateRequest | 

    try:
        # Create an API key
        api_response = api_instance.post_api_v1_api_keys(api_key_create_request)
        print("The response of APIKeysApi->post_api_v1_api_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling APIKeysApi->post_api_v1_api_keys: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_key_create_request** | [**ApiKeyCreateRequest**](ApiKeyCreateRequest.md)|  | 

### Return type

[**ApiKeyCreateResponse**](ApiKeyCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Key created. |  -  |
**400** | Invalid name, scopes or expiry |  -  |
**401** | Unauthorized |  -  |
**403** | Requested a scope only node admins can grant |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


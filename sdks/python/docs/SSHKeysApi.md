# jilebi_sdk.SSHKeysApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_keys_by_id**](SSHKeysApi.md#delete_api_v1_keys_by_id) | **DELETE** /api/v1/keys/{id} | Delete an SSH key
[**get_api_v1_keys**](SSHKeysApi.md#get_api_v1_keys) | **GET** /api/v1/keys | List SSH keys
[**post_api_v1_keys**](SSHKeysApi.md#post_api_v1_keys) | **POST** /api/v1/keys | Add an SSH key
[**put_api_v1_keys_by_id**](SSHKeysApi.md#put_api_v1_keys_by_id) | **PUT** /api/v1/keys/{id} | Rename an SSH key


# **delete_api_v1_keys_by_id**
> ApiSuccess delete_api_v1_keys_by_id(id)

Delete an SSH key

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
    api_instance = jilebi_sdk.SSHKeysApi(api_client)
    id = 'id_example' # str | Id of the SSH key.

    try:
        # Delete an SSH key
        api_response = api_instance.delete_api_v1_keys_by_id(id)
        print("The response of SSHKeysApi->delete_api_v1_keys_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SSHKeysApi->delete_api_v1_keys_by_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| Id of the SSH key. | 

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
**200** | Deleted. |  -  |
**401** | Unauthorized |  -  |
**404** | Key not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_api_v1_keys**
> List[ApiSshKey] get_api_v1_keys()

List SSH keys

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_ssh_key import ApiSshKey
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
    api_instance = jilebi_sdk.SSHKeysApi(api_client)

    try:
        # List SSH keys
        api_response = api_instance.get_api_v1_keys()
        print("The response of SSHKeysApi->get_api_v1_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SSHKeysApi->get_api_v1_keys: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[ApiSshKey]**](ApiSshKey.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The user&#39;s SSH keys. |  -  |
**401** | Unauthorized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_api_v1_keys**
> ApiSshKeyCreateResponse post_api_v1_keys(api_ssh_key_create_request)

Add an SSH key

Upload an existing public key or generate a new ed25519 key pair. Generated private keys are returned once.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_ssh_key_create_request import ApiSshKeyCreateRequest
from jilebi_sdk.models.api_ssh_key_create_response import ApiSshKeyCreateResponse
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
    api_instance = jilebi_sdk.SSHKeysApi(api_client)
    api_ssh_key_create_request = jilebi_sdk.ApiSshKeyCreateRequest() # ApiSshKeyCreateRequest | 

    try:
        # Add an SSH key
        api_response = api_instance.post_api_v1_keys(api_ssh_key_create_request)
        print("The response of SSHKeysApi->post_api_v1_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SSHKeysApi->post_api_v1_keys: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_ssh_key_create_request** | [**ApiSshKeyCreateRequest**](ApiSshKeyCreateRequest.md)|  | 

### Return type

[**ApiSshKeyCreateResponse**](ApiSshKeyCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Key created. |  -  |
**400** | Missing key or invalid type |  -  |
**401** | Unauthorized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_keys_by_id**
> ApiSuccess put_api_v1_keys_by_id(id, api_ssh_key_update_request)

Rename an SSH key

Update the label of an SSH key. Send an empty string to clear it.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_ssh_key_update_request import ApiSshKeyUpdateRequest
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
    api_instance = jilebi_sdk.SSHKeysApi(api_client)
    id = 'id_example' # str | Id of the SSH key.
    api_ssh_key_update_request = jilebi_sdk.ApiSshKeyUpdateRequest() # ApiSshKeyUpdateRequest | 

    try:
        # Rename an SSH key
        api_response = api_instance.put_api_v1_keys_by_id(id, api_ssh_key_update_request)
        print("The response of SSHKeysApi->put_api_v1_keys_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SSHKeysApi->put_api_v1_keys_by_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| Id of the SSH key. | 
 **api_ssh_key_update_request** | [**ApiSshKeyUpdateRequest**](ApiSshKeyUpdateRequest.md)|  | 

### Return type

[**ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Updated. |  -  |
**400** | Invalid name |  -  |
**401** | Unauthorized |  -  |
**404** | Key not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


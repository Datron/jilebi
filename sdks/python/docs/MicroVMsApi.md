# jilebi_sdk.MicroVMsApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_vmm_by_machine_name**](MicroVMsApi.md#delete_api_v1_vmm_by_machine_name) | **DELETE** /api/v1/vmm/{machine_name} | Delete a microVM
[**get_api_v1_vmm**](MicroVMsApi.md#get_api_v1_vmm) | **GET** /api/v1/vmm | List microVMs
[**get_api_v1_vmm_by_machine_name**](MicroVMsApi.md#get_api_v1_vmm_by_machine_name) | **GET** /api/v1/vmm/{machine_name} | Get a microVM
[**post_api_v1_vmm_by_machine_name**](MicroVMsApi.md#post_api_v1_vmm_by_machine_name) | **POST** /api/v1/vmm/{machine_name} | Create a microVM
[**post_api_v1_vmm_by_machine_name_clone**](MicroVMsApi.md#post_api_v1_vmm_by_machine_name_clone) | **POST** /api/v1/vmm/{machine_name}/clone | Clone a microVM
[**put_api_v1_vmm_by_machine_name**](MicroVMsApi.md#put_api_v1_vmm_by_machine_name) | **PUT** /api/v1/vmm/{machine_name} | Resize a microVM
[**put_api_v1_vmm_by_machine_name_pause**](MicroVMsApi.md#put_api_v1_vmm_by_machine_name_pause) | **PUT** /api/v1/vmm/{machine_name}/pause | Pause a microVM
[**put_api_v1_vmm_by_machine_name_restart**](MicroVMsApi.md#put_api_v1_vmm_by_machine_name_restart) | **PUT** /api/v1/vmm/{machine_name}/restart | Restart a microVM
[**put_api_v1_vmm_by_machine_name_restore**](MicroVMsApi.md#put_api_v1_vmm_by_machine_name_restore) | **PUT** /api/v1/vmm/{machine_name}/restore | Restore a microVM from a snapshot
[**put_api_v1_vmm_by_machine_name_shutdown**](MicroVMsApi.md#put_api_v1_vmm_by_machine_name_shutdown) | **PUT** /api/v1/vmm/{machine_name}/shutdown | Shut down a microVM
[**put_api_v1_vmm_by_machine_name_snapshot**](MicroVMsApi.md#put_api_v1_vmm_by_machine_name_snapshot) | **PUT** /api/v1/vmm/{machine_name}/snapshot | Snapshot a microVM
[**put_api_v1_vmm_by_machine_name_start**](MicroVMsApi.md#put_api_v1_vmm_by_machine_name_start) | **PUT** /api/v1/vmm/{machine_name}/start | Start a microVM


# **delete_api_v1_vmm_by_machine_name**
> ApiSuccess delete_api_v1_vmm_by_machine_name(machine_name)

Delete a microVM

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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.

    try:
        # Delete a microVM
        api_response = api_instance.delete_api_v1_vmm_by_machine_name(machine_name)
        print("The response of MicroVMsApi->delete_api_v1_vmm_by_machine_name:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->delete_api_v1_vmm_by_machine_name: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 

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
**404** | MicroVM not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_api_v1_vmm**
> List[ApiVm] get_api_v1_vmm()

List microVMs

List every microVM owned by the authenticated user.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_vm import ApiVm
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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)

    try:
        # List microVMs
        api_response = api_instance.get_api_v1_vmm()
        print("The response of MicroVMsApi->get_api_v1_vmm:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->get_api_v1_vmm: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[ApiVm]**](ApiVm.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The user&#39;s microVMs. |  -  |
**401** | Unauthorized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_api_v1_vmm_by_machine_name**
> ApiVmGetResponse get_api_v1_vmm_by_machine_name(machine_name)

Get a microVM

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_vm_get_response import ApiVmGetResponse
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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.

    try:
        # Get a microVM
        api_response = api_instance.get_api_v1_vmm_by_machine_name(machine_name)
        print("The response of MicroVMsApi->get_api_v1_vmm_by_machine_name:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->get_api_v1_vmm_by_machine_name: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 

### Return type

[**ApiVmGetResponse**](ApiVmGetResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The microVM with live hypervisor info. |  -  |
**401** | Unauthorized |  -  |
**404** | MicroVM not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_api_v1_vmm_by_machine_name**
> ApiVmCreateResponse post_api_v1_vmm_by_machine_name(machine_name, api_vm_create_request)

Create a microVM

Provisions a new microVM on a node with sufficient free capacity.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_vm_create_request import ApiVmCreateRequest
from jilebi_sdk.models.api_vm_create_response import ApiVmCreateResponse
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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.
    api_vm_create_request = jilebi_sdk.ApiVmCreateRequest() # ApiVmCreateRequest | 

    try:
        # Create a microVM
        api_response = api_instance.post_api_v1_vmm_by_machine_name(machine_name, api_vm_create_request)
        print("The response of MicroVMsApi->post_api_v1_vmm_by_machine_name:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->post_api_v1_vmm_by_machine_name: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 
 **api_vm_create_request** | [**ApiVmCreateRequest**](ApiVmCreateRequest.md)|  | 

### Return type

[**ApiVmCreateResponse**](ApiVmCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Created. |  -  |
**400** | Invalid size or over quota |  -  |
**401** | Unauthorized |  -  |
**402** | Overdue unpaid invoice |  -  |
**409** | Name taken or no node available |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_api_v1_vmm_by_machine_name_clone**
> ApiVmCloneResponse post_api_v1_vmm_by_machine_name_clone(machine_name, api_vm_clone_request)

Clone a microVM

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_vm_clone_request import ApiVmCloneRequest
from jilebi_sdk.models.api_vm_clone_response import ApiVmCloneResponse
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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.
    api_vm_clone_request = jilebi_sdk.ApiVmCloneRequest() # ApiVmCloneRequest | 

    try:
        # Clone a microVM
        api_response = api_instance.post_api_v1_vmm_by_machine_name_clone(machine_name, api_vm_clone_request)
        print("The response of MicroVMsApi->post_api_v1_vmm_by_machine_name_clone:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->post_api_v1_vmm_by_machine_name_clone: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 
 **api_vm_clone_request** | [**ApiVmCloneRequest**](ApiVmCloneRequest.md)|  | 

### Return type

[**ApiVmCloneResponse**](ApiVmCloneResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Clone created. |  -  |
**400** | Invalid name or missing SSH key |  -  |
**401** | Unauthorized |  -  |
**404** | Source microVM not found |  -  |
**409** | Name already exists |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_vmm_by_machine_name**
> ApiSuccess put_api_v1_vmm_by_machine_name(machine_name, api_vm_update_request)

Resize a microVM

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_success import ApiSuccess
from jilebi_sdk.models.api_vm_update_request import ApiVmUpdateRequest
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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.
    api_vm_update_request = jilebi_sdk.ApiVmUpdateRequest() # ApiVmUpdateRequest | 

    try:
        # Resize a microVM
        api_response = api_instance.put_api_v1_vmm_by_machine_name(machine_name, api_vm_update_request)
        print("The response of MicroVMsApi->put_api_v1_vmm_by_machine_name:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->put_api_v1_vmm_by_machine_name: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 
 **api_vm_update_request** | [**ApiVmUpdateRequest**](ApiVmUpdateRequest.md)|  | 

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
**400** | Invalid size |  -  |
**401** | Unauthorized |  -  |
**404** | MicroVM not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_vmm_by_machine_name_pause**
> ApiSuccess put_api_v1_vmm_by_machine_name_pause(machine_name)

Pause a microVM

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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.

    try:
        # Pause a microVM
        api_response = api_instance.put_api_v1_vmm_by_machine_name_pause(machine_name)
        print("The response of MicroVMsApi->put_api_v1_vmm_by_machine_name_pause:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->put_api_v1_vmm_by_machine_name_pause: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 

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
**200** | Paused. |  -  |
**400** | MicroVM is not active |  -  |
**401** | Unauthorized |  -  |
**404** | MicroVM not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_vmm_by_machine_name_restart**
> ApiSuccess put_api_v1_vmm_by_machine_name_restart(machine_name)

Restart a microVM

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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.

    try:
        # Restart a microVM
        api_response = api_instance.put_api_v1_vmm_by_machine_name_restart(machine_name)
        print("The response of MicroVMsApi->put_api_v1_vmm_by_machine_name_restart:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->put_api_v1_vmm_by_machine_name_restart: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 

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
**200** | Restarted. |  -  |
**400** | MicroVM is not active |  -  |
**401** | Unauthorized |  -  |
**404** | MicroVM not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_vmm_by_machine_name_restore**
> ApiVmRestoreResponse put_api_v1_vmm_by_machine_name_restore(machine_name, api_vm_restore_request)

Restore a microVM from a snapshot

Restores in place, or creates a new microVM when `new_name` is set.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_vm_restore_request import ApiVmRestoreRequest
from jilebi_sdk.models.api_vm_restore_response import ApiVmRestoreResponse
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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.
    api_vm_restore_request = jilebi_sdk.ApiVmRestoreRequest() # ApiVmRestoreRequest | 

    try:
        # Restore a microVM from a snapshot
        api_response = api_instance.put_api_v1_vmm_by_machine_name_restore(machine_name, api_vm_restore_request)
        print("The response of MicroVMsApi->put_api_v1_vmm_by_machine_name_restore:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->put_api_v1_vmm_by_machine_name_restore: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 
 **api_vm_restore_request** | [**ApiVmRestoreRequest**](ApiVmRestoreRequest.md)|  | 

### Return type

[**ApiVmRestoreResponse**](ApiVmRestoreResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Restored in place. |  -  |
**201** | Restored into a new microVM. |  -  |
**400** | Invalid name |  -  |
**401** | Unauthorized |  -  |
**404** | MicroVM not found |  -  |
**409** | Name already exists |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_vmm_by_machine_name_shutdown**
> ApiSuccess put_api_v1_vmm_by_machine_name_shutdown(machine_name)

Shut down a microVM

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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.

    try:
        # Shut down a microVM
        api_response = api_instance.put_api_v1_vmm_by_machine_name_shutdown(machine_name)
        print("The response of MicroVMsApi->put_api_v1_vmm_by_machine_name_shutdown:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->put_api_v1_vmm_by_machine_name_shutdown: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 

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
**200** | Shut down. |  -  |
**400** | MicroVM is not active |  -  |
**401** | Unauthorized |  -  |
**404** | MicroVM not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_vmm_by_machine_name_snapshot**
> ApiVmSnapshotResponse put_api_v1_vmm_by_machine_name_snapshot(machine_name, api_vm_snapshot_request=api_vm_snapshot_request)

Snapshot a microVM

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_vm_snapshot_request import ApiVmSnapshotRequest
from jilebi_sdk.models.api_vm_snapshot_response import ApiVmSnapshotResponse
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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.
    api_vm_snapshot_request = jilebi_sdk.ApiVmSnapshotRequest() # ApiVmSnapshotRequest |  (optional)

    try:
        # Snapshot a microVM
        api_response = api_instance.put_api_v1_vmm_by_machine_name_snapshot(machine_name, api_vm_snapshot_request=api_vm_snapshot_request)
        print("The response of MicroVMsApi->put_api_v1_vmm_by_machine_name_snapshot:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->put_api_v1_vmm_by_machine_name_snapshot: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 
 **api_vm_snapshot_request** | [**ApiVmSnapshotRequest**](ApiVmSnapshotRequest.md)|  | [optional] 

### Return type

[**ApiVmSnapshotResponse**](ApiVmSnapshotResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Snapshot created. |  -  |
**401** | Unauthorized |  -  |
**404** | MicroVM not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_vmm_by_machine_name_start**
> ApiSuccess put_api_v1_vmm_by_machine_name_start(machine_name)

Start a microVM

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
    api_instance = jilebi_sdk.MicroVMsApi(api_client)
    machine_name = 'machine_name_example' # str | Name of the microVM.

    try:
        # Start a microVM
        api_response = api_instance.put_api_v1_vmm_by_machine_name_start(machine_name)
        print("The response of MicroVMsApi->put_api_v1_vmm_by_machine_name_start:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MicroVMsApi->put_api_v1_vmm_by_machine_name_start: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **machine_name** | **str**| Name of the microVM. | 

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
**200** | Started, or already active. |  -  |
**401** | Unauthorized |  -  |
**402** | Overdue unpaid invoice |  -  |
**404** | MicroVM not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


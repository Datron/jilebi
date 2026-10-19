# jilebi_sdk.RoutesApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_routes_by_name**](RoutesApi.md#delete_api_v1_routes_by_name) | **DELETE** /api/v1/routes/{name} | Delete a route
[**get_api_v1_routes**](RoutesApi.md#get_api_v1_routes) | **GET** /api/v1/routes | List routes
[**post_api_v1_routes**](RoutesApi.md#post_api_v1_routes) | **POST** /api/v1/routes | Create a route
[**put_api_v1_routes_by_name**](RoutesApi.md#put_api_v1_routes_by_name) | **PUT** /api/v1/routes/{name} | Update a route


# **delete_api_v1_routes_by_name**
> ApiSuccess delete_api_v1_routes_by_name(name)

Delete a route

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
    api_instance = jilebi_sdk.RoutesApi(api_client)
    name = 'name_example' # str | Name of the route.

    try:
        # Delete a route
        api_response = api_instance.delete_api_v1_routes_by_name(name)
        print("The response of RoutesApi->delete_api_v1_routes_by_name:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RoutesApi->delete_api_v1_routes_by_name: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **str**| Name of the route. | 

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
**404** | Route not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_api_v1_routes**
> ApiRouteListResponse get_api_v1_routes()

List routes

Routes owned by the user plus routes shared with them.

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_route_list_response import ApiRouteListResponse
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
    api_instance = jilebi_sdk.RoutesApi(api_client)

    try:
        # List routes
        api_response = api_instance.get_api_v1_routes()
        print("The response of RoutesApi->get_api_v1_routes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RoutesApi->get_api_v1_routes: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**ApiRouteListResponse**](ApiRouteListResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Owned and shared routes. |  -  |
**401** | Unauthorized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_api_v1_routes**
> ApiRouteCreateResponse post_api_v1_routes(api_route_create_request)

Create a route

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_route_create_request import ApiRouteCreateRequest
from jilebi_sdk.models.api_route_create_response import ApiRouteCreateResponse
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
    api_instance = jilebi_sdk.RoutesApi(api_client)
    api_route_create_request = jilebi_sdk.ApiRouteCreateRequest() # ApiRouteCreateRequest | 

    try:
        # Create a route
        api_response = api_instance.post_api_v1_routes(api_route_create_request)
        print("The response of RoutesApi->post_api_v1_routes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RoutesApi->post_api_v1_routes: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_route_create_request** | [**ApiRouteCreateRequest**](ApiRouteCreateRequest.md)|  | 

### Return type

[**ApiRouteCreateResponse**](ApiRouteCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Created. |  -  |
**400** | Invalid name, port or emails |  -  |
**401** | Unauthorized |  -  |
**404** | MicroVM not found |  -  |
**409** | Route name already used |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_api_v1_routes_by_name**
> ApiSuccess put_api_v1_routes_by_name(name, api_route_update_request=api_route_update_request)

Update a route

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_route_update_request import ApiRouteUpdateRequest
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
    api_instance = jilebi_sdk.RoutesApi(api_client)
    name = 'name_example' # str | Name of the route.
    api_route_update_request = jilebi_sdk.ApiRouteUpdateRequest() # ApiRouteUpdateRequest |  (optional)

    try:
        # Update a route
        api_response = api_instance.put_api_v1_routes_by_name(name, api_route_update_request=api_route_update_request)
        print("The response of RoutesApi->put_api_v1_routes_by_name:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RoutesApi->put_api_v1_routes_by_name: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **str**| Name of the route. | 
 **api_route_update_request** | [**ApiRouteUpdateRequest**](ApiRouteUpdateRequest.md)|  | [optional] 

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
**400** | Invalid port, privacy or emails |  -  |
**401** | Unauthorized |  -  |
**404** | Route not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# jilebi_sdk.AccountApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_me**](AccountApi.md#get_api_me) | **GET** /api/me | Get the current user and session


# **get_api_me**
> ApiMeResponse get_api_me()

Get the current user and session

### Example

* Api Key Authentication (apiKey):
* Api Key Authentication (sessionCookie):
* Bearer (API key) Authentication (bearerAuth):

```python
import jilebi_sdk
from jilebi_sdk.models.api_me_response import ApiMeResponse
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
    api_instance = jilebi_sdk.AccountApi(api_client)

    try:
        # Get the current user and session
        api_response = api_instance.get_api_me()
        print("The response of AccountApi->get_api_me:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AccountApi->get_api_me: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**ApiMeResponse**](ApiMeResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The authenticated user and their session. |  -  |
**401** | Unauthorized |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


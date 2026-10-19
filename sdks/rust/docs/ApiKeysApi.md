# \ApiKeysApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_api_keys_by_id**](ApiKeysApi.md#delete_api_v1_api_keys_by_id) | **DELETE** /api/v1/api-keys/{id} | Revoke an API key
[**get_api_v1_api_keys**](ApiKeysApi.md#get_api_v1_api_keys) | **GET** /api/v1/api-keys | List API keys
[**post_api_v1_api_keys**](ApiKeysApi.md#post_api_v1_api_keys) | **POST** /api/v1/api-keys | Create an API key



## delete_api_v1_api_keys_by_id

> models::ApiSuccess delete_api_v1_api_keys_by_id(id)
Revoke an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the API key. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_api_keys

> Vec<models::ApiKey> get_api_v1_api_keys()
List API keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiKey>**](ApiKey.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_api_keys

> models::ApiKeyCreateResponse post_api_v1_api_keys(api_key_create_request)
Create an API key

The plaintext key is returned only once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_create_request** | [**ApiKeyCreateRequest**](ApiKeyCreateRequest.md) |  | [required] |

### Return type

[**models::ApiKeyCreateResponse**](ApiKeyCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


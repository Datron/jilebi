# \SshKeysApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_keys_by_id**](SshKeysApi.md#delete_api_v1_keys_by_id) | **DELETE** /api/v1/keys/{id} | Delete an SSH key
[**get_api_v1_keys**](SshKeysApi.md#get_api_v1_keys) | **GET** /api/v1/keys | List SSH keys
[**post_api_v1_keys**](SshKeysApi.md#post_api_v1_keys) | **POST** /api/v1/keys | Add an SSH key
[**put_api_v1_keys_by_id**](SshKeysApi.md#put_api_v1_keys_by_id) | **PUT** /api/v1/keys/{id} | Rename an SSH key



## delete_api_v1_keys_by_id

> models::ApiSuccess delete_api_v1_keys_by_id(id)
Delete an SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the SSH key. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_keys

> Vec<models::ApiSshKey> get_api_v1_keys()
List SSH keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiSshKey>**](ApiSshKey.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_keys

> models::ApiSshKeyCreateResponse post_api_v1_keys(api_ssh_key_create_request)
Add an SSH key

Upload an existing public key or generate a new ed25519 key pair. Generated private keys are returned once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_ssh_key_create_request** | [**ApiSshKeyCreateRequest**](ApiSshKeyCreateRequest.md) |  | [required] |

### Return type

[**models::ApiSshKeyCreateResponse**](ApiSshKeyCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_keys_by_id

> models::ApiSuccess put_api_v1_keys_by_id(id, api_ssh_key_update_request)
Rename an SSH key

Update the label of an SSH key. Send an empty string to clear it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the SSH key. | [required] |
**api_ssh_key_update_request** | [**ApiSshKeyUpdateRequest**](ApiSshKeyUpdateRequest.md) |  | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


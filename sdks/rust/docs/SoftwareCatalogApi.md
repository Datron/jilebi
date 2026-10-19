# \SoftwareCatalogApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_software_by_id**](SoftwareCatalogApi.md#delete_api_v1_software_by_id) | **DELETE** /api/v1/software/{id} | Delete software
[**get_api_v1_software**](SoftwareCatalogApi.md#get_api_v1_software) | **GET** /api/v1/software | List software
[**get_api_v1_software_by_id**](SoftwareCatalogApi.md#get_api_v1_software_by_id) | **GET** /api/v1/software/{id} | Get software
[**post_api_v1_software**](SoftwareCatalogApi.md#post_api_v1_software) | **POST** /api/v1/software | Add software
[**put_api_v1_software_by_id**](SoftwareCatalogApi.md#put_api_v1_software_by_id) | **PUT** /api/v1/software/{id} | Update software



## delete_api_v1_software_by_id

> models::ApiSuccess delete_api_v1_software_by_id(id)
Delete software

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the catalog entry. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_software

> Vec<models::ApiSoftware> get_api_v1_software()
List software

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiSoftware>**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_software_by_id

> models::ApiSoftware get_api_v1_software_by_id(id)
Get software

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the catalog entry. | [required] |

### Return type

[**models::ApiSoftware**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_software

> models::ApiSoftware post_api_v1_software(api_software_create_request)
Add software

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_software_create_request** | [**ApiSoftwareCreateRequest**](ApiSoftwareCreateRequest.md) |  | [required] |

### Return type

[**models::ApiSoftware**](ApiSoftware.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_software_by_id

> models::ApiSuccess put_api_v1_software_by_id(id, api_software_update_request)
Update software

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the catalog entry. | [required] |
**api_software_update_request** | Option<[**ApiSoftwareUpdateRequest**](ApiSoftwareUpdateRequest.md)> |  |  |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \RoutesApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_routes_by_name**](RoutesApi.md#delete_api_v1_routes_by_name) | **DELETE** /api/v1/routes/{name} | Delete a route
[**get_api_v1_routes**](RoutesApi.md#get_api_v1_routes) | **GET** /api/v1/routes | List routes
[**post_api_v1_routes**](RoutesApi.md#post_api_v1_routes) | **POST** /api/v1/routes | Create a route
[**put_api_v1_routes_by_name**](RoutesApi.md#put_api_v1_routes_by_name) | **PUT** /api/v1/routes/{name} | Update a route



## delete_api_v1_routes_by_name

> models::ApiSuccess delete_api_v1_routes_by_name(name)
Delete a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the route. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_routes

> models::ApiRouteListResponse get_api_v1_routes()
List routes

Routes owned by the user plus routes shared with them.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiRouteListResponse**](ApiRouteListResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_routes

> models::ApiRouteCreateResponse post_api_v1_routes(api_route_create_request)
Create a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_route_create_request** | [**ApiRouteCreateRequest**](ApiRouteCreateRequest.md) |  | [required] |

### Return type

[**models::ApiRouteCreateResponse**](ApiRouteCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_routes_by_name

> models::ApiSuccess put_api_v1_routes_by_name(name, api_route_update_request)
Update a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the route. | [required] |
**api_route_update_request** | Option<[**ApiRouteUpdateRequest**](ApiRouteUpdateRequest.md)> |  |  |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


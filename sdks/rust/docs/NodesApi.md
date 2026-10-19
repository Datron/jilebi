# \NodesApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_nodes_by_name**](NodesApi.md#delete_api_v1_nodes_by_name) | **DELETE** /api/v1/nodes/{name} | Delete a node
[**get_api_v1_nodes**](NodesApi.md#get_api_v1_nodes) | **GET** /api/v1/nodes | List nodes
[**get_api_v1_nodes_by_name**](NodesApi.md#get_api_v1_nodes_by_name) | **GET** /api/v1/nodes/{name} | Get a node
[**post_api_v1_nodes**](NodesApi.md#post_api_v1_nodes) | **POST** /api/v1/nodes | Register a node
[**post_api_v1_nodes_by_name_health**](NodesApi.md#post_api_v1_nodes_by_name_health) | **POST** /api/v1/nodes/{name}/health | Check node health
[**post_api_v1_nodes_by_name_refresh**](NodesApi.md#post_api_v1_nodes_by_name_refresh) | **POST** /api/v1/nodes/{name}/refresh | Refresh node resources
[**post_api_v1_nodes_by_name_reinitialize**](NodesApi.md#post_api_v1_nodes_by_name_reinitialize) | **POST** /api/v1/nodes/{name}/reinitialize | Reinitialize a node
[**put_api_v1_nodes_by_name**](NodesApi.md#put_api_v1_nodes_by_name) | **PUT** /api/v1/nodes/{name} | Update a node



## delete_api_v1_nodes_by_name

> models::ApiNode delete_api_v1_nodes_by_name(name)
Delete a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the node. | [required] |

### Return type

[**models::ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_nodes

> Vec<models::ApiNode> get_api_v1_nodes()
List nodes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiNode>**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_nodes_by_name

> models::ApiNode get_api_v1_nodes_by_name(name)
Get a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the node. | [required] |

### Return type

[**models::ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_nodes

> models::ApiNode post_api_v1_nodes(api_node_create_request)
Register a node

Probes the node's /system endpoint and registers it with its control plane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_node_create_request** | [**ApiNodeCreateRequest**](ApiNodeCreateRequest.md) |  | [required] |

### Return type

[**models::ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_nodes_by_name_health

> models::ApiNodeHealthResponse post_api_v1_nodes_by_name_health(name)
Check node health

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the node. | [required] |

### Return type

[**models::ApiNodeHealthResponse**](ApiNodeHealthResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_nodes_by_name_refresh

> models::ApiNodeRefreshResponse post_api_v1_nodes_by_name_refresh(name)
Refresh node resources

Re-probes the node and recomputes available capacity from active microVM allocations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the node. | [required] |

### Return type

[**models::ApiNodeRefreshResponse**](ApiNodeRefreshResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_nodes_by_name_reinitialize

> models::ApiNodeReinitializeResponse post_api_v1_nodes_by_name_reinitialize(name)
Reinitialize a node

Pushes the stored host info back to the node's control plane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the node. | [required] |

### Return type

[**models::ApiNodeReinitializeResponse**](ApiNodeReinitializeResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_nodes_by_name

> models::ApiNode put_api_v1_nodes_by_name(name, api_node_update_request)
Update a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**api_node_update_request** | Option<[**ApiNodeUpdateRequest**](ApiNodeUpdateRequest.md)> |  |  |

### Return type

[**models::ApiNode**](ApiNode.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


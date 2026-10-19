# \MicroVmsApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_vmm_by_machine_name**](MicroVmsApi.md#delete_api_v1_vmm_by_machine_name) | **DELETE** /api/v1/vmm/{machine_name} | Delete a microVM
[**get_api_v1_vmm**](MicroVmsApi.md#get_api_v1_vmm) | **GET** /api/v1/vmm | List microVMs
[**get_api_v1_vmm_by_machine_name**](MicroVmsApi.md#get_api_v1_vmm_by_machine_name) | **GET** /api/v1/vmm/{machine_name} | Get a microVM
[**post_api_v1_vmm_by_machine_name**](MicroVmsApi.md#post_api_v1_vmm_by_machine_name) | **POST** /api/v1/vmm/{machine_name} | Create a microVM
[**post_api_v1_vmm_by_machine_name_clone**](MicroVmsApi.md#post_api_v1_vmm_by_machine_name_clone) | **POST** /api/v1/vmm/{machine_name}/clone | Clone a microVM
[**put_api_v1_vmm_by_machine_name**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name) | **PUT** /api/v1/vmm/{machine_name} | Resize a microVM
[**put_api_v1_vmm_by_machine_name_pause**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_pause) | **PUT** /api/v1/vmm/{machine_name}/pause | Pause a microVM
[**put_api_v1_vmm_by_machine_name_restart**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_restart) | **PUT** /api/v1/vmm/{machine_name}/restart | Restart a microVM
[**put_api_v1_vmm_by_machine_name_restore**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_restore) | **PUT** /api/v1/vmm/{machine_name}/restore | Restore a microVM from a snapshot
[**put_api_v1_vmm_by_machine_name_shutdown**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_shutdown) | **PUT** /api/v1/vmm/{machine_name}/shutdown | Shut down a microVM
[**put_api_v1_vmm_by_machine_name_snapshot**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_snapshot) | **PUT** /api/v1/vmm/{machine_name}/snapshot | Snapshot a microVM
[**put_api_v1_vmm_by_machine_name_start**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_start) | **PUT** /api/v1/vmm/{machine_name}/start | Start a microVM



## delete_api_v1_vmm_by_machine_name

> models::ApiSuccess delete_api_v1_vmm_by_machine_name(machine_name)
Delete a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_vmm

> Vec<models::ApiVm> get_api_v1_vmm()
List microVMs

List every microVM owned by the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiVm>**](ApiVm.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_v1_vmm_by_machine_name

> models::ApiVmGetResponse get_api_v1_vmm_by_machine_name(machine_name)
Get a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |

### Return type

[**models::ApiVmGetResponse**](ApiVmGetResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_vmm_by_machine_name

> models::ApiVmCreateResponse post_api_v1_vmm_by_machine_name(machine_name, api_vm_create_request)
Create a microVM

Provisions a new microVM on a node with sufficient free capacity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |
**api_vm_create_request** | [**ApiVmCreateRequest**](ApiVmCreateRequest.md) |  | [required] |

### Return type

[**models::ApiVmCreateResponse**](ApiVmCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_vmm_by_machine_name_clone

> models::ApiVmCloneResponse post_api_v1_vmm_by_machine_name_clone(machine_name, api_vm_clone_request)
Clone a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |
**api_vm_clone_request** | [**ApiVmCloneRequest**](ApiVmCloneRequest.md) |  | [required] |

### Return type

[**models::ApiVmCloneResponse**](ApiVmCloneResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_vmm_by_machine_name

> models::ApiSuccess put_api_v1_vmm_by_machine_name(machine_name, api_vm_update_request)
Resize a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |
**api_vm_update_request** | [**ApiVmUpdateRequest**](ApiVmUpdateRequest.md) |  | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_vmm_by_machine_name_pause

> models::ApiSuccess put_api_v1_vmm_by_machine_name_pause(machine_name)
Pause a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_vmm_by_machine_name_restart

> models::ApiSuccess put_api_v1_vmm_by_machine_name_restart(machine_name)
Restart a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_vmm_by_machine_name_restore

> models::ApiVmRestoreResponse put_api_v1_vmm_by_machine_name_restore(machine_name, api_vm_restore_request)
Restore a microVM from a snapshot

Restores in place, or creates a new microVM when `new_name` is set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |
**api_vm_restore_request** | [**ApiVmRestoreRequest**](ApiVmRestoreRequest.md) |  | [required] |

### Return type

[**models::ApiVmRestoreResponse**](ApiVmRestoreResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_vmm_by_machine_name_shutdown

> models::ApiSuccess put_api_v1_vmm_by_machine_name_shutdown(machine_name)
Shut down a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_vmm_by_machine_name_snapshot

> models::ApiVmSnapshotResponse put_api_v1_vmm_by_machine_name_snapshot(machine_name, api_vm_snapshot_request)
Snapshot a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |
**api_vm_snapshot_request** | Option<[**ApiVmSnapshotRequest**](ApiVmSnapshotRequest.md)> |  |  |

### Return type

[**models::ApiVmSnapshotResponse**](ApiVmSnapshotResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_api_v1_vmm_by_machine_name_start

> models::ApiSuccess put_api_v1_vmm_by_machine_name_start(machine_name)
Start a microVM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |

### Return type

[**models::ApiSuccess**](ApiSuccess.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


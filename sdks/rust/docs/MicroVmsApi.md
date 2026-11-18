# \MicroVmsApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_v1_vmm_by_machine_name**](MicroVmsApi.md#delete_api_v1_vmm_by_machine_name) | **DELETE** /api/v1/vmm/{machine_name} | Delete a microVM
[**get_api_v1_vmm**](MicroVmsApi.md#get_api_v1_vmm) | **GET** /api/v1/vmm | List microVMs
[**get_api_v1_vmm_by_machine_name**](MicroVmsApi.md#get_api_v1_vmm_by_machine_name) | **GET** /api/v1/vmm/{machine_name} | Get a microVM
[**get_api_v1_vmm_by_machine_name_metrics**](MicroVmsApi.md#get_api_v1_vmm_by_machine_name_metrics) | **GET** /api/v1/vmm/{machine_name}/metrics | Get microVM usage metrics
[**post_api_v1_vmm_by_machine_name**](MicroVmsApi.md#post_api_v1_vmm_by_machine_name) | **POST** /api/v1/vmm/{machine_name} | Create a microVM
[**post_api_v1_vmm_by_machine_name_clone**](MicroVmsApi.md#post_api_v1_vmm_by_machine_name_clone) | **POST** /api/v1/vmm/{machine_name}/clone | Clone a microVM
[**put_api_v1_vmm_by_machine_name**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name) | **PUT** /api/v1/vmm/{machine_name} | Resize a microVM
[**put_api_v1_vmm_by_machine_name_pause**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_pause) | **PUT** /api/v1/vmm/{machine_name}/pause | Pause a microVM
[**put_api_v1_vmm_by_machine_name_restart**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_restart) | **PUT** /api/v1/vmm/{machine_name}/restart | Restart a microVM
[**put_api_v1_vmm_by_machine_name_restore**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_restore) | **PUT** /api/v1/vmm/{machine_name}/restore | Restore a microVM from a snapshot
[**put_api_v1_vmm_by_machine_name_shutdown**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_shutdown) | **PUT** /api/v1/vmm/{machine_name}/shutdown | Shut down a microVM
[**put_api_v1_vmm_by_machine_name_sleep**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_sleep) | **PUT** /api/v1/vmm/{machine_name}/sleep | Sleep a microVM
[**put_api_v1_vmm_by_machine_name_snapshot**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_snapshot) | **PUT** /api/v1/vmm/{machine_name}/snapshot | Snapshot a microVM
[**put_api_v1_vmm_by_machine_name_start**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_start) | **PUT** /api/v1/vmm/{machine_name}/start | Start a microVM
[**put_api_v1_vmm_by_machine_name_wake**](MicroVmsApi.md#put_api_v1_vmm_by_machine_name_wake) | **PUT** /api/v1/vmm/{machine_name}/wake | Wake a microVM



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


## get_api_v1_vmm_by_machine_name_metrics

> models::ApiVmMetricsResponse get_api_v1_vmm_by_machine_name_metrics(machine_name, range, step)
Get microVM usage metrics

Returns CPU, memory, network and disk time series for the microVM. Each series includes its points plus current, average and max values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |
**range** | Option<**String**> | Time window to query. |  |[default to 1h]
**step** | Option<**i32**> | Sampling resolution in seconds. Defaults to a window-appropriate step (max ~500 points). |  |

### Return type

[**models::ApiVmMetricsResponse**](ApiVmMetricsResponse.md)

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


## put_api_v1_vmm_by_machine_name_sleep

> models::ApiVmSleepResponse put_api_v1_vmm_by_machine_name_sleep(machine_name)
Sleep a microVM

Pauses the VM, snapshots it to a rolling slot, then evicts the hypervisor process to free host memory. Wakes in place from the snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |

### Return type

[**models::ApiVmSleepResponse**](ApiVmSleepResponse.md)

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


## put_api_v1_vmm_by_machine_name_wake

> models::ApiVmWakeResponse put_api_v1_vmm_by_machine_name_wake(machine_name, api_vm_wake_request)
Wake a microVM

Resumes a live-paused VM in place or restores a slept VM from its snapshot. When no usable snapshot exists, boots the existing disk fresh (RAM state is lost, disk contents are preserved).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_name** | **String** | Name of the microVM. | [required] |
**api_vm_wake_request** | Option<[**ApiVmWakeRequest**](ApiVmWakeRequest.md)> |  |  |

### Return type

[**models::ApiVmWakeResponse**](ApiVmWakeResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


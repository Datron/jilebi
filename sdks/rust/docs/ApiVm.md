# ApiVm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**user_id** | **String** |  | 
**machine_name** | **String** |  | 
**network_name** | Option<**String**> |  | 
**node_name** | **String** |  | 
**hypervisor_socket** | Option<**String**> |  | 
**ip_address** | Option<**String**> | Private network address (e.g. 192.168.2.2). | 
**cpu_count** | **i32** | vCPU count. | 
**memory_size** | **i32** | Memory in MiB. | 
**disk_size_gb** | **i32** | Disk size in GiB. | 
**state** | [**models::ApiVmState**](ApiVmState.md) |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



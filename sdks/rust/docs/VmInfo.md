# VmInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | **std::collections::HashMap<String, serde_json::Value>** | Cloud-hypervisor VM config. | 
**state** | **State** |  (enum: Created, Running, Shutdown, Paused) | 
**memory_actual_size** | Option<**i32**> | Memory in bytes. | [optional]
**device_tree** | Option<[**std::collections::HashMap<String, models::DeviceNode>**](DeviceNode.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



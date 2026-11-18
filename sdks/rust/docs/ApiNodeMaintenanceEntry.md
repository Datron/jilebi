# ApiNodeMaintenanceEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **String** |  | 
**machine_name** | **String** |  | 
**ip_address** | Option<**String**> |  | [optional]
**prev_state** | **PrevState** | State the VM had before the run, and is restored to. (enum: active, paused) | 
**state** | **State** | Snapshot progress. (enum: pending, ok, failed) | 
**restored** | **bool** | True once the VM has been woken back. | 
**destination_url** | Option<**String**> | Snapshot slot the VM was saved to. | [optional]
**error** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



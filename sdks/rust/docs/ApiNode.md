# ApiNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**url** | **String** |  | 
**vcpu_total** | **i32** |  | 
**vcpu_available** | **i32** |  | 
**memory_total** | **i32** | Bytes. | 
**memory_available** | **i32** | Bytes. | 
**disk_total** | **i32** | Bytes. | 
**disk_available** | **i32** | Bytes. | 
**specialization** | **Specialization** |  (enum: General, Coding) | 
**status** | [**models::ApiNodeStatus**](ApiNodeStatus.md) |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



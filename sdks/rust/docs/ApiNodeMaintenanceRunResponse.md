# ApiNodeMaintenanceRunResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node** | **String** |  | 
**phase** | **Phase** |  (enum: idle, snapshot, restore) | 
**running** | **bool** | True while the run is working in the background. | 
**started_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**routes_pending** | **bool** | True when every VM is back but some app routes still need replaying. | 
**entries** | [**Vec<models::ApiNodeMaintenanceEntry>**](ApiNodeMaintenanceEntry.md) |  | 
**message** | **String** | Human summary of the run. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



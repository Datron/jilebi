# ApiVmSleepResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | Option<**bool**> |  | [optional]
**destination_url** | **String** | Snapshot slot URL the VM was saved to. | 
**already_sleeping** | **bool** | True when the VM was already evicted. | 
**pause_ms** | Option<**i32**> | Time spent pausing the vCPUs. | [optional]
**snapshot_ms** | Option<**i32**> | Time spent writing the snapshot. | [optional]
**total_ms** | Option<**i32**> | Total time for the sleep operation. | [optional]
**deduped** | Option<**bool**> | True when an identical sleep was already running and this call was a no-op. | [optional]
**state_changed** | Option<**bool**> | True when this call actually put the VM to sleep. Callers release node capacity only when this is true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



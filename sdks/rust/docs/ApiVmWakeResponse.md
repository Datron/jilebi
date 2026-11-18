# ApiVmWakeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | Option<**bool**> |  | [optional]
**source_url** | Option<**String**> | Snapshot restored from, when not resumed in place. | [optional]
**already_running** | **bool** | True when the VM was already running. | 
**resumed_in_place** | **bool** | True when a live-paused VM was resumed. | 
**cold_booted** | **bool** | True when no snapshot was usable and the existing disk was booted fresh (RAM state lost). | 
**took_ms** | Option<**i32**> | Time taken to bring the VM back. | [optional]
**deduped** | Option<**bool**> | True when an identical wake was already running and this call was a no-op. | [optional]
**state_changed** | Option<**bool**> | True when this call actually moved the VM to running. Callers reserve node capacity only when this is true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



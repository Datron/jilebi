# ApiVmMetricsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**machine_name** | **String** |  | 
**node_name** | **String** |  | 
**range** | **Range** |  (enum: 1h, 6h, 24h, 7d) | 
**start** | **i32** | Window start, Unix epoch seconds. | 
**end** | **i32** | Window end, Unix epoch seconds. | 
**step_seconds** | **i32** |  | 
**available** | **bool** | True when at least one metrics query succeeded. | 
**error** | Option<**String**> | Why metrics could not be read. | [optional]
**configured** | [**models::ApiVmMetricsResponseConfigured**](ApiVmMetricsResponseConfigured.md) |  | 
**cpu** | [**models::ApiVmMetricsResponseCpu**](ApiVmMetricsResponseCpu.md) |  | 
**memory** | [**models::ApiVmMetricsResponseMemory**](ApiVmMetricsResponseMemory.md) |  | 
**network** | [**models::ApiVmMetricsResponseNetwork**](ApiVmMetricsResponseNetwork.md) |  | 
**disk** | [**models::ApiVmMetricsResponseDisk**](ApiVmMetricsResponseDisk.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



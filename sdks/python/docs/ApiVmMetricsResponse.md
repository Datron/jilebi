# ApiVmMetricsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**machine_name** | **str** |  | 
**node_name** | **str** |  | 
**range** | **str** |  | 
**start** | **int** | Window start, Unix epoch seconds. | 
**end** | **int** | Window end, Unix epoch seconds. | 
**step_seconds** | **int** |  | 
**available** | **bool** | True when at least one metrics query succeeded. | 
**error** | **str** | Why metrics could not be read. | [optional] 
**configured** | [**ApiVmMetricsResponseConfigured**](ApiVmMetricsResponseConfigured.md) |  | 
**cpu** | [**ApiVmMetricsResponseCpu**](ApiVmMetricsResponseCpu.md) |  | 
**memory** | [**ApiVmMetricsResponseMemory**](ApiVmMetricsResponseMemory.md) |  | 
**network** | [**ApiVmMetricsResponseNetwork**](ApiVmMetricsResponseNetwork.md) |  | 
**disk** | [**ApiVmMetricsResponseDisk**](ApiVmMetricsResponseDisk.md) |  | 

## Example

```python
from jilebi_sdk.models.api_vm_metrics_response import ApiVmMetricsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmMetricsResponse from a JSON string
api_vm_metrics_response_instance = ApiVmMetricsResponse.from_json(json)
# print the JSON string representation of the object
print(ApiVmMetricsResponse.to_json())

# convert the object into a dict
api_vm_metrics_response_dict = api_vm_metrics_response_instance.to_dict()
# create an instance of ApiVmMetricsResponse from a dict
api_vm_metrics_response_from_dict = ApiVmMetricsResponse.from_dict(api_vm_metrics_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



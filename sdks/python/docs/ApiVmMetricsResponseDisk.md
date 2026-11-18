# ApiVmMetricsResponseDisk


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**used_gb** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**configured_gb** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**read_gbps** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**write_gbps** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 

## Example

```python
from jilebi_sdk.models.api_vm_metrics_response_disk import ApiVmMetricsResponseDisk

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmMetricsResponseDisk from a JSON string
api_vm_metrics_response_disk_instance = ApiVmMetricsResponseDisk.from_json(json)
# print the JSON string representation of the object
print(ApiVmMetricsResponseDisk.to_json())

# convert the object into a dict
api_vm_metrics_response_disk_dict = api_vm_metrics_response_disk_instance.to_dict()
# create an instance of ApiVmMetricsResponseDisk from a dict
api_vm_metrics_response_disk_from_dict = ApiVmMetricsResponseDisk.from_dict(api_vm_metrics_response_disk_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



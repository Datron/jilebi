# ApiVmMetricsResponseNetwork


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rx_bps** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**tx_bps** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 

## Example

```python
from jilebi_sdk.models.api_vm_metrics_response_network import ApiVmMetricsResponseNetwork

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmMetricsResponseNetwork from a JSON string
api_vm_metrics_response_network_instance = ApiVmMetricsResponseNetwork.from_json(json)
# print the JSON string representation of the object
print(ApiVmMetricsResponseNetwork.to_json())

# convert the object into a dict
api_vm_metrics_response_network_dict = api_vm_metrics_response_network_instance.to_dict()
# create an instance of ApiVmMetricsResponseNetwork from a dict
api_vm_metrics_response_network_from_dict = ApiVmMetricsResponseNetwork.from_dict(api_vm_metrics_response_network_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



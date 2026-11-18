# ApiVmSleepResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** |  | [optional] 
**destination_url** | **str** | Snapshot slot URL the VM was saved to. | 
**already_sleeping** | **bool** | True when the VM was already evicted. | 
**pause_ms** | **int** | Time spent pausing the vCPUs. | [optional] 
**snapshot_ms** | **int** | Time spent writing the snapshot. | [optional] 
**total_ms** | **int** | Total time for the sleep operation. | [optional] 
**deduped** | **bool** | True when an identical sleep was already running and this call was a no-op. | [optional] 
**state_changed** | **bool** | True when this call actually put the VM to sleep. Callers release node capacity only when this is true. | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_sleep_response import ApiVmSleepResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmSleepResponse from a JSON string
api_vm_sleep_response_instance = ApiVmSleepResponse.from_json(json)
# print the JSON string representation of the object
print(ApiVmSleepResponse.to_json())

# convert the object into a dict
api_vm_sleep_response_dict = api_vm_sleep_response_instance.to_dict()
# create an instance of ApiVmSleepResponse from a dict
api_vm_sleep_response_from_dict = ApiVmSleepResponse.from_dict(api_vm_sleep_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



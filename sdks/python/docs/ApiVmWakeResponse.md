# ApiVmWakeResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** |  | [optional] 
**source_url** | **str** | Snapshot restored from, when not resumed in place. | [optional] 
**already_running** | **bool** | True when the VM was already running. | 
**resumed_in_place** | **bool** | True when a live-paused VM was resumed. | 
**cold_booted** | **bool** | True when no snapshot was usable and the existing disk was booted fresh (RAM state lost). | 
**took_ms** | **int** | Time taken to bring the VM back. | [optional] 
**deduped** | **bool** | True when an identical wake was already running and this call was a no-op. | [optional] 
**state_changed** | **bool** | True when this call actually moved the VM to running. Callers reserve node capacity only when this is true. | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_wake_response import ApiVmWakeResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmWakeResponse from a JSON string
api_vm_wake_response_instance = ApiVmWakeResponse.from_json(json)
# print the JSON string representation of the object
print(ApiVmWakeResponse.to_json())

# convert the object into a dict
api_vm_wake_response_dict = api_vm_wake_response_instance.to_dict()
# create an instance of ApiVmWakeResponse from a dict
api_vm_wake_response_from_dict = ApiVmWakeResponse.from_dict(api_vm_wake_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



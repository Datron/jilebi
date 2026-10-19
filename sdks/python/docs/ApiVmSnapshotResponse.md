# ApiVmSnapshotResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_url** | **str** |  | 
**resumed** | **bool** | Whether the VM was resumed after snapshotting. | 

## Example

```python
from jilebi_sdk.models.api_vm_snapshot_response import ApiVmSnapshotResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmSnapshotResponse from a JSON string
api_vm_snapshot_response_instance = ApiVmSnapshotResponse.from_json(json)
# print the JSON string representation of the object
print(ApiVmSnapshotResponse.to_json())

# convert the object into a dict
api_vm_snapshot_response_dict = api_vm_snapshot_response_instance.to_dict()
# create an instance of ApiVmSnapshotResponse from a dict
api_vm_snapshot_response_from_dict = ApiVmSnapshotResponse.from_dict(api_vm_snapshot_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



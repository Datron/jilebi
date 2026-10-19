# ApiVmSnapshotRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_url** | **str** | Object-storage URL to upload the snapshot to. | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_snapshot_request import ApiVmSnapshotRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmSnapshotRequest from a JSON string
api_vm_snapshot_request_instance = ApiVmSnapshotRequest.from_json(json)
# print the JSON string representation of the object
print(ApiVmSnapshotRequest.to_json())

# convert the object into a dict
api_vm_snapshot_request_dict = api_vm_snapshot_request_instance.to_dict()
# create an instance of ApiVmSnapshotRequest from a dict
api_vm_snapshot_request_from_dict = ApiVmSnapshotRequest.from_dict(api_vm_snapshot_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ApiNode


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **str** |  | 
**url** | **str** |  | 
**vcpu_total** | **int** |  | 
**vcpu_available** | **int** |  | 
**memory_total** | **int** | Bytes. | 
**memory_available** | **int** | Bytes. | 
**disk_total** | **int** | Bytes. | 
**disk_available** | **int** | Bytes. | 
**specialization** | **str** |  | 
**status** | [**ApiNodeStatus**](ApiNodeStatus.md) |  | 
**created_at** | **datetime** |  | 
**updated_at** | **datetime** |  | 

## Example

```python
from jilebi_sdk.models.api_node import ApiNode

# TODO update the JSON string below
json = "{}"
# create an instance of ApiNode from a JSON string
api_node_instance = ApiNode.from_json(json)
# print the JSON string representation of the object
print(ApiNode.to_json())

# convert the object into a dict
api_node_dict = api_node_instance.to_dict()
# create an instance of ApiNode from a dict
api_node_from_dict = ApiNode.from_dict(api_node_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



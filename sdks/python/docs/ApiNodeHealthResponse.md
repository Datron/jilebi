# ApiNodeHealthResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**http** | **str** | &#x60;OK (&lt;status&gt;)&#x60; or &#x60;FAIL: &lt;reason&gt;&#x60;. | 

## Example

```python
from jilebi_sdk.models.api_node_health_response import ApiNodeHealthResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiNodeHealthResponse from a JSON string
api_node_health_response_instance = ApiNodeHealthResponse.from_json(json)
# print the JSON string representation of the object
print(ApiNodeHealthResponse.to_json())

# convert the object into a dict
api_node_health_response_dict = api_node_health_response_instance.to_dict()
# create an instance of ApiNodeHealthResponse from a dict
api_node_health_response_from_dict = ApiNodeHealthResponse.from_dict(api_node_health_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



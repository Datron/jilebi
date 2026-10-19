# ApiRouteCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**microvm_id** | **String** | Id of the VM the route forwards to. | 
**port** | **i32** |  | 
**privacy** | **Privacy** |  (enum: private, public, shared) | 
**emails** | Option<**Vec<String>**> | Required when `privacy` is `shared`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



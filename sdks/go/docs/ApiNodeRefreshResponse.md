# ApiNodeRefreshResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Message** | **string** |  | 
**Node** | [**ApiNode**](ApiNode.md) |  | 

## Methods

### NewApiNodeRefreshResponse

`func NewApiNodeRefreshResponse(message string, node ApiNode, ) *ApiNodeRefreshResponse`

NewApiNodeRefreshResponse instantiates a new ApiNodeRefreshResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiNodeRefreshResponseWithDefaults

`func NewApiNodeRefreshResponseWithDefaults() *ApiNodeRefreshResponse`

NewApiNodeRefreshResponseWithDefaults instantiates a new ApiNodeRefreshResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMessage

`func (o *ApiNodeRefreshResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *ApiNodeRefreshResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *ApiNodeRefreshResponse) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetNode

`func (o *ApiNodeRefreshResponse) GetNode() ApiNode`

GetNode returns the Node field if non-nil, zero value otherwise.

### GetNodeOk

`func (o *ApiNodeRefreshResponse) GetNodeOk() (*ApiNode, bool)`

GetNodeOk returns a tuple with the Node field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNode

`func (o *ApiNodeRefreshResponse) SetNode(v ApiNode)`

SetNode sets Node field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



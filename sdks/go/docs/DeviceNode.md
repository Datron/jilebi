# DeviceNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | Pointer to **NullableString** |  | [optional] 
**Resources** | Pointer to **[]interface{}** |  | [optional] 
**Children** | Pointer to **[]string** |  | [optional] 
**PciBdf** | Pointer to **NullableString** |  | [optional] 

## Methods

### NewDeviceNode

`func NewDeviceNode() *DeviceNode`

NewDeviceNode instantiates a new DeviceNode object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDeviceNodeWithDefaults

`func NewDeviceNodeWithDefaults() *DeviceNode`

NewDeviceNodeWithDefaults instantiates a new DeviceNode object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *DeviceNode) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *DeviceNode) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *DeviceNode) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *DeviceNode) HasId() bool`

HasId returns a boolean if a field has been set.

### SetIdNil

`func (o *DeviceNode) SetIdNil(b bool)`

 SetIdNil sets the value for Id to be an explicit nil

### UnsetId
`func (o *DeviceNode) UnsetId()`

UnsetId ensures that no value is present for Id, not even an explicit nil
### GetResources

`func (o *DeviceNode) GetResources() []interface{}`

GetResources returns the Resources field if non-nil, zero value otherwise.

### GetResourcesOk

`func (o *DeviceNode) GetResourcesOk() (*[]interface{}, bool)`

GetResourcesOk returns a tuple with the Resources field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResources

`func (o *DeviceNode) SetResources(v []interface{})`

SetResources sets Resources field to given value.

### HasResources

`func (o *DeviceNode) HasResources() bool`

HasResources returns a boolean if a field has been set.

### SetResourcesNil

`func (o *DeviceNode) SetResourcesNil(b bool)`

 SetResourcesNil sets the value for Resources to be an explicit nil

### UnsetResources
`func (o *DeviceNode) UnsetResources()`

UnsetResources ensures that no value is present for Resources, not even an explicit nil
### GetChildren

`func (o *DeviceNode) GetChildren() []string`

GetChildren returns the Children field if non-nil, zero value otherwise.

### GetChildrenOk

`func (o *DeviceNode) GetChildrenOk() (*[]string, bool)`

GetChildrenOk returns a tuple with the Children field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChildren

`func (o *DeviceNode) SetChildren(v []string)`

SetChildren sets Children field to given value.

### HasChildren

`func (o *DeviceNode) HasChildren() bool`

HasChildren returns a boolean if a field has been set.

### SetChildrenNil

`func (o *DeviceNode) SetChildrenNil(b bool)`

 SetChildrenNil sets the value for Children to be an explicit nil

### UnsetChildren
`func (o *DeviceNode) UnsetChildren()`

UnsetChildren ensures that no value is present for Children, not even an explicit nil
### GetPciBdf

`func (o *DeviceNode) GetPciBdf() string`

GetPciBdf returns the PciBdf field if non-nil, zero value otherwise.

### GetPciBdfOk

`func (o *DeviceNode) GetPciBdfOk() (*string, bool)`

GetPciBdfOk returns a tuple with the PciBdf field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPciBdf

`func (o *DeviceNode) SetPciBdf(v string)`

SetPciBdf sets PciBdf field to given value.

### HasPciBdf

`func (o *DeviceNode) HasPciBdf() bool`

HasPciBdf returns a boolean if a field has been set.

### SetPciBdfNil

`func (o *DeviceNode) SetPciBdfNil(b bool)`

 SetPciBdfNil sets the value for PciBdf to be an explicit nil

### UnsetPciBdf
`func (o *DeviceNode) UnsetPciBdf()`

UnsetPciBdf ensures that no value is present for PciBdf, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



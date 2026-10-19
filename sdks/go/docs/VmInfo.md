# VmInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Config** | **map[string]interface{}** | Cloud-hypervisor VM config. | 
**State** | **string** |  | 
**MemoryActualSize** | Pointer to **NullableInt32** | Memory in bytes. | [optional] 
**DeviceTree** | Pointer to [**map[string]DeviceNode**](DeviceNode.md) |  | [optional] 

## Methods

### NewVmInfo

`func NewVmInfo(config map[string]interface{}, state string, ) *VmInfo`

NewVmInfo instantiates a new VmInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewVmInfoWithDefaults

`func NewVmInfoWithDefaults() *VmInfo`

NewVmInfoWithDefaults instantiates a new VmInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetConfig

`func (o *VmInfo) GetConfig() map[string]interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *VmInfo) GetConfigOk() (*map[string]interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *VmInfo) SetConfig(v map[string]interface{})`

SetConfig sets Config field to given value.


### GetState

`func (o *VmInfo) GetState() string`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *VmInfo) GetStateOk() (*string, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *VmInfo) SetState(v string)`

SetState sets State field to given value.


### GetMemoryActualSize

`func (o *VmInfo) GetMemoryActualSize() int32`

GetMemoryActualSize returns the MemoryActualSize field if non-nil, zero value otherwise.

### GetMemoryActualSizeOk

`func (o *VmInfo) GetMemoryActualSizeOk() (*int32, bool)`

GetMemoryActualSizeOk returns a tuple with the MemoryActualSize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemoryActualSize

`func (o *VmInfo) SetMemoryActualSize(v int32)`

SetMemoryActualSize sets MemoryActualSize field to given value.

### HasMemoryActualSize

`func (o *VmInfo) HasMemoryActualSize() bool`

HasMemoryActualSize returns a boolean if a field has been set.

### SetMemoryActualSizeNil

`func (o *VmInfo) SetMemoryActualSizeNil(b bool)`

 SetMemoryActualSizeNil sets the value for MemoryActualSize to be an explicit nil

### UnsetMemoryActualSize
`func (o *VmInfo) UnsetMemoryActualSize()`

UnsetMemoryActualSize ensures that no value is present for MemoryActualSize, not even an explicit nil
### GetDeviceTree

`func (o *VmInfo) GetDeviceTree() map[string]DeviceNode`

GetDeviceTree returns the DeviceTree field if non-nil, zero value otherwise.

### GetDeviceTreeOk

`func (o *VmInfo) GetDeviceTreeOk() (*map[string]DeviceNode, bool)`

GetDeviceTreeOk returns a tuple with the DeviceTree field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceTree

`func (o *VmInfo) SetDeviceTree(v map[string]DeviceNode)`

SetDeviceTree sets DeviceTree field to given value.

### HasDeviceTree

`func (o *VmInfo) HasDeviceTree() bool`

HasDeviceTree returns a boolean if a field has been set.

### SetDeviceTreeNil

`func (o *VmInfo) SetDeviceTreeNil(b bool)`

 SetDeviceTreeNil sets the value for DeviceTree to be an explicit nil

### UnsetDeviceTree
`func (o *VmInfo) UnsetDeviceTree()`

UnsetDeviceTree ensures that no value is present for DeviceTree, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



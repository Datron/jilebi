# ApiVm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**UserId** | **string** |  | 
**MachineName** | **string** |  | 
**NetworkName** | **NullableString** |  | 
**NodeName** | **string** |  | 
**HypervisorSocket** | **NullableString** |  | 
**IpAddress** | **NullableString** | Private network address (e.g. 192.168.2.2). | 
**CpuCount** | **int32** | vCPU count. | 
**MemorySize** | **int32** | Memory in MiB. | 
**DiskSizeGb** | **int32** | Disk size in GiB. | 
**State** | [**ApiVmState**](ApiVmState.md) |  | 
**CreatedAt** | **time.Time** |  | 
**UpdatedAt** | **time.Time** |  | 

## Methods

### NewApiVm

`func NewApiVm(id string, userId string, machineName string, networkName NullableString, nodeName string, hypervisorSocket NullableString, ipAddress NullableString, cpuCount int32, memorySize int32, diskSizeGb int32, state ApiVmState, createdAt time.Time, updatedAt time.Time, ) *ApiVm`

NewApiVm instantiates a new ApiVm object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmWithDefaults

`func NewApiVmWithDefaults() *ApiVm`

NewApiVmWithDefaults instantiates a new ApiVm object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiVm) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiVm) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiVm) SetId(v string)`

SetId sets Id field to given value.


### GetUserId

`func (o *ApiVm) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *ApiVm) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *ApiVm) SetUserId(v string)`

SetUserId sets UserId field to given value.


### GetMachineName

`func (o *ApiVm) GetMachineName() string`

GetMachineName returns the MachineName field if non-nil, zero value otherwise.

### GetMachineNameOk

`func (o *ApiVm) GetMachineNameOk() (*string, bool)`

GetMachineNameOk returns a tuple with the MachineName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMachineName

`func (o *ApiVm) SetMachineName(v string)`

SetMachineName sets MachineName field to given value.


### GetNetworkName

`func (o *ApiVm) GetNetworkName() string`

GetNetworkName returns the NetworkName field if non-nil, zero value otherwise.

### GetNetworkNameOk

`func (o *ApiVm) GetNetworkNameOk() (*string, bool)`

GetNetworkNameOk returns a tuple with the NetworkName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNetworkName

`func (o *ApiVm) SetNetworkName(v string)`

SetNetworkName sets NetworkName field to given value.


### SetNetworkNameNil

`func (o *ApiVm) SetNetworkNameNil(b bool)`

 SetNetworkNameNil sets the value for NetworkName to be an explicit nil

### UnsetNetworkName
`func (o *ApiVm) UnsetNetworkName()`

UnsetNetworkName ensures that no value is present for NetworkName, not even an explicit nil
### GetNodeName

`func (o *ApiVm) GetNodeName() string`

GetNodeName returns the NodeName field if non-nil, zero value otherwise.

### GetNodeNameOk

`func (o *ApiVm) GetNodeNameOk() (*string, bool)`

GetNodeNameOk returns a tuple with the NodeName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNodeName

`func (o *ApiVm) SetNodeName(v string)`

SetNodeName sets NodeName field to given value.


### GetHypervisorSocket

`func (o *ApiVm) GetHypervisorSocket() string`

GetHypervisorSocket returns the HypervisorSocket field if non-nil, zero value otherwise.

### GetHypervisorSocketOk

`func (o *ApiVm) GetHypervisorSocketOk() (*string, bool)`

GetHypervisorSocketOk returns a tuple with the HypervisorSocket field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHypervisorSocket

`func (o *ApiVm) SetHypervisorSocket(v string)`

SetHypervisorSocket sets HypervisorSocket field to given value.


### SetHypervisorSocketNil

`func (o *ApiVm) SetHypervisorSocketNil(b bool)`

 SetHypervisorSocketNil sets the value for HypervisorSocket to be an explicit nil

### UnsetHypervisorSocket
`func (o *ApiVm) UnsetHypervisorSocket()`

UnsetHypervisorSocket ensures that no value is present for HypervisorSocket, not even an explicit nil
### GetIpAddress

`func (o *ApiVm) GetIpAddress() string`

GetIpAddress returns the IpAddress field if non-nil, zero value otherwise.

### GetIpAddressOk

`func (o *ApiVm) GetIpAddressOk() (*string, bool)`

GetIpAddressOk returns a tuple with the IpAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIpAddress

`func (o *ApiVm) SetIpAddress(v string)`

SetIpAddress sets IpAddress field to given value.


### SetIpAddressNil

`func (o *ApiVm) SetIpAddressNil(b bool)`

 SetIpAddressNil sets the value for IpAddress to be an explicit nil

### UnsetIpAddress
`func (o *ApiVm) UnsetIpAddress()`

UnsetIpAddress ensures that no value is present for IpAddress, not even an explicit nil
### GetCpuCount

`func (o *ApiVm) GetCpuCount() int32`

GetCpuCount returns the CpuCount field if non-nil, zero value otherwise.

### GetCpuCountOk

`func (o *ApiVm) GetCpuCountOk() (*int32, bool)`

GetCpuCountOk returns a tuple with the CpuCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCpuCount

`func (o *ApiVm) SetCpuCount(v int32)`

SetCpuCount sets CpuCount field to given value.


### GetMemorySize

`func (o *ApiVm) GetMemorySize() int32`

GetMemorySize returns the MemorySize field if non-nil, zero value otherwise.

### GetMemorySizeOk

`func (o *ApiVm) GetMemorySizeOk() (*int32, bool)`

GetMemorySizeOk returns a tuple with the MemorySize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemorySize

`func (o *ApiVm) SetMemorySize(v int32)`

SetMemorySize sets MemorySize field to given value.


### GetDiskSizeGb

`func (o *ApiVm) GetDiskSizeGb() int32`

GetDiskSizeGb returns the DiskSizeGb field if non-nil, zero value otherwise.

### GetDiskSizeGbOk

`func (o *ApiVm) GetDiskSizeGbOk() (*int32, bool)`

GetDiskSizeGbOk returns a tuple with the DiskSizeGb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDiskSizeGb

`func (o *ApiVm) SetDiskSizeGb(v int32)`

SetDiskSizeGb sets DiskSizeGb field to given value.


### GetState

`func (o *ApiVm) GetState() ApiVmState`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *ApiVm) GetStateOk() (*ApiVmState, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *ApiVm) SetState(v ApiVmState)`

SetState sets State field to given value.


### GetCreatedAt

`func (o *ApiVm) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiVm) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiVm) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetUpdatedAt

`func (o *ApiVm) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ApiVm) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ApiVm) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



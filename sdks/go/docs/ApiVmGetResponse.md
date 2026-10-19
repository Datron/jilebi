# ApiVmGetResponse

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
**VmInfo** | [**VmInfo**](VmInfo.md) |  | 

## Methods

### NewApiVmGetResponse

`func NewApiVmGetResponse(id string, userId string, machineName string, networkName NullableString, nodeName string, hypervisorSocket NullableString, ipAddress NullableString, cpuCount int32, memorySize int32, diskSizeGb int32, state ApiVmState, createdAt time.Time, updatedAt time.Time, vmInfo VmInfo, ) *ApiVmGetResponse`

NewApiVmGetResponse instantiates a new ApiVmGetResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmGetResponseWithDefaults

`func NewApiVmGetResponseWithDefaults() *ApiVmGetResponse`

NewApiVmGetResponseWithDefaults instantiates a new ApiVmGetResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiVmGetResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiVmGetResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiVmGetResponse) SetId(v string)`

SetId sets Id field to given value.


### GetUserId

`func (o *ApiVmGetResponse) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *ApiVmGetResponse) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *ApiVmGetResponse) SetUserId(v string)`

SetUserId sets UserId field to given value.


### GetMachineName

`func (o *ApiVmGetResponse) GetMachineName() string`

GetMachineName returns the MachineName field if non-nil, zero value otherwise.

### GetMachineNameOk

`func (o *ApiVmGetResponse) GetMachineNameOk() (*string, bool)`

GetMachineNameOk returns a tuple with the MachineName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMachineName

`func (o *ApiVmGetResponse) SetMachineName(v string)`

SetMachineName sets MachineName field to given value.


### GetNetworkName

`func (o *ApiVmGetResponse) GetNetworkName() string`

GetNetworkName returns the NetworkName field if non-nil, zero value otherwise.

### GetNetworkNameOk

`func (o *ApiVmGetResponse) GetNetworkNameOk() (*string, bool)`

GetNetworkNameOk returns a tuple with the NetworkName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNetworkName

`func (o *ApiVmGetResponse) SetNetworkName(v string)`

SetNetworkName sets NetworkName field to given value.


### SetNetworkNameNil

`func (o *ApiVmGetResponse) SetNetworkNameNil(b bool)`

 SetNetworkNameNil sets the value for NetworkName to be an explicit nil

### UnsetNetworkName
`func (o *ApiVmGetResponse) UnsetNetworkName()`

UnsetNetworkName ensures that no value is present for NetworkName, not even an explicit nil
### GetNodeName

`func (o *ApiVmGetResponse) GetNodeName() string`

GetNodeName returns the NodeName field if non-nil, zero value otherwise.

### GetNodeNameOk

`func (o *ApiVmGetResponse) GetNodeNameOk() (*string, bool)`

GetNodeNameOk returns a tuple with the NodeName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNodeName

`func (o *ApiVmGetResponse) SetNodeName(v string)`

SetNodeName sets NodeName field to given value.


### GetHypervisorSocket

`func (o *ApiVmGetResponse) GetHypervisorSocket() string`

GetHypervisorSocket returns the HypervisorSocket field if non-nil, zero value otherwise.

### GetHypervisorSocketOk

`func (o *ApiVmGetResponse) GetHypervisorSocketOk() (*string, bool)`

GetHypervisorSocketOk returns a tuple with the HypervisorSocket field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHypervisorSocket

`func (o *ApiVmGetResponse) SetHypervisorSocket(v string)`

SetHypervisorSocket sets HypervisorSocket field to given value.


### SetHypervisorSocketNil

`func (o *ApiVmGetResponse) SetHypervisorSocketNil(b bool)`

 SetHypervisorSocketNil sets the value for HypervisorSocket to be an explicit nil

### UnsetHypervisorSocket
`func (o *ApiVmGetResponse) UnsetHypervisorSocket()`

UnsetHypervisorSocket ensures that no value is present for HypervisorSocket, not even an explicit nil
### GetIpAddress

`func (o *ApiVmGetResponse) GetIpAddress() string`

GetIpAddress returns the IpAddress field if non-nil, zero value otherwise.

### GetIpAddressOk

`func (o *ApiVmGetResponse) GetIpAddressOk() (*string, bool)`

GetIpAddressOk returns a tuple with the IpAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIpAddress

`func (o *ApiVmGetResponse) SetIpAddress(v string)`

SetIpAddress sets IpAddress field to given value.


### SetIpAddressNil

`func (o *ApiVmGetResponse) SetIpAddressNil(b bool)`

 SetIpAddressNil sets the value for IpAddress to be an explicit nil

### UnsetIpAddress
`func (o *ApiVmGetResponse) UnsetIpAddress()`

UnsetIpAddress ensures that no value is present for IpAddress, not even an explicit nil
### GetCpuCount

`func (o *ApiVmGetResponse) GetCpuCount() int32`

GetCpuCount returns the CpuCount field if non-nil, zero value otherwise.

### GetCpuCountOk

`func (o *ApiVmGetResponse) GetCpuCountOk() (*int32, bool)`

GetCpuCountOk returns a tuple with the CpuCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCpuCount

`func (o *ApiVmGetResponse) SetCpuCount(v int32)`

SetCpuCount sets CpuCount field to given value.


### GetMemorySize

`func (o *ApiVmGetResponse) GetMemorySize() int32`

GetMemorySize returns the MemorySize field if non-nil, zero value otherwise.

### GetMemorySizeOk

`func (o *ApiVmGetResponse) GetMemorySizeOk() (*int32, bool)`

GetMemorySizeOk returns a tuple with the MemorySize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemorySize

`func (o *ApiVmGetResponse) SetMemorySize(v int32)`

SetMemorySize sets MemorySize field to given value.


### GetDiskSizeGb

`func (o *ApiVmGetResponse) GetDiskSizeGb() int32`

GetDiskSizeGb returns the DiskSizeGb field if non-nil, zero value otherwise.

### GetDiskSizeGbOk

`func (o *ApiVmGetResponse) GetDiskSizeGbOk() (*int32, bool)`

GetDiskSizeGbOk returns a tuple with the DiskSizeGb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDiskSizeGb

`func (o *ApiVmGetResponse) SetDiskSizeGb(v int32)`

SetDiskSizeGb sets DiskSizeGb field to given value.


### GetState

`func (o *ApiVmGetResponse) GetState() ApiVmState`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *ApiVmGetResponse) GetStateOk() (*ApiVmState, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *ApiVmGetResponse) SetState(v ApiVmState)`

SetState sets State field to given value.


### GetCreatedAt

`func (o *ApiVmGetResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiVmGetResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiVmGetResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetUpdatedAt

`func (o *ApiVmGetResponse) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ApiVmGetResponse) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ApiVmGetResponse) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.


### GetVmInfo

`func (o *ApiVmGetResponse) GetVmInfo() VmInfo`

GetVmInfo returns the VmInfo field if non-nil, zero value otherwise.

### GetVmInfoOk

`func (o *ApiVmGetResponse) GetVmInfoOk() (*VmInfo, bool)`

GetVmInfoOk returns a tuple with the VmInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVmInfo

`func (o *ApiVmGetResponse) SetVmInfo(v VmInfo)`

SetVmInfo sets VmInfo field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



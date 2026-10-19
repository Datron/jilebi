# ApiNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | **string** |  | 
**Url** | **string** |  | 
**VcpuTotal** | **int32** |  | 
**VcpuAvailable** | **int32** |  | 
**MemoryTotal** | **int32** | Bytes. | 
**MemoryAvailable** | **int32** | Bytes. | 
**DiskTotal** | **int32** | Bytes. | 
**DiskAvailable** | **int32** | Bytes. | 
**Specialization** | **string** |  | 
**Status** | [**ApiNodeStatus**](ApiNodeStatus.md) |  | 
**CreatedAt** | **time.Time** |  | 
**UpdatedAt** | **time.Time** |  | 

## Methods

### NewApiNode

`func NewApiNode(name string, url string, vcpuTotal int32, vcpuAvailable int32, memoryTotal int32, memoryAvailable int32, diskTotal int32, diskAvailable int32, specialization string, status ApiNodeStatus, createdAt time.Time, updatedAt time.Time, ) *ApiNode`

NewApiNode instantiates a new ApiNode object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiNodeWithDefaults

`func NewApiNodeWithDefaults() *ApiNode`

NewApiNodeWithDefaults instantiates a new ApiNode object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *ApiNode) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiNode) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiNode) SetName(v string)`

SetName sets Name field to given value.


### GetUrl

`func (o *ApiNode) GetUrl() string`

GetUrl returns the Url field if non-nil, zero value otherwise.

### GetUrlOk

`func (o *ApiNode) GetUrlOk() (*string, bool)`

GetUrlOk returns a tuple with the Url field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUrl

`func (o *ApiNode) SetUrl(v string)`

SetUrl sets Url field to given value.


### GetVcpuTotal

`func (o *ApiNode) GetVcpuTotal() int32`

GetVcpuTotal returns the VcpuTotal field if non-nil, zero value otherwise.

### GetVcpuTotalOk

`func (o *ApiNode) GetVcpuTotalOk() (*int32, bool)`

GetVcpuTotalOk returns a tuple with the VcpuTotal field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVcpuTotal

`func (o *ApiNode) SetVcpuTotal(v int32)`

SetVcpuTotal sets VcpuTotal field to given value.


### GetVcpuAvailable

`func (o *ApiNode) GetVcpuAvailable() int32`

GetVcpuAvailable returns the VcpuAvailable field if non-nil, zero value otherwise.

### GetVcpuAvailableOk

`func (o *ApiNode) GetVcpuAvailableOk() (*int32, bool)`

GetVcpuAvailableOk returns a tuple with the VcpuAvailable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVcpuAvailable

`func (o *ApiNode) SetVcpuAvailable(v int32)`

SetVcpuAvailable sets VcpuAvailable field to given value.


### GetMemoryTotal

`func (o *ApiNode) GetMemoryTotal() int32`

GetMemoryTotal returns the MemoryTotal field if non-nil, zero value otherwise.

### GetMemoryTotalOk

`func (o *ApiNode) GetMemoryTotalOk() (*int32, bool)`

GetMemoryTotalOk returns a tuple with the MemoryTotal field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemoryTotal

`func (o *ApiNode) SetMemoryTotal(v int32)`

SetMemoryTotal sets MemoryTotal field to given value.


### GetMemoryAvailable

`func (o *ApiNode) GetMemoryAvailable() int32`

GetMemoryAvailable returns the MemoryAvailable field if non-nil, zero value otherwise.

### GetMemoryAvailableOk

`func (o *ApiNode) GetMemoryAvailableOk() (*int32, bool)`

GetMemoryAvailableOk returns a tuple with the MemoryAvailable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemoryAvailable

`func (o *ApiNode) SetMemoryAvailable(v int32)`

SetMemoryAvailable sets MemoryAvailable field to given value.


### GetDiskTotal

`func (o *ApiNode) GetDiskTotal() int32`

GetDiskTotal returns the DiskTotal field if non-nil, zero value otherwise.

### GetDiskTotalOk

`func (o *ApiNode) GetDiskTotalOk() (*int32, bool)`

GetDiskTotalOk returns a tuple with the DiskTotal field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDiskTotal

`func (o *ApiNode) SetDiskTotal(v int32)`

SetDiskTotal sets DiskTotal field to given value.


### GetDiskAvailable

`func (o *ApiNode) GetDiskAvailable() int32`

GetDiskAvailable returns the DiskAvailable field if non-nil, zero value otherwise.

### GetDiskAvailableOk

`func (o *ApiNode) GetDiskAvailableOk() (*int32, bool)`

GetDiskAvailableOk returns a tuple with the DiskAvailable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDiskAvailable

`func (o *ApiNode) SetDiskAvailable(v int32)`

SetDiskAvailable sets DiskAvailable field to given value.


### GetSpecialization

`func (o *ApiNode) GetSpecialization() string`

GetSpecialization returns the Specialization field if non-nil, zero value otherwise.

### GetSpecializationOk

`func (o *ApiNode) GetSpecializationOk() (*string, bool)`

GetSpecializationOk returns a tuple with the Specialization field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSpecialization

`func (o *ApiNode) SetSpecialization(v string)`

SetSpecialization sets Specialization field to given value.


### GetStatus

`func (o *ApiNode) GetStatus() ApiNodeStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ApiNode) GetStatusOk() (*ApiNodeStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ApiNode) SetStatus(v ApiNodeStatus)`

SetStatus sets Status field to given value.


### GetCreatedAt

`func (o *ApiNode) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiNode) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiNode) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetUpdatedAt

`func (o *ApiNode) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ApiNode) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ApiNode) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ApiVmUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CpuCount** | Pointer to **int32** |  | [optional] 
**MemorySize** | Pointer to **int32** |  | [optional] 
**DiskSizeGb** | Pointer to **int32** |  | [optional] 

## Methods

### NewApiVmUpdateRequest

`func NewApiVmUpdateRequest() *ApiVmUpdateRequest`

NewApiVmUpdateRequest instantiates a new ApiVmUpdateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmUpdateRequestWithDefaults

`func NewApiVmUpdateRequestWithDefaults() *ApiVmUpdateRequest`

NewApiVmUpdateRequestWithDefaults instantiates a new ApiVmUpdateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCpuCount

`func (o *ApiVmUpdateRequest) GetCpuCount() int32`

GetCpuCount returns the CpuCount field if non-nil, zero value otherwise.

### GetCpuCountOk

`func (o *ApiVmUpdateRequest) GetCpuCountOk() (*int32, bool)`

GetCpuCountOk returns a tuple with the CpuCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCpuCount

`func (o *ApiVmUpdateRequest) SetCpuCount(v int32)`

SetCpuCount sets CpuCount field to given value.

### HasCpuCount

`func (o *ApiVmUpdateRequest) HasCpuCount() bool`

HasCpuCount returns a boolean if a field has been set.

### GetMemorySize

`func (o *ApiVmUpdateRequest) GetMemorySize() int32`

GetMemorySize returns the MemorySize field if non-nil, zero value otherwise.

### GetMemorySizeOk

`func (o *ApiVmUpdateRequest) GetMemorySizeOk() (*int32, bool)`

GetMemorySizeOk returns a tuple with the MemorySize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemorySize

`func (o *ApiVmUpdateRequest) SetMemorySize(v int32)`

SetMemorySize sets MemorySize field to given value.

### HasMemorySize

`func (o *ApiVmUpdateRequest) HasMemorySize() bool`

HasMemorySize returns a boolean if a field has been set.

### GetDiskSizeGb

`func (o *ApiVmUpdateRequest) GetDiskSizeGb() int32`

GetDiskSizeGb returns the DiskSizeGb field if non-nil, zero value otherwise.

### GetDiskSizeGbOk

`func (o *ApiVmUpdateRequest) GetDiskSizeGbOk() (*int32, bool)`

GetDiskSizeGbOk returns a tuple with the DiskSizeGb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDiskSizeGb

`func (o *ApiVmUpdateRequest) SetDiskSizeGb(v int32)`

SetDiskSizeGb sets DiskSizeGb field to given value.

### HasDiskSizeGb

`func (o *ApiVmUpdateRequest) HasDiskSizeGb() bool`

HasDiskSizeGb returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



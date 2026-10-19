# ApiVmCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CloudinitUserData** | **[]string** | cloud-init &#x60;runcmd&#x60; entries, de-duplicated with the selected software&#39;s commands. | 
**SoftwareIds** | Pointer to **[]string** | Software catalog ids to install. | [optional] 
**PublicKey** | **string** | SSH public key injected into the VM. | 
**CpuCount** | **int32** |  | 
**MemorySize** | **int32** | Memory in MiB. | 
**DiskSizeGb** | **int32** | Disk size in GiB. | 
**Cloning** | Pointer to **string** | Existing machine name to clone from. | [optional] 

## Methods

### NewApiVmCreateRequest

`func NewApiVmCreateRequest(cloudinitUserData []string, publicKey string, cpuCount int32, memorySize int32, diskSizeGb int32, ) *ApiVmCreateRequest`

NewApiVmCreateRequest instantiates a new ApiVmCreateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmCreateRequestWithDefaults

`func NewApiVmCreateRequestWithDefaults() *ApiVmCreateRequest`

NewApiVmCreateRequestWithDefaults instantiates a new ApiVmCreateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCloudinitUserData

`func (o *ApiVmCreateRequest) GetCloudinitUserData() []string`

GetCloudinitUserData returns the CloudinitUserData field if non-nil, zero value otherwise.

### GetCloudinitUserDataOk

`func (o *ApiVmCreateRequest) GetCloudinitUserDataOk() (*[]string, bool)`

GetCloudinitUserDataOk returns a tuple with the CloudinitUserData field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCloudinitUserData

`func (o *ApiVmCreateRequest) SetCloudinitUserData(v []string)`

SetCloudinitUserData sets CloudinitUserData field to given value.


### GetSoftwareIds

`func (o *ApiVmCreateRequest) GetSoftwareIds() []string`

GetSoftwareIds returns the SoftwareIds field if non-nil, zero value otherwise.

### GetSoftwareIdsOk

`func (o *ApiVmCreateRequest) GetSoftwareIdsOk() (*[]string, bool)`

GetSoftwareIdsOk returns a tuple with the SoftwareIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSoftwareIds

`func (o *ApiVmCreateRequest) SetSoftwareIds(v []string)`

SetSoftwareIds sets SoftwareIds field to given value.

### HasSoftwareIds

`func (o *ApiVmCreateRequest) HasSoftwareIds() bool`

HasSoftwareIds returns a boolean if a field has been set.

### GetPublicKey

`func (o *ApiVmCreateRequest) GetPublicKey() string`

GetPublicKey returns the PublicKey field if non-nil, zero value otherwise.

### GetPublicKeyOk

`func (o *ApiVmCreateRequest) GetPublicKeyOk() (*string, bool)`

GetPublicKeyOk returns a tuple with the PublicKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicKey

`func (o *ApiVmCreateRequest) SetPublicKey(v string)`

SetPublicKey sets PublicKey field to given value.


### GetCpuCount

`func (o *ApiVmCreateRequest) GetCpuCount() int32`

GetCpuCount returns the CpuCount field if non-nil, zero value otherwise.

### GetCpuCountOk

`func (o *ApiVmCreateRequest) GetCpuCountOk() (*int32, bool)`

GetCpuCountOk returns a tuple with the CpuCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCpuCount

`func (o *ApiVmCreateRequest) SetCpuCount(v int32)`

SetCpuCount sets CpuCount field to given value.


### GetMemorySize

`func (o *ApiVmCreateRequest) GetMemorySize() int32`

GetMemorySize returns the MemorySize field if non-nil, zero value otherwise.

### GetMemorySizeOk

`func (o *ApiVmCreateRequest) GetMemorySizeOk() (*int32, bool)`

GetMemorySizeOk returns a tuple with the MemorySize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemorySize

`func (o *ApiVmCreateRequest) SetMemorySize(v int32)`

SetMemorySize sets MemorySize field to given value.


### GetDiskSizeGb

`func (o *ApiVmCreateRequest) GetDiskSizeGb() int32`

GetDiskSizeGb returns the DiskSizeGb field if non-nil, zero value otherwise.

### GetDiskSizeGbOk

`func (o *ApiVmCreateRequest) GetDiskSizeGbOk() (*int32, bool)`

GetDiskSizeGbOk returns a tuple with the DiskSizeGb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDiskSizeGb

`func (o *ApiVmCreateRequest) SetDiskSizeGb(v int32)`

SetDiskSizeGb sets DiskSizeGb field to given value.


### GetCloning

`func (o *ApiVmCreateRequest) GetCloning() string`

GetCloning returns the Cloning field if non-nil, zero value otherwise.

### GetCloningOk

`func (o *ApiVmCreateRequest) GetCloningOk() (*string, bool)`

GetCloningOk returns a tuple with the Cloning field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCloning

`func (o *ApiVmCreateRequest) SetCloning(v string)`

SetCloning sets Cloning field to given value.

### HasCloning

`func (o *ApiVmCreateRequest) HasCloning() bool`

HasCloning returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



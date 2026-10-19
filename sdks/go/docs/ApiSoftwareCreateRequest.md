# ApiSoftwareCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**Name** | **string** |  | 
**DisplayName** | **string** |  | 
**Description** | Pointer to **string** |  | [optional] 
**Icon** | Pointer to **string** |  | [optional] 
**InstallCommand** | **string** |  | 
**SetupCommand** | Pointer to **string** |  | [optional] 
**RunAsUser** | Pointer to **bool** |  | [optional] 
**RecommendedVcpu** | Pointer to **int32** |  | [optional] 
**RecommendedMemory** | Pointer to **int32** |  | [optional] 
**RecommendedDisk** | Pointer to **int32** |  | [optional] 
**Category** | Pointer to **string** |  | [optional] 

## Methods

### NewApiSoftwareCreateRequest

`func NewApiSoftwareCreateRequest(id string, name string, displayName string, installCommand string, ) *ApiSoftwareCreateRequest`

NewApiSoftwareCreateRequest instantiates a new ApiSoftwareCreateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiSoftwareCreateRequestWithDefaults

`func NewApiSoftwareCreateRequestWithDefaults() *ApiSoftwareCreateRequest`

NewApiSoftwareCreateRequestWithDefaults instantiates a new ApiSoftwareCreateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiSoftwareCreateRequest) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiSoftwareCreateRequest) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiSoftwareCreateRequest) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *ApiSoftwareCreateRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiSoftwareCreateRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiSoftwareCreateRequest) SetName(v string)`

SetName sets Name field to given value.


### GetDisplayName

`func (o *ApiSoftwareCreateRequest) GetDisplayName() string`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *ApiSoftwareCreateRequest) GetDisplayNameOk() (*string, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *ApiSoftwareCreateRequest) SetDisplayName(v string)`

SetDisplayName sets DisplayName field to given value.


### GetDescription

`func (o *ApiSoftwareCreateRequest) GetDescription() string`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *ApiSoftwareCreateRequest) GetDescriptionOk() (*string, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *ApiSoftwareCreateRequest) SetDescription(v string)`

SetDescription sets Description field to given value.

### HasDescription

`func (o *ApiSoftwareCreateRequest) HasDescription() bool`

HasDescription returns a boolean if a field has been set.

### GetIcon

`func (o *ApiSoftwareCreateRequest) GetIcon() string`

GetIcon returns the Icon field if non-nil, zero value otherwise.

### GetIconOk

`func (o *ApiSoftwareCreateRequest) GetIconOk() (*string, bool)`

GetIconOk returns a tuple with the Icon field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIcon

`func (o *ApiSoftwareCreateRequest) SetIcon(v string)`

SetIcon sets Icon field to given value.

### HasIcon

`func (o *ApiSoftwareCreateRequest) HasIcon() bool`

HasIcon returns a boolean if a field has been set.

### GetInstallCommand

`func (o *ApiSoftwareCreateRequest) GetInstallCommand() string`

GetInstallCommand returns the InstallCommand field if non-nil, zero value otherwise.

### GetInstallCommandOk

`func (o *ApiSoftwareCreateRequest) GetInstallCommandOk() (*string, bool)`

GetInstallCommandOk returns a tuple with the InstallCommand field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInstallCommand

`func (o *ApiSoftwareCreateRequest) SetInstallCommand(v string)`

SetInstallCommand sets InstallCommand field to given value.


### GetSetupCommand

`func (o *ApiSoftwareCreateRequest) GetSetupCommand() string`

GetSetupCommand returns the SetupCommand field if non-nil, zero value otherwise.

### GetSetupCommandOk

`func (o *ApiSoftwareCreateRequest) GetSetupCommandOk() (*string, bool)`

GetSetupCommandOk returns a tuple with the SetupCommand field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSetupCommand

`func (o *ApiSoftwareCreateRequest) SetSetupCommand(v string)`

SetSetupCommand sets SetupCommand field to given value.

### HasSetupCommand

`func (o *ApiSoftwareCreateRequest) HasSetupCommand() bool`

HasSetupCommand returns a boolean if a field has been set.

### GetRunAsUser

`func (o *ApiSoftwareCreateRequest) GetRunAsUser() bool`

GetRunAsUser returns the RunAsUser field if non-nil, zero value otherwise.

### GetRunAsUserOk

`func (o *ApiSoftwareCreateRequest) GetRunAsUserOk() (*bool, bool)`

GetRunAsUserOk returns a tuple with the RunAsUser field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRunAsUser

`func (o *ApiSoftwareCreateRequest) SetRunAsUser(v bool)`

SetRunAsUser sets RunAsUser field to given value.

### HasRunAsUser

`func (o *ApiSoftwareCreateRequest) HasRunAsUser() bool`

HasRunAsUser returns a boolean if a field has been set.

### GetRecommendedVcpu

`func (o *ApiSoftwareCreateRequest) GetRecommendedVcpu() int32`

GetRecommendedVcpu returns the RecommendedVcpu field if non-nil, zero value otherwise.

### GetRecommendedVcpuOk

`func (o *ApiSoftwareCreateRequest) GetRecommendedVcpuOk() (*int32, bool)`

GetRecommendedVcpuOk returns a tuple with the RecommendedVcpu field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedVcpu

`func (o *ApiSoftwareCreateRequest) SetRecommendedVcpu(v int32)`

SetRecommendedVcpu sets RecommendedVcpu field to given value.

### HasRecommendedVcpu

`func (o *ApiSoftwareCreateRequest) HasRecommendedVcpu() bool`

HasRecommendedVcpu returns a boolean if a field has been set.

### GetRecommendedMemory

`func (o *ApiSoftwareCreateRequest) GetRecommendedMemory() int32`

GetRecommendedMemory returns the RecommendedMemory field if non-nil, zero value otherwise.

### GetRecommendedMemoryOk

`func (o *ApiSoftwareCreateRequest) GetRecommendedMemoryOk() (*int32, bool)`

GetRecommendedMemoryOk returns a tuple with the RecommendedMemory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedMemory

`func (o *ApiSoftwareCreateRequest) SetRecommendedMemory(v int32)`

SetRecommendedMemory sets RecommendedMemory field to given value.

### HasRecommendedMemory

`func (o *ApiSoftwareCreateRequest) HasRecommendedMemory() bool`

HasRecommendedMemory returns a boolean if a field has been set.

### GetRecommendedDisk

`func (o *ApiSoftwareCreateRequest) GetRecommendedDisk() int32`

GetRecommendedDisk returns the RecommendedDisk field if non-nil, zero value otherwise.

### GetRecommendedDiskOk

`func (o *ApiSoftwareCreateRequest) GetRecommendedDiskOk() (*int32, bool)`

GetRecommendedDiskOk returns a tuple with the RecommendedDisk field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedDisk

`func (o *ApiSoftwareCreateRequest) SetRecommendedDisk(v int32)`

SetRecommendedDisk sets RecommendedDisk field to given value.

### HasRecommendedDisk

`func (o *ApiSoftwareCreateRequest) HasRecommendedDisk() bool`

HasRecommendedDisk returns a boolean if a field has been set.

### GetCategory

`func (o *ApiSoftwareCreateRequest) GetCategory() string`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *ApiSoftwareCreateRequest) GetCategoryOk() (*string, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *ApiSoftwareCreateRequest) SetCategory(v string)`

SetCategory sets Category field to given value.

### HasCategory

`func (o *ApiSoftwareCreateRequest) HasCategory() bool`

HasCategory returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



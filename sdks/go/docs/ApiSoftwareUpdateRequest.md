# ApiSoftwareUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | Pointer to **string** |  | [optional] 
**DisplayName** | Pointer to **string** |  | [optional] 
**Description** | Pointer to **string** |  | [optional] 
**Icon** | Pointer to **string** |  | [optional] 
**InstallCommand** | Pointer to **string** |  | [optional] 
**SetupCommand** | Pointer to **string** |  | [optional] 
**RunAsUser** | Pointer to **bool** |  | [optional] 
**RecommendedVcpu** | Pointer to **int32** |  | [optional] 
**RecommendedMemory** | Pointer to **int32** |  | [optional] 
**RecommendedDisk** | Pointer to **int32** |  | [optional] 
**Category** | Pointer to **string** |  | [optional] 

## Methods

### NewApiSoftwareUpdateRequest

`func NewApiSoftwareUpdateRequest() *ApiSoftwareUpdateRequest`

NewApiSoftwareUpdateRequest instantiates a new ApiSoftwareUpdateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiSoftwareUpdateRequestWithDefaults

`func NewApiSoftwareUpdateRequestWithDefaults() *ApiSoftwareUpdateRequest`

NewApiSoftwareUpdateRequestWithDefaults instantiates a new ApiSoftwareUpdateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *ApiSoftwareUpdateRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiSoftwareUpdateRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiSoftwareUpdateRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *ApiSoftwareUpdateRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### GetDisplayName

`func (o *ApiSoftwareUpdateRequest) GetDisplayName() string`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *ApiSoftwareUpdateRequest) GetDisplayNameOk() (*string, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *ApiSoftwareUpdateRequest) SetDisplayName(v string)`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *ApiSoftwareUpdateRequest) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### GetDescription

`func (o *ApiSoftwareUpdateRequest) GetDescription() string`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *ApiSoftwareUpdateRequest) GetDescriptionOk() (*string, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *ApiSoftwareUpdateRequest) SetDescription(v string)`

SetDescription sets Description field to given value.

### HasDescription

`func (o *ApiSoftwareUpdateRequest) HasDescription() bool`

HasDescription returns a boolean if a field has been set.

### GetIcon

`func (o *ApiSoftwareUpdateRequest) GetIcon() string`

GetIcon returns the Icon field if non-nil, zero value otherwise.

### GetIconOk

`func (o *ApiSoftwareUpdateRequest) GetIconOk() (*string, bool)`

GetIconOk returns a tuple with the Icon field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIcon

`func (o *ApiSoftwareUpdateRequest) SetIcon(v string)`

SetIcon sets Icon field to given value.

### HasIcon

`func (o *ApiSoftwareUpdateRequest) HasIcon() bool`

HasIcon returns a boolean if a field has been set.

### GetInstallCommand

`func (o *ApiSoftwareUpdateRequest) GetInstallCommand() string`

GetInstallCommand returns the InstallCommand field if non-nil, zero value otherwise.

### GetInstallCommandOk

`func (o *ApiSoftwareUpdateRequest) GetInstallCommandOk() (*string, bool)`

GetInstallCommandOk returns a tuple with the InstallCommand field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInstallCommand

`func (o *ApiSoftwareUpdateRequest) SetInstallCommand(v string)`

SetInstallCommand sets InstallCommand field to given value.

### HasInstallCommand

`func (o *ApiSoftwareUpdateRequest) HasInstallCommand() bool`

HasInstallCommand returns a boolean if a field has been set.

### GetSetupCommand

`func (o *ApiSoftwareUpdateRequest) GetSetupCommand() string`

GetSetupCommand returns the SetupCommand field if non-nil, zero value otherwise.

### GetSetupCommandOk

`func (o *ApiSoftwareUpdateRequest) GetSetupCommandOk() (*string, bool)`

GetSetupCommandOk returns a tuple with the SetupCommand field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSetupCommand

`func (o *ApiSoftwareUpdateRequest) SetSetupCommand(v string)`

SetSetupCommand sets SetupCommand field to given value.

### HasSetupCommand

`func (o *ApiSoftwareUpdateRequest) HasSetupCommand() bool`

HasSetupCommand returns a boolean if a field has been set.

### GetRunAsUser

`func (o *ApiSoftwareUpdateRequest) GetRunAsUser() bool`

GetRunAsUser returns the RunAsUser field if non-nil, zero value otherwise.

### GetRunAsUserOk

`func (o *ApiSoftwareUpdateRequest) GetRunAsUserOk() (*bool, bool)`

GetRunAsUserOk returns a tuple with the RunAsUser field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRunAsUser

`func (o *ApiSoftwareUpdateRequest) SetRunAsUser(v bool)`

SetRunAsUser sets RunAsUser field to given value.

### HasRunAsUser

`func (o *ApiSoftwareUpdateRequest) HasRunAsUser() bool`

HasRunAsUser returns a boolean if a field has been set.

### GetRecommendedVcpu

`func (o *ApiSoftwareUpdateRequest) GetRecommendedVcpu() int32`

GetRecommendedVcpu returns the RecommendedVcpu field if non-nil, zero value otherwise.

### GetRecommendedVcpuOk

`func (o *ApiSoftwareUpdateRequest) GetRecommendedVcpuOk() (*int32, bool)`

GetRecommendedVcpuOk returns a tuple with the RecommendedVcpu field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedVcpu

`func (o *ApiSoftwareUpdateRequest) SetRecommendedVcpu(v int32)`

SetRecommendedVcpu sets RecommendedVcpu field to given value.

### HasRecommendedVcpu

`func (o *ApiSoftwareUpdateRequest) HasRecommendedVcpu() bool`

HasRecommendedVcpu returns a boolean if a field has been set.

### GetRecommendedMemory

`func (o *ApiSoftwareUpdateRequest) GetRecommendedMemory() int32`

GetRecommendedMemory returns the RecommendedMemory field if non-nil, zero value otherwise.

### GetRecommendedMemoryOk

`func (o *ApiSoftwareUpdateRequest) GetRecommendedMemoryOk() (*int32, bool)`

GetRecommendedMemoryOk returns a tuple with the RecommendedMemory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedMemory

`func (o *ApiSoftwareUpdateRequest) SetRecommendedMemory(v int32)`

SetRecommendedMemory sets RecommendedMemory field to given value.

### HasRecommendedMemory

`func (o *ApiSoftwareUpdateRequest) HasRecommendedMemory() bool`

HasRecommendedMemory returns a boolean if a field has been set.

### GetRecommendedDisk

`func (o *ApiSoftwareUpdateRequest) GetRecommendedDisk() int32`

GetRecommendedDisk returns the RecommendedDisk field if non-nil, zero value otherwise.

### GetRecommendedDiskOk

`func (o *ApiSoftwareUpdateRequest) GetRecommendedDiskOk() (*int32, bool)`

GetRecommendedDiskOk returns a tuple with the RecommendedDisk field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedDisk

`func (o *ApiSoftwareUpdateRequest) SetRecommendedDisk(v int32)`

SetRecommendedDisk sets RecommendedDisk field to given value.

### HasRecommendedDisk

`func (o *ApiSoftwareUpdateRequest) HasRecommendedDisk() bool`

HasRecommendedDisk returns a boolean if a field has been set.

### GetCategory

`func (o *ApiSoftwareUpdateRequest) GetCategory() string`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *ApiSoftwareUpdateRequest) GetCategoryOk() (*string, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *ApiSoftwareUpdateRequest) SetCategory(v string)`

SetCategory sets Category field to given value.

### HasCategory

`func (o *ApiSoftwareUpdateRequest) HasCategory() bool`

HasCategory returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ApiSoftware

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**Name** | **string** |  | 
**DisplayName** | **string** |  | 
**Description** | **NullableString** |  | 
**Icon** | **NullableString** |  | 
**InstallCommand** | **string** |  | 
**SetupCommand** | **NullableString** |  | 
**RunAsUser** | **bool** |  | 
**RecommendedVcpu** | **NullableInt32** |  | 
**RecommendedMemory** | **NullableInt32** |  | 
**RecommendedDisk** | **NullableInt32** |  | 
**Category** | **NullableString** |  | 
**CreatedAt** | **time.Time** |  | 
**UpdatedAt** | **time.Time** |  | 

## Methods

### NewApiSoftware

`func NewApiSoftware(id string, name string, displayName string, description NullableString, icon NullableString, installCommand string, setupCommand NullableString, runAsUser bool, recommendedVcpu NullableInt32, recommendedMemory NullableInt32, recommendedDisk NullableInt32, category NullableString, createdAt time.Time, updatedAt time.Time, ) *ApiSoftware`

NewApiSoftware instantiates a new ApiSoftware object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiSoftwareWithDefaults

`func NewApiSoftwareWithDefaults() *ApiSoftware`

NewApiSoftwareWithDefaults instantiates a new ApiSoftware object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiSoftware) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiSoftware) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiSoftware) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *ApiSoftware) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiSoftware) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiSoftware) SetName(v string)`

SetName sets Name field to given value.


### GetDisplayName

`func (o *ApiSoftware) GetDisplayName() string`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *ApiSoftware) GetDisplayNameOk() (*string, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *ApiSoftware) SetDisplayName(v string)`

SetDisplayName sets DisplayName field to given value.


### GetDescription

`func (o *ApiSoftware) GetDescription() string`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *ApiSoftware) GetDescriptionOk() (*string, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *ApiSoftware) SetDescription(v string)`

SetDescription sets Description field to given value.


### SetDescriptionNil

`func (o *ApiSoftware) SetDescriptionNil(b bool)`

 SetDescriptionNil sets the value for Description to be an explicit nil

### UnsetDescription
`func (o *ApiSoftware) UnsetDescription()`

UnsetDescription ensures that no value is present for Description, not even an explicit nil
### GetIcon

`func (o *ApiSoftware) GetIcon() string`

GetIcon returns the Icon field if non-nil, zero value otherwise.

### GetIconOk

`func (o *ApiSoftware) GetIconOk() (*string, bool)`

GetIconOk returns a tuple with the Icon field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIcon

`func (o *ApiSoftware) SetIcon(v string)`

SetIcon sets Icon field to given value.


### SetIconNil

`func (o *ApiSoftware) SetIconNil(b bool)`

 SetIconNil sets the value for Icon to be an explicit nil

### UnsetIcon
`func (o *ApiSoftware) UnsetIcon()`

UnsetIcon ensures that no value is present for Icon, not even an explicit nil
### GetInstallCommand

`func (o *ApiSoftware) GetInstallCommand() string`

GetInstallCommand returns the InstallCommand field if non-nil, zero value otherwise.

### GetInstallCommandOk

`func (o *ApiSoftware) GetInstallCommandOk() (*string, bool)`

GetInstallCommandOk returns a tuple with the InstallCommand field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInstallCommand

`func (o *ApiSoftware) SetInstallCommand(v string)`

SetInstallCommand sets InstallCommand field to given value.


### GetSetupCommand

`func (o *ApiSoftware) GetSetupCommand() string`

GetSetupCommand returns the SetupCommand field if non-nil, zero value otherwise.

### GetSetupCommandOk

`func (o *ApiSoftware) GetSetupCommandOk() (*string, bool)`

GetSetupCommandOk returns a tuple with the SetupCommand field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSetupCommand

`func (o *ApiSoftware) SetSetupCommand(v string)`

SetSetupCommand sets SetupCommand field to given value.


### SetSetupCommandNil

`func (o *ApiSoftware) SetSetupCommandNil(b bool)`

 SetSetupCommandNil sets the value for SetupCommand to be an explicit nil

### UnsetSetupCommand
`func (o *ApiSoftware) UnsetSetupCommand()`

UnsetSetupCommand ensures that no value is present for SetupCommand, not even an explicit nil
### GetRunAsUser

`func (o *ApiSoftware) GetRunAsUser() bool`

GetRunAsUser returns the RunAsUser field if non-nil, zero value otherwise.

### GetRunAsUserOk

`func (o *ApiSoftware) GetRunAsUserOk() (*bool, bool)`

GetRunAsUserOk returns a tuple with the RunAsUser field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRunAsUser

`func (o *ApiSoftware) SetRunAsUser(v bool)`

SetRunAsUser sets RunAsUser field to given value.


### GetRecommendedVcpu

`func (o *ApiSoftware) GetRecommendedVcpu() int32`

GetRecommendedVcpu returns the RecommendedVcpu field if non-nil, zero value otherwise.

### GetRecommendedVcpuOk

`func (o *ApiSoftware) GetRecommendedVcpuOk() (*int32, bool)`

GetRecommendedVcpuOk returns a tuple with the RecommendedVcpu field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedVcpu

`func (o *ApiSoftware) SetRecommendedVcpu(v int32)`

SetRecommendedVcpu sets RecommendedVcpu field to given value.


### SetRecommendedVcpuNil

`func (o *ApiSoftware) SetRecommendedVcpuNil(b bool)`

 SetRecommendedVcpuNil sets the value for RecommendedVcpu to be an explicit nil

### UnsetRecommendedVcpu
`func (o *ApiSoftware) UnsetRecommendedVcpu()`

UnsetRecommendedVcpu ensures that no value is present for RecommendedVcpu, not even an explicit nil
### GetRecommendedMemory

`func (o *ApiSoftware) GetRecommendedMemory() int32`

GetRecommendedMemory returns the RecommendedMemory field if non-nil, zero value otherwise.

### GetRecommendedMemoryOk

`func (o *ApiSoftware) GetRecommendedMemoryOk() (*int32, bool)`

GetRecommendedMemoryOk returns a tuple with the RecommendedMemory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedMemory

`func (o *ApiSoftware) SetRecommendedMemory(v int32)`

SetRecommendedMemory sets RecommendedMemory field to given value.


### SetRecommendedMemoryNil

`func (o *ApiSoftware) SetRecommendedMemoryNil(b bool)`

 SetRecommendedMemoryNil sets the value for RecommendedMemory to be an explicit nil

### UnsetRecommendedMemory
`func (o *ApiSoftware) UnsetRecommendedMemory()`

UnsetRecommendedMemory ensures that no value is present for RecommendedMemory, not even an explicit nil
### GetRecommendedDisk

`func (o *ApiSoftware) GetRecommendedDisk() int32`

GetRecommendedDisk returns the RecommendedDisk field if non-nil, zero value otherwise.

### GetRecommendedDiskOk

`func (o *ApiSoftware) GetRecommendedDiskOk() (*int32, bool)`

GetRecommendedDiskOk returns a tuple with the RecommendedDisk field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendedDisk

`func (o *ApiSoftware) SetRecommendedDisk(v int32)`

SetRecommendedDisk sets RecommendedDisk field to given value.


### SetRecommendedDiskNil

`func (o *ApiSoftware) SetRecommendedDiskNil(b bool)`

 SetRecommendedDiskNil sets the value for RecommendedDisk to be an explicit nil

### UnsetRecommendedDisk
`func (o *ApiSoftware) UnsetRecommendedDisk()`

UnsetRecommendedDisk ensures that no value is present for RecommendedDisk, not even an explicit nil
### GetCategory

`func (o *ApiSoftware) GetCategory() string`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *ApiSoftware) GetCategoryOk() (*string, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *ApiSoftware) SetCategory(v string)`

SetCategory sets Category field to given value.


### SetCategoryNil

`func (o *ApiSoftware) SetCategoryNil(b bool)`

 SetCategoryNil sets the value for Category to be an explicit nil

### UnsetCategory
`func (o *ApiSoftware) UnsetCategory()`

UnsetCategory ensures that no value is present for Category, not even an explicit nil
### GetCreatedAt

`func (o *ApiSoftware) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiSoftware) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiSoftware) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetUpdatedAt

`func (o *ApiSoftware) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ApiSoftware) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ApiSoftware) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ApiRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**Name** | **string** |  | 
**MicrovmId** | **string** |  | 
**Privacy** | **string** |  | 
**Port** | **int32** |  | 
**UserIds** | **[]string** |  | 
**MachineName** | **string** |  | 
**MachineUserId** | **string** |  | 
**NodeName** | **string** |  | 
**IpAddress** | **NullableString** |  | 
**OwnerUsername** | **NullableString** |  | 
**OwnerEmail** | **string** |  | 
**CreatedAt** | **time.Time** |  | 
**UpdatedAt** | **time.Time** |  | 
**Url** | **string** |  | 
**SharedUsers** | [**[]SharedUser**](SharedUser.md) |  | 

## Methods

### NewApiRoute

`func NewApiRoute(id string, name string, microvmId string, privacy string, port int32, userIds []string, machineName string, machineUserId string, nodeName string, ipAddress NullableString, ownerUsername NullableString, ownerEmail string, createdAt time.Time, updatedAt time.Time, url string, sharedUsers []SharedUser, ) *ApiRoute`

NewApiRoute instantiates a new ApiRoute object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiRouteWithDefaults

`func NewApiRouteWithDefaults() *ApiRoute`

NewApiRouteWithDefaults instantiates a new ApiRoute object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiRoute) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiRoute) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiRoute) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *ApiRoute) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiRoute) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiRoute) SetName(v string)`

SetName sets Name field to given value.


### GetMicrovmId

`func (o *ApiRoute) GetMicrovmId() string`

GetMicrovmId returns the MicrovmId field if non-nil, zero value otherwise.

### GetMicrovmIdOk

`func (o *ApiRoute) GetMicrovmIdOk() (*string, bool)`

GetMicrovmIdOk returns a tuple with the MicrovmId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMicrovmId

`func (o *ApiRoute) SetMicrovmId(v string)`

SetMicrovmId sets MicrovmId field to given value.


### GetPrivacy

`func (o *ApiRoute) GetPrivacy() string`

GetPrivacy returns the Privacy field if non-nil, zero value otherwise.

### GetPrivacyOk

`func (o *ApiRoute) GetPrivacyOk() (*string, bool)`

GetPrivacyOk returns a tuple with the Privacy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrivacy

`func (o *ApiRoute) SetPrivacy(v string)`

SetPrivacy sets Privacy field to given value.


### GetPort

`func (o *ApiRoute) GetPort() int32`

GetPort returns the Port field if non-nil, zero value otherwise.

### GetPortOk

`func (o *ApiRoute) GetPortOk() (*int32, bool)`

GetPortOk returns a tuple with the Port field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPort

`func (o *ApiRoute) SetPort(v int32)`

SetPort sets Port field to given value.


### GetUserIds

`func (o *ApiRoute) GetUserIds() []string`

GetUserIds returns the UserIds field if non-nil, zero value otherwise.

### GetUserIdsOk

`func (o *ApiRoute) GetUserIdsOk() (*[]string, bool)`

GetUserIdsOk returns a tuple with the UserIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserIds

`func (o *ApiRoute) SetUserIds(v []string)`

SetUserIds sets UserIds field to given value.


### GetMachineName

`func (o *ApiRoute) GetMachineName() string`

GetMachineName returns the MachineName field if non-nil, zero value otherwise.

### GetMachineNameOk

`func (o *ApiRoute) GetMachineNameOk() (*string, bool)`

GetMachineNameOk returns a tuple with the MachineName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMachineName

`func (o *ApiRoute) SetMachineName(v string)`

SetMachineName sets MachineName field to given value.


### GetMachineUserId

`func (o *ApiRoute) GetMachineUserId() string`

GetMachineUserId returns the MachineUserId field if non-nil, zero value otherwise.

### GetMachineUserIdOk

`func (o *ApiRoute) GetMachineUserIdOk() (*string, bool)`

GetMachineUserIdOk returns a tuple with the MachineUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMachineUserId

`func (o *ApiRoute) SetMachineUserId(v string)`

SetMachineUserId sets MachineUserId field to given value.


### GetNodeName

`func (o *ApiRoute) GetNodeName() string`

GetNodeName returns the NodeName field if non-nil, zero value otherwise.

### GetNodeNameOk

`func (o *ApiRoute) GetNodeNameOk() (*string, bool)`

GetNodeNameOk returns a tuple with the NodeName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNodeName

`func (o *ApiRoute) SetNodeName(v string)`

SetNodeName sets NodeName field to given value.


### GetIpAddress

`func (o *ApiRoute) GetIpAddress() string`

GetIpAddress returns the IpAddress field if non-nil, zero value otherwise.

### GetIpAddressOk

`func (o *ApiRoute) GetIpAddressOk() (*string, bool)`

GetIpAddressOk returns a tuple with the IpAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIpAddress

`func (o *ApiRoute) SetIpAddress(v string)`

SetIpAddress sets IpAddress field to given value.


### SetIpAddressNil

`func (o *ApiRoute) SetIpAddressNil(b bool)`

 SetIpAddressNil sets the value for IpAddress to be an explicit nil

### UnsetIpAddress
`func (o *ApiRoute) UnsetIpAddress()`

UnsetIpAddress ensures that no value is present for IpAddress, not even an explicit nil
### GetOwnerUsername

`func (o *ApiRoute) GetOwnerUsername() string`

GetOwnerUsername returns the OwnerUsername field if non-nil, zero value otherwise.

### GetOwnerUsernameOk

`func (o *ApiRoute) GetOwnerUsernameOk() (*string, bool)`

GetOwnerUsernameOk returns a tuple with the OwnerUsername field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOwnerUsername

`func (o *ApiRoute) SetOwnerUsername(v string)`

SetOwnerUsername sets OwnerUsername field to given value.


### SetOwnerUsernameNil

`func (o *ApiRoute) SetOwnerUsernameNil(b bool)`

 SetOwnerUsernameNil sets the value for OwnerUsername to be an explicit nil

### UnsetOwnerUsername
`func (o *ApiRoute) UnsetOwnerUsername()`

UnsetOwnerUsername ensures that no value is present for OwnerUsername, not even an explicit nil
### GetOwnerEmail

`func (o *ApiRoute) GetOwnerEmail() string`

GetOwnerEmail returns the OwnerEmail field if non-nil, zero value otherwise.

### GetOwnerEmailOk

`func (o *ApiRoute) GetOwnerEmailOk() (*string, bool)`

GetOwnerEmailOk returns a tuple with the OwnerEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOwnerEmail

`func (o *ApiRoute) SetOwnerEmail(v string)`

SetOwnerEmail sets OwnerEmail field to given value.


### GetCreatedAt

`func (o *ApiRoute) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiRoute) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiRoute) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetUpdatedAt

`func (o *ApiRoute) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ApiRoute) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ApiRoute) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.


### GetUrl

`func (o *ApiRoute) GetUrl() string`

GetUrl returns the Url field if non-nil, zero value otherwise.

### GetUrlOk

`func (o *ApiRoute) GetUrlOk() (*string, bool)`

GetUrlOk returns a tuple with the Url field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUrl

`func (o *ApiRoute) SetUrl(v string)`

SetUrl sets Url field to given value.


### GetSharedUsers

`func (o *ApiRoute) GetSharedUsers() []SharedUser`

GetSharedUsers returns the SharedUsers field if non-nil, zero value otherwise.

### GetSharedUsersOk

`func (o *ApiRoute) GetSharedUsersOk() (*[]SharedUser, bool)`

GetSharedUsersOk returns a tuple with the SharedUsers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSharedUsers

`func (o *ApiRoute) SetSharedUsers(v []SharedUser)`

SetSharedUsers sets SharedUsers field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



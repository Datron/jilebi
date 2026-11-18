# ApiNodeMaintenanceRunResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Node** | **string** |  | 
**Phase** | **string** |  | 
**Running** | **bool** | True while the run is working in the background. | 
**StartedAt** | Pointer to **NullableTime** |  | [optional] 
**RoutesPending** | **bool** | True when every VM is back but some app routes still need replaying. | 
**Entries** | [**[]ApiNodeMaintenanceEntry**](ApiNodeMaintenanceEntry.md) |  | 
**Message** | **string** | Human summary of the run. | 

## Methods

### NewApiNodeMaintenanceRunResponse

`func NewApiNodeMaintenanceRunResponse(node string, phase string, running bool, routesPending bool, entries []ApiNodeMaintenanceEntry, message string, ) *ApiNodeMaintenanceRunResponse`

NewApiNodeMaintenanceRunResponse instantiates a new ApiNodeMaintenanceRunResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiNodeMaintenanceRunResponseWithDefaults

`func NewApiNodeMaintenanceRunResponseWithDefaults() *ApiNodeMaintenanceRunResponse`

NewApiNodeMaintenanceRunResponseWithDefaults instantiates a new ApiNodeMaintenanceRunResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNode

`func (o *ApiNodeMaintenanceRunResponse) GetNode() string`

GetNode returns the Node field if non-nil, zero value otherwise.

### GetNodeOk

`func (o *ApiNodeMaintenanceRunResponse) GetNodeOk() (*string, bool)`

GetNodeOk returns a tuple with the Node field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNode

`func (o *ApiNodeMaintenanceRunResponse) SetNode(v string)`

SetNode sets Node field to given value.


### GetPhase

`func (o *ApiNodeMaintenanceRunResponse) GetPhase() string`

GetPhase returns the Phase field if non-nil, zero value otherwise.

### GetPhaseOk

`func (o *ApiNodeMaintenanceRunResponse) GetPhaseOk() (*string, bool)`

GetPhaseOk returns a tuple with the Phase field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhase

`func (o *ApiNodeMaintenanceRunResponse) SetPhase(v string)`

SetPhase sets Phase field to given value.


### GetRunning

`func (o *ApiNodeMaintenanceRunResponse) GetRunning() bool`

GetRunning returns the Running field if non-nil, zero value otherwise.

### GetRunningOk

`func (o *ApiNodeMaintenanceRunResponse) GetRunningOk() (*bool, bool)`

GetRunningOk returns a tuple with the Running field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRunning

`func (o *ApiNodeMaintenanceRunResponse) SetRunning(v bool)`

SetRunning sets Running field to given value.


### GetStartedAt

`func (o *ApiNodeMaintenanceRunResponse) GetStartedAt() time.Time`

GetStartedAt returns the StartedAt field if non-nil, zero value otherwise.

### GetStartedAtOk

`func (o *ApiNodeMaintenanceRunResponse) GetStartedAtOk() (*time.Time, bool)`

GetStartedAtOk returns a tuple with the StartedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStartedAt

`func (o *ApiNodeMaintenanceRunResponse) SetStartedAt(v time.Time)`

SetStartedAt sets StartedAt field to given value.

### HasStartedAt

`func (o *ApiNodeMaintenanceRunResponse) HasStartedAt() bool`

HasStartedAt returns a boolean if a field has been set.

### SetStartedAtNil

`func (o *ApiNodeMaintenanceRunResponse) SetStartedAtNil(b bool)`

 SetStartedAtNil sets the value for StartedAt to be an explicit nil

### UnsetStartedAt
`func (o *ApiNodeMaintenanceRunResponse) UnsetStartedAt()`

UnsetStartedAt ensures that no value is present for StartedAt, not even an explicit nil
### GetRoutesPending

`func (o *ApiNodeMaintenanceRunResponse) GetRoutesPending() bool`

GetRoutesPending returns the RoutesPending field if non-nil, zero value otherwise.

### GetRoutesPendingOk

`func (o *ApiNodeMaintenanceRunResponse) GetRoutesPendingOk() (*bool, bool)`

GetRoutesPendingOk returns a tuple with the RoutesPending field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRoutesPending

`func (o *ApiNodeMaintenanceRunResponse) SetRoutesPending(v bool)`

SetRoutesPending sets RoutesPending field to given value.


### GetEntries

`func (o *ApiNodeMaintenanceRunResponse) GetEntries() []ApiNodeMaintenanceEntry`

GetEntries returns the Entries field if non-nil, zero value otherwise.

### GetEntriesOk

`func (o *ApiNodeMaintenanceRunResponse) GetEntriesOk() (*[]ApiNodeMaintenanceEntry, bool)`

GetEntriesOk returns a tuple with the Entries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntries

`func (o *ApiNodeMaintenanceRunResponse) SetEntries(v []ApiNodeMaintenanceEntry)`

SetEntries sets Entries field to given value.


### GetMessage

`func (o *ApiNodeMaintenanceRunResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *ApiNodeMaintenanceRunResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *ApiNodeMaintenanceRunResponse) SetMessage(v string)`

SetMessage sets Message field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



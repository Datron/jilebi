# ApiNodeMaintenanceStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Node** | **string** |  | 
**Phase** | **string** |  | 
**Running** | **bool** | True while the run is working in the background. | 
**StartedAt** | Pointer to **NullableTime** |  | [optional] 
**RoutesPending** | **bool** | True when every VM is back but some app routes still need replaying. | 
**Entries** | [**[]ApiNodeMaintenanceEntry**](ApiNodeMaintenanceEntry.md) |  | 

## Methods

### NewApiNodeMaintenanceStatus

`func NewApiNodeMaintenanceStatus(node string, phase string, running bool, routesPending bool, entries []ApiNodeMaintenanceEntry, ) *ApiNodeMaintenanceStatus`

NewApiNodeMaintenanceStatus instantiates a new ApiNodeMaintenanceStatus object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiNodeMaintenanceStatusWithDefaults

`func NewApiNodeMaintenanceStatusWithDefaults() *ApiNodeMaintenanceStatus`

NewApiNodeMaintenanceStatusWithDefaults instantiates a new ApiNodeMaintenanceStatus object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNode

`func (o *ApiNodeMaintenanceStatus) GetNode() string`

GetNode returns the Node field if non-nil, zero value otherwise.

### GetNodeOk

`func (o *ApiNodeMaintenanceStatus) GetNodeOk() (*string, bool)`

GetNodeOk returns a tuple with the Node field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNode

`func (o *ApiNodeMaintenanceStatus) SetNode(v string)`

SetNode sets Node field to given value.


### GetPhase

`func (o *ApiNodeMaintenanceStatus) GetPhase() string`

GetPhase returns the Phase field if non-nil, zero value otherwise.

### GetPhaseOk

`func (o *ApiNodeMaintenanceStatus) GetPhaseOk() (*string, bool)`

GetPhaseOk returns a tuple with the Phase field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhase

`func (o *ApiNodeMaintenanceStatus) SetPhase(v string)`

SetPhase sets Phase field to given value.


### GetRunning

`func (o *ApiNodeMaintenanceStatus) GetRunning() bool`

GetRunning returns the Running field if non-nil, zero value otherwise.

### GetRunningOk

`func (o *ApiNodeMaintenanceStatus) GetRunningOk() (*bool, bool)`

GetRunningOk returns a tuple with the Running field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRunning

`func (o *ApiNodeMaintenanceStatus) SetRunning(v bool)`

SetRunning sets Running field to given value.


### GetStartedAt

`func (o *ApiNodeMaintenanceStatus) GetStartedAt() time.Time`

GetStartedAt returns the StartedAt field if non-nil, zero value otherwise.

### GetStartedAtOk

`func (o *ApiNodeMaintenanceStatus) GetStartedAtOk() (*time.Time, bool)`

GetStartedAtOk returns a tuple with the StartedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStartedAt

`func (o *ApiNodeMaintenanceStatus) SetStartedAt(v time.Time)`

SetStartedAt sets StartedAt field to given value.

### HasStartedAt

`func (o *ApiNodeMaintenanceStatus) HasStartedAt() bool`

HasStartedAt returns a boolean if a field has been set.

### SetStartedAtNil

`func (o *ApiNodeMaintenanceStatus) SetStartedAtNil(b bool)`

 SetStartedAtNil sets the value for StartedAt to be an explicit nil

### UnsetStartedAt
`func (o *ApiNodeMaintenanceStatus) UnsetStartedAt()`

UnsetStartedAt ensures that no value is present for StartedAt, not even an explicit nil
### GetRoutesPending

`func (o *ApiNodeMaintenanceStatus) GetRoutesPending() bool`

GetRoutesPending returns the RoutesPending field if non-nil, zero value otherwise.

### GetRoutesPendingOk

`func (o *ApiNodeMaintenanceStatus) GetRoutesPendingOk() (*bool, bool)`

GetRoutesPendingOk returns a tuple with the RoutesPending field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRoutesPending

`func (o *ApiNodeMaintenanceStatus) SetRoutesPending(v bool)`

SetRoutesPending sets RoutesPending field to given value.


### GetEntries

`func (o *ApiNodeMaintenanceStatus) GetEntries() []ApiNodeMaintenanceEntry`

GetEntries returns the Entries field if non-nil, zero value otherwise.

### GetEntriesOk

`func (o *ApiNodeMaintenanceStatus) GetEntriesOk() (*[]ApiNodeMaintenanceEntry, bool)`

GetEntriesOk returns a tuple with the Entries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntries

`func (o *ApiNodeMaintenanceStatus) SetEntries(v []ApiNodeMaintenanceEntry)`

SetEntries sets Entries field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



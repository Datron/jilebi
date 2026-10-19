# ApiBillingGenerateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Username** | **string** |  | 
**Start** | **NullableInt32** | Window start (epoch ms). | 
**End** | **NullableInt32** | Window end (epoch ms). | 
**MachinesChecked** | **int32** |  | 
**BillsCreated** | **int32** |  | 
**EmailsSent** | **int32** |  | 
**Bills** | [**[]ApiBillingGenerateResponseBillsInner**](ApiBillingGenerateResponseBillsInner.md) |  | 

## Methods

### NewApiBillingGenerateResponse

`func NewApiBillingGenerateResponse(username string, start NullableInt32, end NullableInt32, machinesChecked int32, billsCreated int32, emailsSent int32, bills []ApiBillingGenerateResponseBillsInner, ) *ApiBillingGenerateResponse`

NewApiBillingGenerateResponse instantiates a new ApiBillingGenerateResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiBillingGenerateResponseWithDefaults

`func NewApiBillingGenerateResponseWithDefaults() *ApiBillingGenerateResponse`

NewApiBillingGenerateResponseWithDefaults instantiates a new ApiBillingGenerateResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetUsername

`func (o *ApiBillingGenerateResponse) GetUsername() string`

GetUsername returns the Username field if non-nil, zero value otherwise.

### GetUsernameOk

`func (o *ApiBillingGenerateResponse) GetUsernameOk() (*string, bool)`

GetUsernameOk returns a tuple with the Username field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsername

`func (o *ApiBillingGenerateResponse) SetUsername(v string)`

SetUsername sets Username field to given value.


### GetStart

`func (o *ApiBillingGenerateResponse) GetStart() int32`

GetStart returns the Start field if non-nil, zero value otherwise.

### GetStartOk

`func (o *ApiBillingGenerateResponse) GetStartOk() (*int32, bool)`

GetStartOk returns a tuple with the Start field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStart

`func (o *ApiBillingGenerateResponse) SetStart(v int32)`

SetStart sets Start field to given value.


### SetStartNil

`func (o *ApiBillingGenerateResponse) SetStartNil(b bool)`

 SetStartNil sets the value for Start to be an explicit nil

### UnsetStart
`func (o *ApiBillingGenerateResponse) UnsetStart()`

UnsetStart ensures that no value is present for Start, not even an explicit nil
### GetEnd

`func (o *ApiBillingGenerateResponse) GetEnd() int32`

GetEnd returns the End field if non-nil, zero value otherwise.

### GetEndOk

`func (o *ApiBillingGenerateResponse) GetEndOk() (*int32, bool)`

GetEndOk returns a tuple with the End field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnd

`func (o *ApiBillingGenerateResponse) SetEnd(v int32)`

SetEnd sets End field to given value.


### SetEndNil

`func (o *ApiBillingGenerateResponse) SetEndNil(b bool)`

 SetEndNil sets the value for End to be an explicit nil

### UnsetEnd
`func (o *ApiBillingGenerateResponse) UnsetEnd()`

UnsetEnd ensures that no value is present for End, not even an explicit nil
### GetMachinesChecked

`func (o *ApiBillingGenerateResponse) GetMachinesChecked() int32`

GetMachinesChecked returns the MachinesChecked field if non-nil, zero value otherwise.

### GetMachinesCheckedOk

`func (o *ApiBillingGenerateResponse) GetMachinesCheckedOk() (*int32, bool)`

GetMachinesCheckedOk returns a tuple with the MachinesChecked field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMachinesChecked

`func (o *ApiBillingGenerateResponse) SetMachinesChecked(v int32)`

SetMachinesChecked sets MachinesChecked field to given value.


### GetBillsCreated

`func (o *ApiBillingGenerateResponse) GetBillsCreated() int32`

GetBillsCreated returns the BillsCreated field if non-nil, zero value otherwise.

### GetBillsCreatedOk

`func (o *ApiBillingGenerateResponse) GetBillsCreatedOk() (*int32, bool)`

GetBillsCreatedOk returns a tuple with the BillsCreated field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBillsCreated

`func (o *ApiBillingGenerateResponse) SetBillsCreated(v int32)`

SetBillsCreated sets BillsCreated field to given value.


### GetEmailsSent

`func (o *ApiBillingGenerateResponse) GetEmailsSent() int32`

GetEmailsSent returns the EmailsSent field if non-nil, zero value otherwise.

### GetEmailsSentOk

`func (o *ApiBillingGenerateResponse) GetEmailsSentOk() (*int32, bool)`

GetEmailsSentOk returns a tuple with the EmailsSent field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailsSent

`func (o *ApiBillingGenerateResponse) SetEmailsSent(v int32)`

SetEmailsSent sets EmailsSent field to given value.


### GetBills

`func (o *ApiBillingGenerateResponse) GetBills() []ApiBillingGenerateResponseBillsInner`

GetBills returns the Bills field if non-nil, zero value otherwise.

### GetBillsOk

`func (o *ApiBillingGenerateResponse) GetBillsOk() (*[]ApiBillingGenerateResponseBillsInner, bool)`

GetBillsOk returns a tuple with the Bills field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBills

`func (o *ApiBillingGenerateResponse) SetBills(v []ApiBillingGenerateResponseBillsInner)`

SetBills sets Bills field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



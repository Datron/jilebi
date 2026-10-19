# \BillingApi

All URIs are relative to *https://jilebi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_v1_billing_admin_users**](BillingApi.md#get_api_v1_billing_admin_users) | **GET** /api/v1/billing/admin/users | List billable users
[**post_api_v1_billing_admin_generate_by_username**](BillingApi.md#post_api_v1_billing_admin_generate_by_username) | **POST** /api/v1/billing/admin/generate/{username} | Generate an invoice
[**post_api_v1_billing_admin_reset_dodo_by_username**](BillingApi.md#post_api_v1_billing_admin_reset_dodo_by_username) | **POST** /api/v1/billing/admin/reset-dodo/{username} | Reset a user's Dodo data
[**post_api_v1_billing_pay_by_bill_id**](BillingApi.md#post_api_v1_billing_pay_by_bill_id) | **POST** /api/v1/billing/pay/{billId} | Pay an invoice



## get_api_v1_billing_admin_users

> models::ApiBillingUsersResponse get_api_v1_billing_admin_users()
List billable users

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiBillingUsersResponse**](ApiBillingUsersResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_billing_admin_generate_by_username

> models::ApiBillingGenerateResponse post_api_v1_billing_admin_generate_by_username(username, start, end)
Generate an invoice

Generates (and emails) an invoice for a user over an optional time window.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**start** | Option<**chrono::NaiveDate**> | Window start (YYYY-MM-DD). |  |
**end** | Option<**chrono::NaiveDate**> | Window end (YYYY-MM-DD). |  |

### Return type

[**models::ApiBillingGenerateResponse**](ApiBillingGenerateResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_billing_admin_reset_dodo_by_username

> models::ApiBillingResetDodoResponse post_api_v1_billing_admin_reset_dodo_by_username(username)
Reset a user's Dodo data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Username of the account to reset. | [required] |

### Return type

[**models::ApiBillingResetDodoResponse**](ApiBillingResetDodoResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v1_billing_pay_by_bill_id

> post_api_v1_billing_pay_by_bill_id(bill_id)
Pay an invoice

HTMX endpoint. Charges the saved mandate, or redirects to a Dodo checkout through the `HX-Redirect` response header (empty body). Failures return an HTML fragment targeting `#pay-error`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bill_id** | **String** | Id of the invoice to pay. | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [sessionCookie](../README.md#sessionCookie), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


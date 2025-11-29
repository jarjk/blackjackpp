# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bet**](DefaultApi.md#bet) | **POST** /bet/{username} | 
[**game_state_of**](DefaultApi.md#game_state_of) | **GET** /game_state/{username} | 
[**index**](DefaultApi.md#index) | **GET** / | 
[**join**](DefaultApi.md#join) | **GET** /join | 
[**make_move**](DefaultApi.md#make_move) | **POST** /move/{username} | 



## bet

> models::Game bet(username, amount)


make a bet of `amount` for `username` requires a game waiting for bet of `username` `bet` shall be non-zero, but shouldn't exceed user wealth deals and so might end the game with a blackjack returns game state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**amount** | **i32** |  | [required] |

### Return type

[**models::Game**](Game.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_state_of

> models::Game game_state_of(username)


get the game state of a specific `username`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**models::Game**](Game.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index

> index()


index, redirect to git repo

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join

> String join(username)


join the game as `username` if already joined and finished playing, rejoins NOTE: currently each game has it's own dealer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## make_move

> models::Game make_move(username, action)


make a [`move`](MoveAction) for `username` requires a previously made [bet] might end the game returns game state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**action** | **String** | what a user can do during their turn | [required] |

### Return type

[**models::Game**](Game.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


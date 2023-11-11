# Playlist

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**privacy** | **String** |  | 
**editable** | **bool** |  | 
**title** | **String** |  | 
**thumbnails** | [**Vec<crate::models::Thumbnail>**](Thumbnail.md) |  | 
**description** | Option<**String**> |  | 
**authors** | [**Vec<crate::models::ArtistRun>**](ArtistRun.md) |  | 
**r#type** | **String** | can be `Playlist`, `Chart` or `Radio` | 
**year** | Option<**String**> |  | 
**track_count** | Option<[**serde_json::Value**](.md)> |  | 
**duration** | Option<**String**> |  | 
**duration_seconds** | **f64** |  | 
**tracks** | [**Vec<crate::models::PlaylistItem>**](PlaylistItem.md) |  | 
**continuation** | Option<**String**> |  | 
**suggestions** | [**Vec<crate::models::PlaylistItem>**](PlaylistItem.md) |  | 
**suggestions_continuation** | Option<**String**> |  | 
**related** | [**Vec<crate::models::ParsedPlaylist>**](ParsedPlaylist.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



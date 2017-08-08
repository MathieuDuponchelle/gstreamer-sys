// This file was generated by gir (3294959) from gir-files (???)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gstreamer_sys as gst;
extern crate gstreamer_video_sys as gst_video;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType, Volatile};

// Enums
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GstPlayerColorBalanceType {
    Hue = 3,
    Brightness = 0,
    Saturation = 2,
    Contrast = 1,
}
pub const GST_PLAYER_COLOR_BALANCE_HUE: GstPlayerColorBalanceType = GstPlayerColorBalanceType::Hue;
pub const GST_PLAYER_COLOR_BALANCE_BRIGHTNESS: GstPlayerColorBalanceType = GstPlayerColorBalanceType::Brightness;
pub const GST_PLAYER_COLOR_BALANCE_SATURATION: GstPlayerColorBalanceType = GstPlayerColorBalanceType::Saturation;
pub const GST_PLAYER_COLOR_BALANCE_CONTRAST: GstPlayerColorBalanceType = GstPlayerColorBalanceType::Contrast;

pub type PlayerError = c_int;
pub const GST_PLAYER_ERROR_FAILED: PlayerError = 0;
pub type GstPlayerError = PlayerError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GstPlayerSnapshotFormat {
    RawNative = 0,
    RawXrgb = 1,
    RawBgrx = 2,
    Jpg = 3,
    Png = 4,
}
pub const GST_PLAYER_THUMBNAIL_RAW_NATIVE: GstPlayerSnapshotFormat = GstPlayerSnapshotFormat::RawNative;
pub const GST_PLAYER_THUMBNAIL_RAW_xRGB: GstPlayerSnapshotFormat = GstPlayerSnapshotFormat::RawXrgb;
pub const GST_PLAYER_THUMBNAIL_RAW_BGRx: GstPlayerSnapshotFormat = GstPlayerSnapshotFormat::RawBgrx;
pub const GST_PLAYER_THUMBNAIL_JPG: GstPlayerSnapshotFormat = GstPlayerSnapshotFormat::Jpg;
pub const GST_PLAYER_THUMBNAIL_PNG: GstPlayerSnapshotFormat = GstPlayerSnapshotFormat::Png;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GstPlayerState {
    Stopped = 0,
    Buffering = 1,
    Paused = 2,
    Playing = 3,
}
pub const GST_PLAYER_STATE_STOPPED: GstPlayerState = GstPlayerState::Stopped;
pub const GST_PLAYER_STATE_BUFFERING: GstPlayerState = GstPlayerState::Buffering;
pub const GST_PLAYER_STATE_PAUSED: GstPlayerState = GstPlayerState::Paused;
pub const GST_PLAYER_STATE_PLAYING: GstPlayerState = GstPlayerState::Playing;

// Callbacks
pub type GstPlayerSignalDispatcherFunc = Option<unsafe extern "C" fn(gpointer)>;

// Records
#[repr(C)]
pub struct GstPlayerAudioInfoClass(c_void);

#[repr(C)]
pub struct GstPlayerClass(c_void);

#[repr(C)]
pub struct GstPlayerGMainContextSignalDispatcherClass(c_void);

#[repr(C)]
pub struct GstPlayerMediaInfoClass(c_void);

#[repr(C)]
pub struct GstPlayerSignalDispatcherInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub dispatch: Option<unsafe extern "C" fn(*mut GstPlayerSignalDispatcher, *mut GstPlayer, GstPlayerSignalDispatcherFunc, gpointer, glib::GDestroyNotify)>,
}

#[repr(C)]
pub struct GstPlayerStreamInfoClass(c_void);

#[repr(C)]
pub struct GstPlayerSubtitleInfoClass(c_void);

#[repr(C)]
pub struct GstPlayerVideoInfoClass(c_void);

#[repr(C)]
pub struct GstPlayerVideoOverlayVideoRendererClass(c_void);

#[repr(C)]
pub struct GstPlayerVideoRendererInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub create_video_sink: Option<unsafe extern "C" fn(*mut GstPlayerVideoRenderer, *mut GstPlayer) -> *mut gst::GstElement>,
}

#[repr(C)]
pub struct GstPlayerVisualization {
    pub name: *mut c_char,
    pub description: *mut c_char,
}

// Classes
#[repr(C)]
pub struct GstPlayer(c_void);

#[repr(C)]
pub struct GstPlayerAudioInfo(c_void);

#[repr(C)]
pub struct GstPlayerGMainContextSignalDispatcher(c_void);

#[repr(C)]
pub struct GstPlayerMediaInfo(c_void);

#[repr(C)]
pub struct GstPlayerStreamInfo(c_void);

#[repr(C)]
pub struct GstPlayerSubtitleInfo(c_void);

#[repr(C)]
pub struct GstPlayerVideoInfo(c_void);

#[repr(C)]
pub struct GstPlayerVideoOverlayVideoRenderer(c_void);

// Interfaces
#[repr(C)]
pub struct GstPlayerSignalDispatcher(c_void);
#[repr(C)]
pub struct GstPlayerVideoRenderer(c_void);

extern "C" {

    //=========================================================================
    // GstPlayerColorBalanceType
    //=========================================================================
    pub fn gst_player_color_balance_type_get_type() -> GType;
    pub fn gst_player_color_balance_type_get_name(type_: GstPlayerColorBalanceType) -> *const c_char;

    //=========================================================================
    // GstPlayerError
    //=========================================================================
    pub fn gst_player_error_get_type() -> GType;
    pub fn gst_player_error_get_name(error: GstPlayerError) -> *const c_char;
    pub fn gst_player_error_quark() -> glib::GQuark;

    //=========================================================================
    // GstPlayerState
    //=========================================================================
    pub fn gst_player_state_get_type() -> GType;
    pub fn gst_player_state_get_name(state: GstPlayerState) -> *const c_char;

    //=========================================================================
    // GstPlayerVisualization
    //=========================================================================
    pub fn gst_player_visualization_get_type() -> GType;
    pub fn gst_player_visualization_copy(vis: *const GstPlayerVisualization) -> *mut GstPlayerVisualization;
    pub fn gst_player_visualization_free(vis: *mut GstPlayerVisualization);

    //=========================================================================
    // GstPlayer
    //=========================================================================
    pub fn gst_player_get_type() -> GType;
    pub fn gst_player_new(video_renderer: *mut GstPlayerVideoRenderer, signal_dispatcher: *mut GstPlayerSignalDispatcher) -> *mut GstPlayer;
    pub fn gst_player_config_get_position_update_interval(config: *const gst::GstStructure) -> c_uint;
    pub fn gst_player_config_get_seek_accurate(config: *const gst::GstStructure) -> gboolean;
    pub fn gst_player_config_get_user_agent(config: *const gst::GstStructure) -> *mut c_char;
    pub fn gst_player_config_set_position_update_interval(config: *mut gst::GstStructure, interval: c_uint);
    pub fn gst_player_config_set_user_agent(config: *mut gst::GstStructure, agent: *const c_char);
    pub fn gst_player_get_audio_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_get_subtitle_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_get_video_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_visualizations_free(viss: *mut *mut GstPlayerVisualization);
    pub fn gst_player_visualizations_get() -> *mut *mut GstPlayerVisualization;
    pub fn gst_player_config_set_seek_accurate(player: *mut GstPlayer, accurate: gboolean);
    pub fn gst_player_get_audio_video_offset(player: *mut GstPlayer) -> i64;
    pub fn gst_player_get_color_balance(player: *mut GstPlayer, type_: GstPlayerColorBalanceType) -> c_double;
    pub fn gst_player_get_config(player: *mut GstPlayer) -> *mut gst::GstStructure;
    pub fn gst_player_get_current_audio_track(player: *mut GstPlayer) -> *mut GstPlayerAudioInfo;
    pub fn gst_player_get_current_subtitle_track(player: *mut GstPlayer) -> *mut GstPlayerSubtitleInfo;
    pub fn gst_player_get_current_video_track(player: *mut GstPlayer) -> *mut GstPlayerVideoInfo;
    pub fn gst_player_get_current_visualization(player: *mut GstPlayer) -> *mut c_char;
    pub fn gst_player_get_duration(player: *mut GstPlayer) -> gst::GstClockTime;
    pub fn gst_player_get_media_info(player: *mut GstPlayer) -> *mut GstPlayerMediaInfo;
    pub fn gst_player_get_multiview_flags(player: *mut GstPlayer) -> gst_video::GstVideoMultiviewFlags;
    pub fn gst_player_get_multiview_mode(player: *mut GstPlayer) -> gst_video::GstVideoMultiviewMode;
    pub fn gst_player_get_mute(player: *mut GstPlayer) -> gboolean;
    pub fn gst_player_get_pipeline(player: *mut GstPlayer) -> *mut gst::GstElement;
    pub fn gst_player_get_position(player: *mut GstPlayer) -> gst::GstClockTime;
    pub fn gst_player_get_rate(player: *mut GstPlayer) -> c_double;
    pub fn gst_player_get_subtitle_uri(player: *mut GstPlayer) -> *mut c_char;
    pub fn gst_player_get_uri(player: *mut GstPlayer) -> *mut c_char;
    pub fn gst_player_get_video_snapshot(player: *mut GstPlayer, format: GstPlayerSnapshotFormat, config: *const gst::GstStructure) -> *mut gst::GstSample;
    pub fn gst_player_get_volume(player: *mut GstPlayer) -> c_double;
    pub fn gst_player_has_color_balance(player: *mut GstPlayer) -> gboolean;
    pub fn gst_player_pause(player: *mut GstPlayer);
    pub fn gst_player_play(player: *mut GstPlayer);
    pub fn gst_player_seek(player: *mut GstPlayer, position: gst::GstClockTime);
    pub fn gst_player_set_audio_track(player: *mut GstPlayer, stream_index: c_int) -> gboolean;
    pub fn gst_player_set_audio_track_enabled(player: *mut GstPlayer, enabled: gboolean);
    pub fn gst_player_set_audio_video_offset(player: *mut GstPlayer, offset: i64);
    pub fn gst_player_set_color_balance(player: *mut GstPlayer, type_: GstPlayerColorBalanceType, value: c_double);
    pub fn gst_player_set_config(player: *mut GstPlayer, config: *mut gst::GstStructure) -> gboolean;
    pub fn gst_player_set_multiview_flags(player: *mut GstPlayer, flags: gst_video::GstVideoMultiviewFlags);
    pub fn gst_player_set_multiview_mode(player: *mut GstPlayer, mode: gst_video::GstVideoMultiviewMode);
    pub fn gst_player_set_mute(player: *mut GstPlayer, val: gboolean);
    pub fn gst_player_set_rate(player: *mut GstPlayer, rate: c_double);
    pub fn gst_player_set_subtitle_track(player: *mut GstPlayer, stream_index: c_int) -> gboolean;
    pub fn gst_player_set_subtitle_track_enabled(player: *mut GstPlayer, enabled: gboolean);
    pub fn gst_player_set_subtitle_uri(player: *mut GstPlayer, uri: *const c_char);
    pub fn gst_player_set_uri(player: *mut GstPlayer, uri: *const c_char);
    pub fn gst_player_set_video_track(player: *mut GstPlayer, stream_index: c_int) -> gboolean;
    pub fn gst_player_set_video_track_enabled(player: *mut GstPlayer, enabled: gboolean);
    pub fn gst_player_set_visualization(player: *mut GstPlayer, name: *const c_char) -> gboolean;
    pub fn gst_player_set_visualization_enabled(player: *mut GstPlayer, enabled: gboolean);
    pub fn gst_player_set_volume(player: *mut GstPlayer, val: c_double);
    pub fn gst_player_stop(player: *mut GstPlayer);

    //=========================================================================
    // GstPlayerAudioInfo
    //=========================================================================
    pub fn gst_player_audio_info_get_type() -> GType;
    pub fn gst_player_audio_info_get_bitrate(info: *const GstPlayerAudioInfo) -> c_int;
    pub fn gst_player_audio_info_get_channels(info: *const GstPlayerAudioInfo) -> c_int;
    pub fn gst_player_audio_info_get_language(info: *const GstPlayerAudioInfo) -> *const c_char;
    pub fn gst_player_audio_info_get_max_bitrate(info: *const GstPlayerAudioInfo) -> c_int;
    pub fn gst_player_audio_info_get_sample_rate(info: *const GstPlayerAudioInfo) -> c_int;

    //=========================================================================
    // GstPlayerGMainContextSignalDispatcher
    //=========================================================================
    pub fn gst_player_g_main_context_signal_dispatcher_get_type() -> GType;
    pub fn gst_player_g_main_context_signal_dispatcher_new(application_context: *mut glib::GMainContext) -> *mut GstPlayerSignalDispatcher;

    //=========================================================================
    // GstPlayerMediaInfo
    //=========================================================================
    pub fn gst_player_media_info_get_type() -> GType;
    pub fn gst_player_media_info_get_audio_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_media_info_get_container_format(info: *const GstPlayerMediaInfo) -> *const c_char;
    pub fn gst_player_media_info_get_duration(info: *const GstPlayerMediaInfo) -> gst::GstClockTime;
    pub fn gst_player_media_info_get_image_sample(info: *const GstPlayerMediaInfo) -> *mut gst::GstSample;
    pub fn gst_player_media_info_get_number_of_audio_streams(info: *const GstPlayerMediaInfo) -> c_uint;
    pub fn gst_player_media_info_get_number_of_streams(info: *const GstPlayerMediaInfo) -> c_uint;
    pub fn gst_player_media_info_get_number_of_subtitle_streams(info: *const GstPlayerMediaInfo) -> c_uint;
    pub fn gst_player_media_info_get_number_of_video_streams(info: *const GstPlayerMediaInfo) -> c_uint;
    pub fn gst_player_media_info_get_stream_list(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_media_info_get_subtitle_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_media_info_get_tags(info: *const GstPlayerMediaInfo) -> *mut gst::GstTagList;
    pub fn gst_player_media_info_get_title(info: *const GstPlayerMediaInfo) -> *const c_char;
    pub fn gst_player_media_info_get_uri(info: *const GstPlayerMediaInfo) -> *const c_char;
    pub fn gst_player_media_info_get_video_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_media_info_is_live(info: *const GstPlayerMediaInfo) -> gboolean;
    pub fn gst_player_media_info_is_seekable(info: *const GstPlayerMediaInfo) -> gboolean;

    //=========================================================================
    // GstPlayerStreamInfo
    //=========================================================================
    pub fn gst_player_stream_info_get_type() -> GType;
    pub fn gst_player_stream_info_get_caps(info: *const GstPlayerStreamInfo) -> *mut gst::GstCaps;
    pub fn gst_player_stream_info_get_codec(info: *const GstPlayerStreamInfo) -> *const c_char;
    pub fn gst_player_stream_info_get_index(info: *const GstPlayerStreamInfo) -> c_int;
    pub fn gst_player_stream_info_get_stream_type(info: *const GstPlayerStreamInfo) -> *const c_char;
    pub fn gst_player_stream_info_get_tags(info: *const GstPlayerStreamInfo) -> *mut gst::GstTagList;

    //=========================================================================
    // GstPlayerSubtitleInfo
    //=========================================================================
    pub fn gst_player_subtitle_info_get_type() -> GType;
    pub fn gst_player_subtitle_info_get_language(info: *const GstPlayerSubtitleInfo) -> *const c_char;

    //=========================================================================
    // GstPlayerVideoInfo
    //=========================================================================
    pub fn gst_player_video_info_get_type() -> GType;
    pub fn gst_player_video_info_get_bitrate(info: *const GstPlayerVideoInfo) -> c_int;
    pub fn gst_player_video_info_get_framerate(info: *const GstPlayerVideoInfo, fps_n: *mut c_int, fps_d: *mut c_int);
    pub fn gst_player_video_info_get_height(info: *const GstPlayerVideoInfo) -> c_int;
    pub fn gst_player_video_info_get_max_bitrate(info: *const GstPlayerVideoInfo) -> c_int;
    pub fn gst_player_video_info_get_pixel_aspect_ratio(info: *const GstPlayerVideoInfo, par_n: *mut c_uint, par_d: *mut c_uint);
    pub fn gst_player_video_info_get_width(info: *const GstPlayerVideoInfo) -> c_int;

    //=========================================================================
    // GstPlayerVideoOverlayVideoRenderer
    //=========================================================================
    pub fn gst_player_video_overlay_video_renderer_get_type() -> GType;
    pub fn gst_player_video_overlay_video_renderer_new(window_handle: gpointer) -> *mut GstPlayerVideoRenderer;
    pub fn gst_player_video_overlay_video_renderer_new_with_sink(window_handle: gpointer, video_sink: *mut gst::GstElement) -> *mut GstPlayerVideoRenderer;
    pub fn gst_player_video_overlay_video_renderer_expose(self_: *mut GstPlayerVideoOverlayVideoRenderer);
    pub fn gst_player_video_overlay_video_renderer_get_render_rectangle(self_: *mut GstPlayerVideoOverlayVideoRenderer, x: *mut c_int, y: *mut c_int, width: *mut c_int, height: *mut c_int);
    pub fn gst_player_video_overlay_video_renderer_get_window_handle(self_: *mut GstPlayerVideoOverlayVideoRenderer) -> gpointer;
    pub fn gst_player_video_overlay_video_renderer_set_render_rectangle(self_: *mut GstPlayerVideoOverlayVideoRenderer, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn gst_player_video_overlay_video_renderer_set_window_handle(self_: *mut GstPlayerVideoOverlayVideoRenderer, window_handle: gpointer);

    //=========================================================================
    // GstPlayerSignalDispatcher
    //=========================================================================
    pub fn gst_player_signal_dispatcher_get_type() -> GType;

    //=========================================================================
    // GstPlayerVideoRenderer
    //=========================================================================
    pub fn gst_player_video_renderer_get_type() -> GType;

}

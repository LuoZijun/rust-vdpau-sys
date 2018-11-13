pub const VDP_TRUE: ::std::os::raw::c_uint = 1;
pub const VDP_FALSE: ::std::os::raw::c_uint = 0;
pub const VDP_INVALID_HANDLE: ::std::os::raw::c_uint = 4294967295;
pub const VDPAU_INTERFACE_VERSION: ::std::os::raw::c_uint = 1;
pub const VDPAU_VERSION: ::std::os::raw::c_uint = 1;
pub const VDP_PROCAMP_VERSION: ::std::os::raw::c_uint = 0;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_STATE_VERSION: ::std::os::raw::c_uint = 0;
pub const VDP_OUTPUT_SURFACE_RENDER_ROTATE_0: ::std::os::raw::c_uint = 0;
pub const VDP_OUTPUT_SURFACE_RENDER_ROTATE_90: ::std::os::raw::c_uint = 1;
pub const VDP_OUTPUT_SURFACE_RENDER_ROTATE_180: ::std::os::raw::c_uint = 2;
pub const VDP_OUTPUT_SURFACE_RENDER_ROTATE_270: ::std::os::raw::c_uint = 3;
pub const VDP_OUTPUT_SURFACE_RENDER_COLOR_PER_VERTEX: ::std::os::raw::c_uint = 4;
pub const VDP_DECODER_LEVEL_MPEG1_NA: ::std::os::raw::c_uint = 0;
pub const VDP_DECODER_LEVEL_MPEG2_LL: ::std::os::raw::c_uint = 0;
pub const VDP_DECODER_LEVEL_MPEG2_ML: ::std::os::raw::c_uint = 1;
pub const VDP_DECODER_LEVEL_MPEG2_HL14: ::std::os::raw::c_uint = 2;
pub const VDP_DECODER_LEVEL_MPEG2_HL: ::std::os::raw::c_uint = 3;
pub const VDP_DECODER_LEVEL_H264_1: ::std::os::raw::c_uint = 10;
pub const VDP_DECODER_LEVEL_H264_1b: ::std::os::raw::c_uint = 9;
pub const VDP_DECODER_LEVEL_H264_1_1: ::std::os::raw::c_uint = 11;
pub const VDP_DECODER_LEVEL_H264_1_2: ::std::os::raw::c_uint = 12;
pub const VDP_DECODER_LEVEL_H264_1_3: ::std::os::raw::c_uint = 13;
pub const VDP_DECODER_LEVEL_H264_2: ::std::os::raw::c_uint = 20;
pub const VDP_DECODER_LEVEL_H264_2_1: ::std::os::raw::c_uint = 21;
pub const VDP_DECODER_LEVEL_H264_2_2: ::std::os::raw::c_uint = 22;
pub const VDP_DECODER_LEVEL_H264_3: ::std::os::raw::c_uint = 30;
pub const VDP_DECODER_LEVEL_H264_3_1: ::std::os::raw::c_uint = 31;
pub const VDP_DECODER_LEVEL_H264_3_2: ::std::os::raw::c_uint = 32;
pub const VDP_DECODER_LEVEL_H264_4: ::std::os::raw::c_uint = 40;
pub const VDP_DECODER_LEVEL_H264_4_1: ::std::os::raw::c_uint = 41;
pub const VDP_DECODER_LEVEL_H264_4_2: ::std::os::raw::c_uint = 42;
pub const VDP_DECODER_LEVEL_H264_5: ::std::os::raw::c_uint = 50;
pub const VDP_DECODER_LEVEL_H264_5_1: ::std::os::raw::c_uint = 51;
pub const VDP_DECODER_LEVEL_VC1_SIMPLE_LOW: ::std::os::raw::c_uint = 0;
pub const VDP_DECODER_LEVEL_VC1_SIMPLE_MEDIUM: ::std::os::raw::c_uint = 1;
pub const VDP_DECODER_LEVEL_VC1_MAIN_LOW: ::std::os::raw::c_uint = 0;
pub const VDP_DECODER_LEVEL_VC1_MAIN_MEDIUM: ::std::os::raw::c_uint = 1;
pub const VDP_DECODER_LEVEL_VC1_MAIN_HIGH: ::std::os::raw::c_uint = 2;
pub const VDP_DECODER_LEVEL_VC1_ADVANCED_L0: ::std::os::raw::c_uint = 0;
pub const VDP_DECODER_LEVEL_VC1_ADVANCED_L1: ::std::os::raw::c_uint = 1;
pub const VDP_DECODER_LEVEL_VC1_ADVANCED_L2: ::std::os::raw::c_uint = 2;
pub const VDP_DECODER_LEVEL_VC1_ADVANCED_L3: ::std::os::raw::c_uint = 3;
pub const VDP_DECODER_LEVEL_VC1_ADVANCED_L4: ::std::os::raw::c_uint = 4;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_SP_L0: ::std::os::raw::c_uint = 0;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_SP_L1: ::std::os::raw::c_uint = 1;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_SP_L2: ::std::os::raw::c_uint = 2;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_SP_L3: ::std::os::raw::c_uint = 3;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_ASP_L0: ::std::os::raw::c_uint = 0;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_ASP_L1: ::std::os::raw::c_uint = 1;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_ASP_L2: ::std::os::raw::c_uint = 2;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_ASP_L3: ::std::os::raw::c_uint = 3;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_ASP_L4: ::std::os::raw::c_uint = 4;
pub const VDP_DECODER_LEVEL_MPEG4_PART2_ASP_L5: ::std::os::raw::c_uint = 5;
pub const VDP_DECODER_LEVEL_DIVX_NA: ::std::os::raw::c_uint = 0;
pub const VDP_BITSTREAM_BUFFER_VERSION: ::std::os::raw::c_uint = 0;
pub const VDP_LAYER_VERSION: ::std::os::raw::c_uint = 0;
pub const VDP_FUNC_ID_BASE_WINSYS: ::std::os::raw::c_uint = 4096;


/// \brief A boolean value, holding \ref VDP_TRUE or \ref
/// VDP_FALSE.
pub type VdpBool = ::std::os::raw::c_int;
/// \brief The set of all chroma formats for \ref VdpVideoSurface
/// "VdpVideoSurface"s.
pub type VdpChromaType = u32;
/// \brief The set of all known YCbCr surface formats.
pub type VdpYCbCrFormat = u32;
/// \brief  The set of all known RGB surface formats.
pub type VdpRGBAFormat = u32;
/// \brief  The set of all known indexed surface formats.
pub type VdpIndexedFormat = u32;
/// \brief A location within a surface.
///
/// The VDPAU co-ordinate system has its origin at the top-left
/// of a surface, with x and y components increasing right and
/// down.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpPoint {
    /// X co-ordinate.
    pub x: u32,
    /// Y co-ordinate.
    pub y: u32,
}
/// \brief A rectangular region of a surface.
///
/// The co-ordinates are top-left inclusive, bottom-right
/// exclusive.
///
/// The VDPAU co-ordinate system has its origin at the top-left
/// of a surface, with x and y components increasing right and
/// down.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpRect {
    /// Left X co-ordinate. Inclusive.
    pub x0: u32,
    /// Top Y co-ordinate. Inclusive.
    pub y0: u32,
    /// Right X co-ordinate. Exclusive.
    pub x1: u32,
    /// Bottom Y co-ordinate. Exclusive.
    pub y1: u32,
}
/// A constant RGBA color.
///
/// Note that the components are stored as float values in the
/// range 0.0...1.0 rather than format-specific integer values.
/// This allows VdpColor values to be independent from the exact
/// surface format(s) in use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
pub const VDP_STATUS_OK: VdpStatus = 0;
pub const VDP_STATUS_NO_IMPLEMENTATION: VdpStatus = 1;
pub const VDP_STATUS_DISPLAY_PREEMPTED: VdpStatus = 2;
pub const VDP_STATUS_INVALID_HANDLE: VdpStatus = 3;
pub const VDP_STATUS_INVALID_POINTER: VdpStatus = 4;
pub const VDP_STATUS_INVALID_CHROMA_TYPE: VdpStatus = 5;
pub const VDP_STATUS_INVALID_Y_CB_CR_FORMAT: VdpStatus = 6;
pub const VDP_STATUS_INVALID_RGBA_FORMAT: VdpStatus = 7;
pub const VDP_STATUS_INVALID_INDEXED_FORMAT: VdpStatus = 8;
pub const VDP_STATUS_INVALID_COLOR_STANDARD: VdpStatus = 9;
pub const VDP_STATUS_INVALID_COLOR_TABLE_FORMAT: VdpStatus = 10;
pub const VDP_STATUS_INVALID_BLEND_FACTOR: VdpStatus = 11;
pub const VDP_STATUS_INVALID_BLEND_EQUATION: VdpStatus = 12;
pub const VDP_STATUS_INVALID_FLAG: VdpStatus = 13;
pub const VDP_STATUS_INVALID_DECODER_PROFILE: VdpStatus = 14;
pub const VDP_STATUS_INVALID_VIDEO_MIXER_FEATURE: VdpStatus = 15;
pub const VDP_STATUS_INVALID_VIDEO_MIXER_PARAMETER: VdpStatus = 16;
pub const VDP_STATUS_INVALID_VIDEO_MIXER_ATTRIBUTE: VdpStatus = 17;
pub const VDP_STATUS_INVALID_VIDEO_MIXER_PICTURE_STRUCTURE: VdpStatus = 18;
pub const VDP_STATUS_INVALID_FUNC_ID: VdpStatus = 19;
pub const VDP_STATUS_INVALID_SIZE: VdpStatus = 20;
pub const VDP_STATUS_INVALID_VALUE: VdpStatus = 21;
pub const VDP_STATUS_INVALID_STRUCT_VERSION: VdpStatus = 22;
pub const VDP_STATUS_RESOURCES: VdpStatus = 23;
pub const VDP_STATUS_HANDLE_DEVICE_MISMATCH: VdpStatus = 24;
pub const VDP_STATUS_ERROR: VdpStatus = 25;
pub type VdpStatus = ::std::os::raw::c_uint;
/// \brief Retrieve a string describing an error code.
/// \param[in] status The error code.
/// \return A pointer to the string. Note that this is a
/// statically allocated read-only string. As such, the
/// application must not free the returned pointer. The
/// pointer is valid as long as the VDPAU implementation is
/// present within the application's address space.
pub type VdpGetErrorString =
    ::std::option::Option<unsafe extern "C" fn(status: VdpStatus) -> *const ::std::os::raw::c_char>;
/// \brief Retrieve the VDPAU version implemented by the backend.
/// \param[out] api_version The API version.
/// \return VdpStatus The completion status of the operation.
pub type VdpGetApiVersion =
    ::std::option::Option<unsafe extern "C" fn(api_version: *mut u32) -> VdpStatus>;
/// \brief Retrieve an implementation-specific string description
/// of the implementation. This typically includes detailed version
/// information.
/// \param[out] information_string A pointer to the information
/// string. Note that this is a statically allocated
/// read-only string. As such, the application must not
/// free the returned pointer. The pointer is valid as long
/// as the implementation is present within the
/// application's address space.
/// \return VdpStatus The completion status of the operation.
///
/// Note that the returned string is useful for information
/// reporting. It is not intended that the application should
/// parse this string in order to determine any information about
/// the implementation.
pub type VdpGetInformationString = ::std::option::Option<
    unsafe extern "C" fn(information_string: *mut *const ::std::os::raw::c_char) -> VdpStatus,
>;
/// \brief  An opaque handle representing a VdpDevice object.
pub type VdpDevice = u32;
/// \brief Destroy a VdpDevice.
/// \param[in] device The device to destroy.
/// \return VdpStatus The completion status of the operation.
pub type VdpDeviceDestroy =
    ::std::option::Option<unsafe extern "C" fn(device: VdpDevice) -> VdpStatus>;
/// \brief Storage for a color space conversion matrix.
///
/// Note that the application may choose to construct the matrix
/// content by either:
/// - Directly filling in the fields of the CSC matrix
/// - Using the \ref VdpGenerateCSCMatrix helper function.
///
/// The color space conversion equation is as follows:
///
/// \f[
/// \left( \begin{array}{c} R \\ G \\ B \end{array} \right)
/// =
/// \left( \begin{array}{cccc}
/// m_{0,0} & m_{0,1} & m_{0,2} & m_{0,3} \\
/// m_{1,0} & m_{1,1} & m_{1,2} & m_{1,3} \\
/// m_{2,0} & m_{2,1} & m_{2,2} & m_{2,3}
/// \end{array}
/// \right)
/// *
/// \left( \begin{array}{c} Y \\ Cb \\ Cr \\ 1.0 \end{array}
/// \right)
/// \f]
pub type VdpCSCMatrix = [[f32; 4usize]; 3usize];
/// \brief Procamp operation parameterization data.
///
/// When performing a color space conversion operation, various
/// adjustments can be performed at the same time, such as
/// brightness and contrast. This structure defines the level of
/// adjustments to make.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpProcamp {
    /// This field must be filled with VDP_PROCAMP_VERSION
    pub struct_version: u32,
    /// Brightness adjustment amount. A value clamped between
    /// -1.0 and 1.0. 0.0 represents no modification.
    pub brightness: f32,
    /// Contrast adjustment amount. A value clamped between
    /// 0.0 and 10.0. 1.0 represents no modification.
    pub contrast: f32,
    /// Saturation adjustment amount. A value clamped between 0.0 and
    /// 10.0. 1.0 represents no modification.
    pub saturation: f32,
    /// Hue adjustment amount. A value clamped between
    /// -PI and PI. 0.0 represents no modification.
    pub hue: f32,
}
/// \brief YCbCr color space specification.
///
/// A number of YCbCr color spaces exist. This enumeration
/// defines the specifications known to VDPAU.
pub type VdpColorStandard = u32;
/// \brief Generate a color space conversion matrix
/// \param[in] procamp The procamp adjustments to make. If NULL,
/// no adjustments will be made.
/// \param[in] standard The YCbCr color space to convert from.
/// \param[out] csc_matrix The CSC matrix to initialize.
/// \return VdpStatus The completion status of the operation.
pub type VdpGenerateCSCMatrix = ::std::option::Option<
    unsafe extern "C" fn(
        procamp: *mut VdpProcamp,
        standard: VdpColorStandard,
        csc_matrix: *mut VdpCSCMatrix,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's VdpVideoSurface
/// capabilities.
/// \param[in] device The device to query.
/// \param[in] surface_chroma_type The type of chroma type for
/// which information is requested.
/// \param[out] is_supported Is this chroma type supported?
/// \param[out] max_width The maximum supported surface width for
/// this chroma type.
/// \param[out] max_height The maximum supported surface height
/// for this chroma type.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoSurfaceQueryCapabilities = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        surface_chroma_type: VdpChromaType,
        is_supported: *mut VdpBool,
        max_width: *mut u32,
        max_height: *mut u32,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's VdpVideoSurface
/// GetBits/PutBits capabilities.
/// \param[in] device The device to query.
/// \param[in] surface_chroma_type The type of chroma type for
/// which information is requested.
/// \param[in] bits_ycbcr_format The format of application "bits"
/// buffer for which information is requested.
/// \param[out] is_supported Is this chroma type supported?
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoSurfaceQueryGetPutBitsYCbCrCapabilities = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        surface_chroma_type: VdpChromaType,
        bits_ycbcr_format: VdpYCbCrFormat,
        is_supported: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief An opaque handle representing a VdpVideoSurface
/// object.
pub type VdpVideoSurface = u32;
/// \brief Create a VdpVideoSurface.
/// \param[in] device The device that will contain the surface.
/// \param[in] chroma_type The chroma type of the new surface.
/// \param[in] width The width of the new surface.
/// \param[in] height The height of the new surface.
/// \param[out] surface The new surface's handle.
/// \return VdpStatus The completion status of the operation.
///
/// The memory backing the surface may not be initialized during
/// creation. Applications are expected to initialize any region
/// that they use, via \ref VdpDecoderRender or \ref
/// VdpVideoSurfacePutBitsYCbCr.
///
/// Note that certain widths/heights are impossible for specific values of
/// chroma_type. For example, the definition of VDP_CHROMA_TYPE_420 implies
/// that the width must be even, since each single chroma sample covers two
/// luma samples horizontally. A similar argument applies to surface heights,
/// although doubly so, since interlaced pictures must be supported; each
/// field's height must itself be a multiple of 2. Hence the overall surface's
/// height must be a multiple of 4.
///
/// Similar rules apply to other chroma_type values.
///
/// Implementations may also impose additional restrictions on the surface
/// sizes they support, potentially requiring additional rounding of actual
/// surface sizes.
///
/// In most cases, this is not an issue, since:
/// - Video streams are encoded as an array of macro-blocks, which typically
/// have larger size alignment requirements than video surfaces do.
/// - APIs such as \ref VdpVideoMixerRender allow specification of a sub-region
/// of the surface to read, which allows the padding data to be clipped away.
///
/// However, other APIs such as \ref VdpVideoSurfaceGetBitsYCbCr and
/// \ref VdpVideoSurfacePutBitsYCbCr do not allow a sub-region to be specified,
/// and always operate on surface size that was actually allocated, rather
/// than the surface size that was requested. In this case, applications need
/// to be aware of the actual surface size, in order to allocate appropriately
/// sized buffers for the get-/put-bits operations.
///
/// For this reason, applications may need to call
/// \ref VdpVideoSurfaceGetParameters after creation, in order to retrieve the
/// actual surface size.
pub type VdpVideoSurfaceCreate = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        chroma_type: VdpChromaType,
        width: u32,
        height: u32,
        surface: *mut VdpVideoSurface,
    ) -> VdpStatus,
>;
/// \brief Destroy a VdpVideoSurface.
/// \param[in] surface The surface's handle.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoSurfaceDestroy =
    ::std::option::Option<unsafe extern "C" fn(surface: VdpVideoSurface) -> VdpStatus>;
/// \brief Retrieve the parameters used to create a
/// VdpVideoSurface.
/// \param[in] surface The surface's handle.
/// \param[out] chroma_type The chroma type of the surface.
/// \param[out] width The width of the surface.
/// \param[out] height The height of the surface.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoSurfaceGetParameters = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpVideoSurface,
        chroma_type: *mut VdpChromaType,
        width: *mut u32,
        height: *mut u32,
    ) -> VdpStatus,
>;
/// \brief Copy image data from a VdpVideoSurface to application
/// memory in a specified YCbCr format.
/// \param[in] surface The surface's handle.
/// \param[in] destination_ycbcr_format The format of the
/// application's data buffers.
/// \param[in] destination_data Pointers to the application data
/// buffers into which the image data will be written. Note
/// that this is an array of pointers, one per plane. The
/// destination_format parameter will define how many
/// planes are required.
/// \param[in] destination_pitches Pointers to the pitch values
/// for the application data buffers. Note that this is an
/// array of pointers, one per plane. The
/// destination_format parameter will define how many
/// planes are required.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoSurfaceGetBitsYCbCr = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpVideoSurface,
        destination_ycbcr_format: VdpYCbCrFormat,
        destination_data: *const *const ::std::os::raw::c_void,
        destination_pitches: *const u32,
    ) -> VdpStatus,
>;
/// \brief Copy image data from application memory in a specific
/// YCbCr format to a VdpVideoSurface.
/// \param[in] surface The surface's handle.
/// \param[in] source_ycbcr_format The format of the
/// application's data buffers.
/// \param[in] source_data Pointers to the application data
/// buffers from which the image data will be copied. Note
/// that this is an array of pointers, one per plane. The
/// source_format parameter will define how many
/// planes are required.
/// \param[in] source_pitches Pointers to the pitch values
/// for the application data buffers. Note that this is an
/// array of pointers, one per plane. The
/// source_format parameter will define how many
/// planes are required.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoSurfacePutBitsYCbCr = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpVideoSurface,
        source_ycbcr_format: VdpYCbCrFormat,
        source_data: *const *const ::std::os::raw::c_void,
        source_pitches: *const u32,
    ) -> VdpStatus,
>;
/// \brief The set of all known color table formats, for use with
/// \ref VdpOutputSurfacePutBitsIndexed.
pub type VdpColorTableFormat = u32;
/// \brief Query the implementation's VdpOutputSurface
/// capabilities.
/// \param[in] device The device to query.
/// \param[in] surface_rgba_format The surface format for
/// which information is requested.
/// \param[out] is_supported Is this surface format supported?
/// \param[out] max_width The maximum supported surface width for
/// this chroma type.
/// \param[out] max_height The maximum supported surface height
/// for this chroma type.
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfaceQueryCapabilities = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        surface_rgba_format: VdpRGBAFormat,
        is_supported: *mut VdpBool,
        max_width: *mut u32,
        max_height: *mut u32,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's capability to perform a
/// PutBits operation using application data matching the
/// surface's format.
/// \param[in] device The device to query.
/// \param[in] surface_rgba_format The surface format for
/// which information is requested.
/// \param[out] is_supported Is this surface format supported?
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfaceQueryGetPutBitsNativeCapabilities = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        surface_rgba_format: VdpRGBAFormat,
        is_supported: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's capability to perform a
/// PutBits operation using application data in a specific
/// indexed format.
/// \param[in] device The device to query.
/// \param[in] surface_rgba_format The surface format for
/// which information is requested.
/// \param[in] bits_indexed_format The format of the application
/// data buffer.
/// \param[in] color_table_format The format of the color lookup
/// table.
/// \param[out] is_supported Is this surface format supported?
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfaceQueryPutBitsIndexedCapabilities = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        surface_rgba_format: VdpRGBAFormat,
        bits_indexed_format: VdpIndexedFormat,
        color_table_format: VdpColorTableFormat,
        is_supported: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's capability to perform a
/// PutBits operation using application data in a specific
/// YCbCr/YUB format.
/// \param[in] device The device to query.
/// \param[in] surface_rgba_format The surface format for which
/// information is requested.
/// \param[in] bits_ycbcr_format The format of the application
/// data buffer.
/// \param[out] is_supported Is this surface format supported?
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfaceQueryPutBitsYCbCrCapabilities = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        surface_rgba_format: VdpRGBAFormat,
        bits_ycbcr_format: VdpYCbCrFormat,
        is_supported: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief An opaque handle representing a VdpOutputSurface
/// object.
pub type VdpOutputSurface = u32;
/// \brief Create a VdpOutputSurface.
/// \param[in] device The device that will contain the surface.
/// \param[in] rgba_format The format of the new surface.
/// \param[in] width The width of the new surface.
/// \param[in] height The height of the new surface.
/// \param[out] surface The new surface's handle.
/// \return VdpStatus The completion status of the operation.
///
/// The memory backing the surface will be initialized to 0 color
/// and 0 alpha (i.e. black.)
pub type VdpOutputSurfaceCreate = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        rgba_format: VdpRGBAFormat,
        width: u32,
        height: u32,
        surface: *mut VdpOutputSurface,
    ) -> VdpStatus,
>;
/// \brief Destroy a VdpOutputSurface.
/// \param[in] surface The surface's handle.
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfaceDestroy =
    ::std::option::Option<unsafe extern "C" fn(surface: VdpOutputSurface) -> VdpStatus>;
/// \brief Retrieve the parameters used to create a
/// VdpOutputSurface.
/// \param[in] surface The surface's handle.
/// \param[out] rgba_format The format of the surface.
/// \param[out] width The width of the surface.
/// \param[out] height The height of the surface.
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfaceGetParameters = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpOutputSurface,
        rgba_format: *mut VdpRGBAFormat,
        width: *mut u32,
        height: *mut u32,
    ) -> VdpStatus,
>;
/// \brief Copy image data from a VdpOutputSurface to application
/// memory in the surface's native format.
/// \param[in] surface The surface's handle.
/// \param[in] source_rect The sub-rectangle of the source
/// surface to copy. If NULL, the entire surface will be
/// retrieved.
/// \param[in] destination_data Pointers to the application data
/// buffers into which the image data will be written. Note
/// that this is an array of pointers, one per plane. The
/// destination_format parameter will define how many
/// planes are required.
/// \param[in] destination_pitches Pointers to the pitch values
/// for the application data buffers. Note that this is an
/// array of pointers, one per plane. The
/// destination_format parameter will define how many
/// planes are required.
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfaceGetBitsNative = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpOutputSurface,
        source_rect: *const VdpRect,
        destination_data: *const *const ::std::os::raw::c_void,
        destination_pitches: *const u32,
    ) -> VdpStatus,
>;
/// \brief Copy image data from application memory in the
/// surface's native format to a VdpOutputSurface.
/// \param[in] surface The surface's handle.
/// \param[in] source_data Pointers to the application data
/// buffers from which the image data will be copied. Note
/// that this is an array of pointers, one per plane. The
/// source_format parameter will define how many
/// planes are required.
/// \param[in] source_pitches Pointers to the pitch values
/// for the application data buffers. Note that this is an
/// array of pointers, one per plane. The
/// source_format parameter will define how many
/// planes are required.
/// \param[in] destination_rect The sub-rectangle of the surface
/// to fill with application data. If NULL, the entire
/// surface will be updated.
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfacePutBitsNative = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpOutputSurface,
        source_data: *const *const ::std::os::raw::c_void,
        source_pitches: *const u32,
        destination_rect: *const VdpRect,
    ) -> VdpStatus,
>;
/// \brief Copy image data from application memory in a specific
/// indexed format to a VdpOutputSurface.
/// \param[in] surface The surface's handle.
/// \param[in] source_indexed_format The format of the
/// application's data buffers.
/// \param[in] source_data Pointers to the application data
/// buffers from which the image data will be copied. Note
/// that this is an array of pointers, one per plane. The
/// source_indexed_format parameter will define how many
/// planes are required.
/// \param[in] source_pitches Pointers to the pitch values
/// for the application data buffers. Note that this is an
/// array of pointers, one per plane. The
/// source_indexed_format parameter will define how many
/// planes are required.
/// \param[in] destination_rect The sub-rectangle of the surface
/// to fill with application data. If NULL, the entire
/// surface will be updated.
/// \param[in] color_table_format The format of the color_table.
/// \param[in] color_table A table that maps between source index
/// and target color data. See \ref VdpColorTableFormat for
/// details regarding the memory layout.
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfacePutBitsIndexed = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpOutputSurface,
        source_indexed_format: VdpIndexedFormat,
        source_data: *const *const ::std::os::raw::c_void,
        source_pitch: *const u32,
        destination_rect: *const VdpRect,
        color_table_format: VdpColorTableFormat,
        color_table: *const ::std::os::raw::c_void,
    ) -> VdpStatus,
>;
/// \brief Copy image data from application memory in a specific
/// YCbCr format to a VdpOutputSurface.
/// \param[in] surface The surface's handle.
/// \param[in] source_ycbcr_format The format of the
/// application's data buffers.
/// \param[in] source_data Pointers to the application data
/// buffers from which the image data will be copied. Note
/// that this is an array of pointers, one per plane. The
/// source_ycbcr_format parameter will define how many
/// planes are required.
/// \param[in] source_pitches Pointers to the pitch values
/// for the application data buffers. Note that this is an
/// array of pointers, one per plane. The
/// source_ycbcr_format parameter will define how many
/// planes are required.
/// \param[in] destination_rect The sub-rectangle of the surface
/// to fill with application data. If NULL, the entire
/// surface will be updated.
/// \param[in] csc_matrix The color space conversion matrix used
/// by the copy operation. If NULL, a default matrix will
/// be used internally. Th default matrix is equivalent to
/// ITU-R BT.601 with no procamp changes.
/// \return VdpStatus The completion status of the operation.
pub type VdpOutputSurfacePutBitsYCbCr = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpOutputSurface,
        source_ycbcr_format: VdpYCbCrFormat,
        source_data: *const *const ::std::os::raw::c_void,
        source_pitches: *const u32,
        destination_rect: *const VdpRect,
        csc_matrix: *const VdpCSCMatrix,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's VdpBitmapSurface
/// capabilities.
/// \param[in] device The device to query.
/// \param[in] surface_rgba_format The surface format for
/// which information is requested.
/// \param[out] is_supported Is this surface format supported?
/// \param[out] max_width The maximum supported surface width for
/// this chroma type.
/// \param[out] max_height The maximum supported surface height
/// for this chroma type.
/// \return VdpStatus The completion status of the operation.
pub type VdpBitmapSurfaceQueryCapabilities = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        surface_rgba_format: VdpRGBAFormat,
        is_supported: *mut VdpBool,
        max_width: *mut u32,
        max_height: *mut u32,
    ) -> VdpStatus,
>;
/// \brief An opaque handle representing a VdpBitmapSurface
/// object.
pub type VdpBitmapSurface = u32;
/// \brief Create a VdpBitmapSurface.
/// \param[in] device The device that will contain the surface.
/// \param[in] rgba_format The format of the new surface.
/// \param[in] width The width of the new surface.
/// \param[in] height The height of the new surface.
/// \param[in] frequently_accessed Is this bitmap used
/// frequently, or infrequently, by compositing options?
/// Implementations may use this as a hint to determine how
/// to allocate the underlying storage for the surface.
/// \param[out] surface The new surface's handle.
/// \return VdpStatus The completion status of the operation.
///
/// The memory backing the surface may not be initialized
/// during creation. Applications are expected initialize any
/// region that they use, via \ref VdpBitmapSurfacePutBitsNative.
pub type VdpBitmapSurfaceCreate = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        rgba_format: VdpRGBAFormat,
        width: u32,
        height: u32,
        frequently_accessed: VdpBool,
        surface: *mut VdpBitmapSurface,
    ) -> VdpStatus,
>;
/// \brief Destroy a VdpBitmapSurface.
/// \param[in] surface The surface's handle.
/// \return VdpStatus The completion status of the operation.
pub type VdpBitmapSurfaceDestroy =
    ::std::option::Option<unsafe extern "C" fn(surface: VdpBitmapSurface) -> VdpStatus>;
/// \brief Retrieve the parameters used to create a
/// VdpBitmapSurface.
/// \param[in] surface The surface's handle.
/// \param[out] rgba_format The format of the surface.
/// \param[out] width The width of the surface.
/// \param[out] height The height of the surface.
/// \param[out] frequently_accessed The frequently_accessed state
/// of the surface.
/// \return VdpStatus The completion status of the operation.
pub type VdpBitmapSurfaceGetParameters = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpBitmapSurface,
        rgba_format: *mut VdpRGBAFormat,
        width: *mut u32,
        height: *mut u32,
        frequently_accessed: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief Copy image data from application memory in the
/// surface's native format to a VdpBitmapSurface.
/// \param[in] surface The surface's handle.
/// \param[in] source_data Pointers to the application data
/// buffers from which the image data will be copied. Note
/// that this is an array of pointers, one per plane. The
/// source_format parameter will define how many
/// planes are required.
/// \param[in] source_pitches Pointers to the pitch values
/// for the application data buffers. Note that this is an
/// array of pointers, one per plane. The
/// source_format parameter will define how many
/// planes are required.
/// \param[in] destination_rect The sub-rectangle of the surface
/// to fill with application data. If NULL, the entire
/// surface will be updated.
/// \return VdpStatus The completion status of the operation.
pub type VdpBitmapSurfacePutBitsNative = ::std::option::Option<
    unsafe extern "C" fn(
        surface: VdpBitmapSurface,
        source_data: *const *const ::std::os::raw::c_void,
        source_pitches: *const u32,
        destination_rect: *const VdpRect,
    ) -> VdpStatus,
>;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_ZERO: VdpOutputSurfaceRenderBlendFactor = 0;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_ONE: VdpOutputSurfaceRenderBlendFactor = 1;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_SRC_COLOR: VdpOutputSurfaceRenderBlendFactor = 2;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_ONE_MINUS_SRC_COLOR:
    VdpOutputSurfaceRenderBlendFactor = 3;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_SRC_ALPHA: VdpOutputSurfaceRenderBlendFactor = 4;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA:
    VdpOutputSurfaceRenderBlendFactor = 5;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_DST_ALPHA: VdpOutputSurfaceRenderBlendFactor = 6;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_ONE_MINUS_DST_ALPHA:
    VdpOutputSurfaceRenderBlendFactor = 7;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_DST_COLOR: VdpOutputSurfaceRenderBlendFactor = 8;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_ONE_MINUS_DST_COLOR:
    VdpOutputSurfaceRenderBlendFactor = 9;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_SRC_ALPHA_SATURATE:
    VdpOutputSurfaceRenderBlendFactor = 10;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_CONSTANT_COLOR: VdpOutputSurfaceRenderBlendFactor =
    11;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR:
    VdpOutputSurfaceRenderBlendFactor = 12;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_CONSTANT_ALPHA: VdpOutputSurfaceRenderBlendFactor =
    13;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA:
    VdpOutputSurfaceRenderBlendFactor = 14;
pub type VdpOutputSurfaceRenderBlendFactor = ::std::os::raw::c_uint;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_EQUATION_SUBTRACT: VdpOutputSurfaceRenderBlendEquation =
    0;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_EQUATION_REVERSE_SUBTRACT:
    VdpOutputSurfaceRenderBlendEquation = 1;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_EQUATION_ADD: VdpOutputSurfaceRenderBlendEquation = 2;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_EQUATION_MIN: VdpOutputSurfaceRenderBlendEquation = 3;
pub const VDP_OUTPUT_SURFACE_RENDER_BLEND_EQUATION_MAX: VdpOutputSurfaceRenderBlendEquation = 4;
pub type VdpOutputSurfaceRenderBlendEquation = ::std::os::raw::c_uint;
/// \brief Complete blending operation definition.
///
/// A "blend state" operation controls the math behind certain rendering
/// operations.
///
/// The blend math is the familiar OpenGL blend math:
/// \f[
/// dst.a = equation(blendFactorDstAlpha*dst.a,
/// blendFactorSrcAlpha*src.a);
/// \f]
/// \f[
/// dst.rgb = equation(blendFactorDstColor*dst.rgb,
/// blendFactorSrcColor*src.rgb);
/// \f]
///
/// Note that when equation is MIN or MAX, the blend factors and constants
/// are ignored, and are treated as if they were 1.0.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpOutputSurfaceRenderBlendState {
    /// This field must be filled with VDP_OUTPUT_SURFACE_RENDER_BLEND_STATE_VERSIION
    pub struct_version: u32,
    pub blend_factor_source_color: VdpOutputSurfaceRenderBlendFactor,
    pub blend_factor_destination_color: VdpOutputSurfaceRenderBlendFactor,
    pub blend_factor_source_alpha: VdpOutputSurfaceRenderBlendFactor,
    pub blend_factor_destination_alpha: VdpOutputSurfaceRenderBlendFactor,
    pub blend_equation_color: VdpOutputSurfaceRenderBlendEquation,
    pub blend_equation_alpha: VdpOutputSurfaceRenderBlendEquation,
    pub blend_constant: VdpColor,
}
/// \brief Composite a sub-rectangle of a \ref VdpOutputSurface
/// "VdpOutputSurface" into a sub-rectangle of another
/// \ref VdpOutputSurface VdpOutputSurface.
/// \param[in] destination_surface The destination surface of the
/// compositing operation.
/// \param[in] destination_rect The sub-rectangle of the
/// destination surface to update. If NULL, the entire
/// destination surface will be updated.
/// \param[in] source_surface The source surface for the
/// compositing operation. The surface is treated as having
/// four components: red, green, blue and alpha. Any
/// missing components are treated as 1.0. For example, for
/// an A8 VdpOutputSurface, alpha will come from the surface
/// but red, green and blue will be treated as 1.0. If
/// source_surface is NULL, all components will be treated
/// as 1.0. Note that destination_surface and
/// source_surface must have been allocated via the same
/// \ref VdpDevice "VdpDevice".
/// \param[in] source_rect The sub-rectangle of the source
/// surface to read from. If NULL, the entire
/// source_surface will be read. Left/right and/or top/bottom
/// co-ordinates may be swapped to flip the source. Any
/// flip occurs prior to any requested rotation. Values
/// from outside the source surface are valid and samples
/// at those locations will be taken from the nearest edge.
/// \param[in] colors A pointer to an array of \ref VdpColor
/// "VdpColor" objects. If the flag
/// VDP_OUTPUT_SURFACE_RENDER_COLOR_PER_VERTEX is set,
/// VDPAU will four entries from the array, and treat them
/// as the colors corresponding to the upper-left,
/// upper-right, lower-right and lower-left corners of the
/// post-rotation source (i.e. indices 0, 1, 2 and 3 run
/// clockwise from the upper left corner). If the flag
/// VDP_OUTPUT_SURFACE_RENDER_COLOR_PER_VERTEX is not
/// set, VDPAU will use the single VdpColor for all four
/// corners. If colors is NULL then red, green, blue and
/// alpha values of 1.0 will be used.
/// \param[in] blend_state If a blend state is provided, the
/// blend state will be used for the composite operation. If
/// NULL, blending is effectively disabled, which is
/// equivalent to a blend equation of ADD, source blend
/// factors of ONE and destination blend factors of ZERO.
/// See \ref VdpOutputSurfaceRenderBlendState for details
/// regarding the mathematics of the blending operation.
/// \param[in] flags A set of flags influencing how the
/// compositing operation works.
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_ROTATE_0
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_ROTATE_90
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_ROTATE_180
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_ROTATE_270
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_COLOR_PER_VERTEX
/// \return VdpStatus The completion status of the operation.
///
/// The general compositing pipeline is as follows.
///
/// -# Extract source_rect from source_surface.
///
/// -# The extracted source is rotated 0, 90, 180 or 270 degrees
/// according to the flags.
///
/// -# The rotated source is component-wise multiplied by a
/// smooth-shaded quad with a (potentially) different color at
/// each vertex.
///
/// -# The resulting rotated, smooth-shaded quad is scaled to the
/// size of destination_rect and composited with
/// destination_surface using the provided blend state.
///
pub type VdpOutputSurfaceRenderOutputSurface = ::std::option::Option<
    unsafe extern "C" fn(
        destination_surface: VdpOutputSurface,
        destination_rect: *const VdpRect,
        source_surface: VdpOutputSurface,
        source_rect: *const VdpRect,
        colors: *const VdpColor,
        blend_state: *const VdpOutputSurfaceRenderBlendState,
        flags: u32,
    ) -> VdpStatus,
>;
/// \brief Composite a sub-rectangle of a \ref VdpBitmapSurface
/// "VdpBitmapSurface" into a sub-rectangle of a
/// \ref VdpOutputSurface VdpOutputSurface.
/// \param[in] destination_surface The destination surface of the
/// compositing operation.
/// \param[in] destination_rect The sub-rectangle of the
/// destination surface to update. If NULL, the entire
/// destination surface will be updated.
/// \param[in] source_surface The source surface for the
/// compositing operation. The surface is treated as having
/// four components: red, green, blue and alpha. Any
/// missing components are treated as 1.0. For example, for
/// an A8 VdpBitmapSurface, alpha will come from the surface
/// but red, green and blue will be treated as 1.0. If
/// source_surface is NULL, all components will be treated
/// as 1.0. Note that destination_surface and
/// source_surface must have been allocated via the same
/// \ref VdpDevice "VdpDevice".
/// \param[in] source_rect The sub-rectangle of the source
/// surface to read from. If NULL, the entire
/// source_surface will be read. Left/right ot top/bottom
/// co-ordinates may be swapped to flip the source. Any
/// flip occurs prior to any requested rotation. Values
/// from outside the source surface are valid and samples
/// at those locations will be taken from the nearest edge.
/// \param[in] colors A pointer to an array of \ref VdpColor
/// "VdpColor" objects. If the flag
/// VDP_OUTPUT_SURFACE_RENDER_COLOR_PER_VERTEX is set,
/// VDPAU will four entries from the array, and treat them
/// as the colors corresponding to the upper-left,
/// upper-right, lower-right and lower-left corners of the
/// post-rotation source (i.e. indices 0, 1, 2 and 3 run
/// clockwise from the upper left corner). If the flag
/// VDP_OUTPUT_SURFACE_RENDER_COLOR_PER_VERTEX is not
/// set, VDPAU will use the single VdpColor for all four
/// corners. If colors is NULL then red, green, blue and
/// alpha values of 1.0 will be used.
/// \param[in] blend_state If a blend state is provided, the
/// blend state will be used for the composite operation. If
/// NULL, blending is effectively disabled, which is
/// equivalent to a blend equation of ADD, source blend
/// factors of ONE and destination blend factors of ZERO.
/// See \ref VdpOutputSurfaceRenderBlendState for details
/// regarding the mathematics of the blending operation.
/// \param[in] flags A set of flags influencing how the
/// compositing operation works.
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_ROTATE_0
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_ROTATE_90
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_ROTATE_180
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_ROTATE_270
/// \arg \ref VDP_OUTPUT_SURFACE_RENDER_COLOR_PER_VERTEX
/// \return VdpStatus The completion status of the operation.
///
/// The general compositing pipeline is as follows.
///
/// -# Extract source_rect from source_surface.
///
/// -# The extracted source is rotated 0, 90, 180 or 270 degrees
/// according to the flags.
///
/// -# The rotated source is component-wise multiplied by a
/// smooth-shaded quad with a (potentially) different color at
/// each vertex.
///
/// -# The resulting rotated, smooth-shaded quad is scaled to the
/// size of destination_rect and composited with
/// destination_surface using the provided blend state.
///
pub type VdpOutputSurfaceRenderBitmapSurface = ::std::option::Option<
    unsafe extern "C" fn(
        destination_surface: VdpOutputSurface,
        destination_rect: *const VdpRect,
        source_surface: VdpBitmapSurface,
        source_rect: *const VdpRect,
        colors: *const VdpColor,
        blend_state: *const VdpOutputSurfaceRenderBlendState,
        flags: u32,
    ) -> VdpStatus,
>;
/// \brief The set of all known compressed video formats, and
/// associated profiles, that may be decoded.
pub type VdpDecoderProfile = u32;
/// \brief Query the implementation's VdpDecoder capabilities.
/// \param[in] device The device to query.
/// \param[in] profile The decoder profile for which information is requested.
/// \param[out] is_supported Is this profile supported?
/// \param[out] max_level The maximum specification level supported for this
/// profile.
/// \param[out] max_macroblocks The maximum supported surface size in
/// macroblocks. Note that this could be greater than that dictated by
/// the maximum level.
/// \param[out] max_width The maximum supported surface width for this profile.
/// Note that this could be greater than that dictated by the maximum
/// level.
/// \param[out] max_height The maximum supported surface height for this
/// profile. Note that this could be greater than that dictated by the
/// maximum level.
/// \return VdpStatus The completion status of the operation.
pub type VdpDecoderQueryCapabilities = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        profile: VdpDecoderProfile,
        is_supported: *mut VdpBool,
        max_level: *mut u32,
        max_macroblocks: *mut u32,
        max_width: *mut u32,
        max_height: *mut u32,
    ) -> VdpStatus,
>;
/// \brief An opaque handle representing a VdpDecoder object.
pub type VdpDecoder = u32;
/// \brief Create a VdpDecoder.
/// \param[in] device The device that will contain the surface.
/// \param[in] profile The video format the decoder will decode.
/// \param[in] width The width of the new surface.
/// \param[in] height The height of the new surface.
/// \param[in] max_references The maximum number of references that may be
/// used by a single frame in the stream to be decoded. This parameter
/// exists mainly for formats such as H.264, where different streams
/// may use a different number of references. Requesting too many
/// references may waste memory, but decoding should still operate
/// correctly. Requesting too few references will cause decoding to
/// fail.
/// \param[out] decoder The new decoder's handle.
/// \return VdpStatus The completion status of the operation.
pub type VdpDecoderCreate = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        profile: VdpDecoderProfile,
        width: u32,
        height: u32,
        max_references: u32,
        decoder: *mut VdpDecoder,
    ) -> VdpStatus,
>;
/// \brief Destroy a VdpDecoder.
/// \param[in] surface The decoder's handle.
/// \return VdpStatus The completion status of the operation.
pub type VdpDecoderDestroy =
    ::std::option::Option<unsafe extern "C" fn(decoder: VdpDecoder) -> VdpStatus>;
/// \brief Retrieve the parameters used to create a
/// VdpDecoder.
/// \param[in] surface The surface's handle.
/// \param[out] profile The video format used to create the
/// decoder.
/// \param[out] width The width of surfaces decode by the
/// decoder.
/// \param[out] height The height of surfaces decode by the
/// decoder
/// \return VdpStatus The completion status of the operation.
pub type VdpDecoderGetParameters = ::std::option::Option<
    unsafe extern "C" fn(
        decoder: VdpDecoder,
        profile: *mut VdpDecoderProfile,
        width: *mut u32,
        height: *mut u32,
    ) -> VdpStatus,
>;
/// \brief Application data buffer containing compressed video
/// data.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpBitstreamBuffer {
    /// This field must be filled with VDP_BITSTREAM_BUFFER_VERSION
    pub struct_version: u32,
    /// A pointer to the bitstream data bytes
    pub bitstream: *const ::std::os::raw::c_void,
    /// The number of data bytes
    pub bitstream_bytes: u32,
}
/// \brief A generic "picture information" pointer type.
///
/// This type serves solely to document the expected usage of a
/// generic (void *) function parameter. In actual usage, the
/// application is expected to physically provide a pointer to an
/// instance of one of the "real" VdpPictureInfo* structures,
/// picking the type appropriate for the decoder object in
/// question.
pub type VdpPictureInfo = *mut ::std::os::raw::c_void;
/// \brief Picture parameter information for an MPEG 1 or MPEG 2
/// picture.
///
/// Note: References to "copy of bitstream field" in the field descriptions
/// may refer to data literally parsed from the bitstream, or derived from
/// the bitstream using a mechanism described in the specification.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VdpPictureInfoMPEG1Or2 {
    /// Reference used by B and P frames.
    /// Set to VDP_INVALID_HANDLE when not used.
    pub forward_reference: VdpVideoSurface,
    /// Reference used by B frames.
    /// Set to VDP_INVALID_HANDLE when not used.
    pub backward_reference: VdpVideoSurface,
    /// Number of slices in the bitstream provided.
    pub slice_count: u32,
    /// Copy of the MPEG bitstream field.
    pub picture_structure: u8,
    /// Copy of the MPEG bitstream field.
    pub picture_coding_type: u8,
    /// Copy of the MPEG bitstream field.
    pub intra_dc_precision: u8,
    /// Copy of the MPEG bitstream field.
    pub frame_pred_frame_dct: u8,
    /// Copy of the MPEG bitstream field.
    pub concealment_motion_vectors: u8,
    /// Copy of the MPEG bitstream field.
    pub intra_vlc_format: u8,
    /// Copy of the MPEG bitstream field.
    pub alternate_scan: u8,
    /// Copy of the MPEG bitstream field.
    pub q_scale_type: u8,
    /// Copy of the MPEG bitstream field.
    pub top_field_first: u8,
    /// Copy of the MPEG-1 bitstream field. For MPEG-2, set to 0.
    pub full_pel_forward_vector: u8,
    /// Copy of the MPEG-1 bitstream field. For MPEG-2, set to 0.
    pub full_pel_backward_vector: u8,
    /// Copy of the MPEG bitstream field.
    /// For MPEG-1, fill both horizontal and vertical entries.
    pub f_code: [[u8; 2usize]; 2usize],
    /// Copy of the MPEG bitstream field, converted to raster order.
    pub intra_quantizer_matrix: [u8; 64usize],
    /// Copy of the MPEG bitstream field, converted to raster order.
    pub non_intra_quantizer_matrix: [u8; 64usize],
}
/// \brief Information about an H.264 reference frame
///
/// Note: References to "copy of bitstream field" in the field descriptions
/// may refer to data literally parsed from the bitstream, or derived from
/// the bitstream using a mechanism described in the specification.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpReferenceFrameH264 {
    /// The surface that contains the reference image.
    /// Set to VDP_INVALID_HANDLE for unused entries.
    pub surface: VdpVideoSurface,
    /// Is this a long term reference (else short term).
    pub is_long_term: VdpBool,
    /// Is the top field used as a reference.
    /// Set to VDP_FALSE for unused entries.
    pub top_is_reference: VdpBool,
    /// Is the bottom field used as a reference.
    /// Set to VDP_FALSE for unused entries.
    pub bottom_is_reference: VdpBool,
    /// [0]: top, [1]: bottom
    pub field_order_cnt: [i32; 2usize],
    /// Copy of the H.264 bitstream field:
    /// frame_num from slice_header for short-term references,
    /// LongTermPicNum from decoding algorithm for long-term references.
    pub frame_idx: u16,
}
/// \brief Picture parameter information for an H.264 picture.
///
/// Note: The \ref referenceFrames array must contain the "DPB" as
/// defined by the H.264 specification. In particular, once a
/// reference frame has been decoded to a surface, that surface must
/// continue to appear in the DPB until no longer required to predict
/// any future frame. Once a surface is removed from the DPB, it can
/// no longer be used as a reference, unless decoded again.
///
/// Also note that only surfaces previously generated using \ref
/// VdpDecoderRender may be used as reference frames. In particular,
/// surfaces filled using any "put bits" API will not work.
///
/// Note: References to "copy of bitstream field" in the field descriptions
/// may refer to data literally parsed from the bitstream, or derived from
/// the bitstream using a mechanism described in the specification.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VdpPictureInfoH264 {
    /// Number of slices in the bitstream provided.
    pub slice_count: u32,
    /// [0]: top, [1]: bottom
    pub field_order_cnt: [i32; 2usize],
    /// Will the decoded frame be used as a reference later.
    pub is_reference: VdpBool,
    /// Copy of the H.264 bitstream field.
    pub frame_num: u16,
    /// Copy of the H.264 bitstream field.
    pub field_pic_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub bottom_field_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub num_ref_frames: u8,
    /// Copy of the H.264 bitstream field.
    pub mb_adaptive_frame_field_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub constrained_intra_pred_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub weighted_pred_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub weighted_bipred_idc: u8,
    /// Copy of the H.264 bitstream field.
    pub frame_mbs_only_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub transform_8x8_mode_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub chroma_qp_index_offset: i8,
    /// Copy of the H.264 bitstream field.
    pub second_chroma_qp_index_offset: i8,
    /// Copy of the H.264 bitstream field.
    pub pic_init_qp_minus26: i8,
    /// Copy of the H.264 bitstream field.
    pub num_ref_idx_l0_active_minus1: u8,
    /// Copy of the H.264 bitstream field.
    pub num_ref_idx_l1_active_minus1: u8,
    /// Copy of the H.264 bitstream field.
    pub log2_max_frame_num_minus4: u8,
    /// Copy of the H.264 bitstream field.
    pub pic_order_cnt_type: u8,
    /// Copy of the H.264 bitstream field.
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    /// Copy of the H.264 bitstream field.
    pub delta_pic_order_always_zero_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub direct_8x8_inference_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub entropy_coding_mode_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub pic_order_present_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub deblocking_filter_control_present_flag: u8,
    /// Copy of the H.264 bitstream field.
    pub redundant_pic_cnt_present_flag: u8,
    /// Copy of the H.264 bitstream field, converted to raster order.
    pub scaling_lists_4x4: [[u8; 16usize]; 6usize],
    /// Copy of the H.264 bitstream field, converted to raster order.
    pub scaling_lists_8x8: [[u8; 64usize]; 2usize],
    /// See \ref VdpPictureInfoH264 for instructions regarding this field.
    pub referenceFrames: [VdpReferenceFrameH264; 16usize],
}
/// \brief Picture parameter information for a VC1 picture.
///
/// Note: References to "copy of bitstream field" in the field descriptions
/// may refer to data literally parsed from the bitstream, or derived from
/// the bitstream using a mechanism described in the specification.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpPictureInfoVC1 {
    /// Reference used by B and P frames.
    /// Set to VDP_INVALID_HANDLE when not used.
    pub forward_reference: VdpVideoSurface,
    /// Reference used by B frames.
    /// Set to VDP_INVALID_HANDLE when not used.
    pub backward_reference: VdpVideoSurface,
    /// Number of slices in the bitstream provided.
    pub slice_count: u32,
    /// I=0, P=1, B=3, BI=4  from 7.1.1.4.
    pub picture_type: u8,
    /// Progressive=0, Frame-interlace=2, Field-interlace=3; see VC-1 7.1.1.15.
    pub frame_coding_mode: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.1.5.
    pub postprocflag: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.1.8.
    pub pulldown: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.1.9.
    pub interlace: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.1.10.
    pub tfcntrflag: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.1.11.
    pub finterpflag: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.1.3.
    pub psf: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.8.
    pub dquant: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.3.
    pub panscan_flag: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.4.
    pub refdist_flag: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.11.
    pub quantizer: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.7.
    pub extended_mv: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.14.
    pub extended_dmv: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.10.
    pub overlap: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.9.
    pub vstransform: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.5.
    pub loopfilter: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.6.
    pub fastuvmc: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.12.15.
    pub range_mapy_flag: u8,
    /// Copy of the VC-1 bitstream field.
    pub range_mapy: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 6.2.16.
    pub range_mapuv_flag: u8,
    /// Copy of the VC-1 bitstream field.
    pub range_mapuv: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 J.1.10.
    /// Only used by simple and main profiles.
    pub multires: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 J.1.16.
    /// Only used by simple and main profiles.
    pub syncmarker: u8,
    /// VC-1 SP/MP range reduction control.
    /// Only used by simple and main profiles.
    /// Bit 0: Copy of rangered VC-1 bitstream field; See VC-1 J.1.17.
    /// Bit 1: Copy of rangeredfrm VC-1 bitstream fiels; See VC-1 7.1.13.
    pub rangered: u8,
    /// Copy of the VC-1 bitstream field. See VC-1 J.1.17.
    /// Only used by simple and main profiles.
    pub maxbframes: u8,
    /// Out-of-loop deblocking enable.
    /// Bit 0 of POSTPROC from VC-1 7.1.1.27
    /// Note that bit 1 of POSTPROC (dering enable) should not be included.
    pub deblockEnable: u8,
    /// Parameter used by VC-1 Annex H deblocking algorithm. Note that VDPAU
    /// implementations may choose which deblocking algorithm to use.
    /// See VC-1 7.1.1.6
    pub pquant: u8,
}
/// \brief Picture parameter information for an MPEG-4 Part 2 picture.
///
/// Note: References to "copy of bitstream field" in the field descriptions
/// may refer to data literally parsed from the bitstream, or derived from
/// the bitstream using a mechanism described in the specification.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VdpPictureInfoMPEG4Part2 {
    /// Reference used by B and P frames.
    /// Set to VDP_INVALID_HANDLE when not used.
    pub forward_reference: VdpVideoSurface,
    /// Reference used by B frames.
    /// Set to VDP_INVALID_HANDLE when not used.
    pub backward_reference: VdpVideoSurface,
    /// Copy of the bitstream field.
    pub trd: [i32; 2usize],
    /// Copy of the bitstream field.
    pub trb: [i32; 2usize],
    /// Copy of the bitstream field.
    pub vop_time_increment_resolution: u16,
    /// Copy of the bitstream field.
    pub vop_coding_type: u8,
    /// Copy of the bitstream field.
    pub vop_fcode_forward: u8,
    /// Copy of the bitstream field.
    pub vop_fcode_backward: u8,
    /// Copy of the bitstream field.
    pub resync_marker_disable: u8,
    /// Copy of the bitstream field.
    pub interlaced: u8,
    /// Copy of the bitstream field.
    pub quant_type: u8,
    /// Copy of the bitstream field.
    pub quarter_sample: u8,
    /// Copy of the bitstream field.
    pub short_video_header: u8,
    /// Derived from vop_rounding_type bitstream field.
    pub rounding_control: u8,
    /// Copy of the bitstream field.
    pub alternate_vertical_scan_flag: u8,
    /// Copy of the bitstream field.
    pub top_field_first: u8,
    /// Copy of the bitstream field.
    pub intra_quantizer_matrix: [u8; 64usize],
    /// Copy of the bitstream field.
    pub non_intra_quantizer_matrix: [u8; 64usize],
}
/// \brief Picture parameter information for a DivX 4 picture.
///
/// Due to similarites between MPEG-4 Part 2 and DivX 4, the picture
/// parameter structure is re-used.
pub type VdpPictureInfoDivX4 = VdpPictureInfoMPEG4Part2;
/// \brief Picture parameter information for a DivX 5 picture.
///
/// Due to similarites between MPEG-4 Part 2 and DivX 5, the picture
/// parameter structure is re-used.
pub type VdpPictureInfoDivX5 = VdpPictureInfoMPEG4Part2;
/// \brief Decode a compressed field/frame and render the result
/// into a \ref VdpVideoSurface "VdpVideoSurface".
/// \param[in] decoder The decoder object that will perform the
/// decode operation.
/// \param[in] target The video surface to render to.
/// \param[in] picture_info A (pointer to a) structure containing
/// information about the picture to be decoded. Note that
/// the appropriate type of VdpPictureInfo* structure must
/// be provided to match to profile that the decoder was
/// created for.
/// \param[in] bitstream_buffer_count The number of bitstream
/// buffers containing compressed data for this picture.
/// \param[in] bitstream_buffers An array of bitstream buffers.
/// \return VdpStatus The completion status of the operation.
///
/// See \ref video_mixer_usage for additional information.
pub type VdpDecoderRender = ::std::option::Option<
    unsafe extern "C" fn(
        decoder: VdpDecoder,
        target: VdpVideoSurface,
        picture_info: *const VdpPictureInfo,
        bitstream_buffer_count: u32,
        bitstream_buffers: *const VdpBitstreamBuffer,
    ) -> VdpStatus,
>;
/// \brief A VdpVideoMixer feature that must be requested at
/// creation time to be used.
///
/// Certain advanced VdpVideoMixer features are optional, and the
/// ability to use those features at all must be requested when
/// the VdpVideoMixer object is created. Each feature is named via
/// a specific VdpVideoMixerFeature value.
///
/// Once requested, these features are permanently available
/// within that specific VdpVideoMixer object. All features that
/// are not explicitly requested at creation time default to
/// being permanently unavailable.
///
/// Even when requested, all features default to being initially
/// disabled. However, applications can subsequently enable and
/// disable features at any time. See \ref
/// VdpVideoMixerSetFeatureEnables.
///
/// Some features allow configuration of their operation. Each
/// configurable item is an \ref VdpVideoMixerAttribute. These
/// attributes may be manipulated at any time using \ref
/// VdpVideoMixerSetAttributeValues.
pub type VdpVideoMixerFeature = u32;
/// \brief A VdpVideoMixer creation parameter.
///
/// When a VdpVideoMixer is created, certain parameters may be
/// supplied. Each parameter is named via a specific
/// VdpVideoMixerParameter value.
///
/// Each parameter has a specific type, and specific default
/// value if not specified at VdpVideoMixer creation time. The
/// application may query the legal supported range for some
/// parameters.
pub type VdpVideoMixerParameter = u32;
/// \brief An adjustable attribute of VdpVideoMixer operation.
///
/// Various attributes of VdpVideoMixer operation may be adjusted
/// at any time. Each attribute is named via a specific
/// VdpVideoMixerAttribute value.
///
/// Each attribute has a specific type, and specific default
/// value if not specified at VdpVideoMixer creation time. The
/// application may query the legal supported range for some
/// attributes.
pub type VdpVideoMixerAttribute = u32;
/// \brief Query the implementation's support for a specific
/// feature.
/// \param[in] device The device to query.
/// \param[in] feature The feature for which support is to be
/// queried.
/// \param[out] is_supported Is the specified feature supported?
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerQueryFeatureSupport = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        feature: VdpVideoMixerFeature,
        is_supported: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's support for a specific
/// parameter.
/// \param[in] device The device to query.
/// \param[in] parameter The parameter for which support is to be
/// queried.
/// \param[out] is_supported Is the specified parameter
/// supported?
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerQueryParameterSupport = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        parameter: VdpVideoMixerParameter,
        is_supported: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's support for a specific
/// attribute.
/// \param[in] device The device to query.
/// \param[in] feature The feature for which support is to be
/// queried.
/// \param[out] is_supported Is the specified feature supported?
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerQueryAttributeSupport = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        attribute: VdpVideoMixerAttribute,
        is_supported: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's supported for a specific
/// parameter.
/// \param[in] device The device to query.
/// \param[in] parameter The parameter for which support is to be
/// queried.
/// \param[out] min_value The minimum supported value.
/// \param[out] max_value The maximum supported value.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerQueryParameterValueRange = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        parameter: VdpVideoMixerParameter,
        min_value: *mut ::std::os::raw::c_void,
        max_value: *mut ::std::os::raw::c_void,
    ) -> VdpStatus,
>;
/// \brief Query the implementation's supported for a specific
/// attribute.
/// \param[in] device The device to query.
/// \param[in] attribute The attribute for which support is to be
/// queried.
/// \param[out] min_value The minimum supported value.
/// \param[out] max_value The maximum supported value.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerQueryAttributeValueRange = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        attribute: VdpVideoMixerAttribute,
        min_value: *mut ::std::os::raw::c_void,
        max_value: *mut ::std::os::raw::c_void,
    ) -> VdpStatus,
>;
/// \brief An opaque handle representing a VdpVideoMixer object.
pub type VdpVideoMixer = u32;
/// \brief Create a VdpVideoMixer.
/// \param[in] device The device that will contain the mixer.
/// \param[in] feature_count The number of features to request.
/// \param[in] features The list of features to request.
/// \param[in] parameter_count The number of parameters to set.
/// \param[in] parameters The list of parameters to set.
/// \param[in] parameter_values The values for the parameters. Note that each
/// entry in the value array is a pointer to the actual value. In other
/// words, the values themselves are not cast to "void *" and passed
/// "inside" the array.
/// \param[out] mixer The new mixer's handle.
/// \return VdpStatus The completion status of the operation.
///
/// Initially, all requested features will be disabled. They can
/// be enabled using \ref VdpVideoMixerSetFeatureEnables.
///
/// Initially, all attributes will have default values. Values
/// can be changed using \ref VdpVideoMixerSetAttributeValues.
pub type VdpVideoMixerCreate = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        feature_count: u32,
        features: *const VdpVideoMixerFeature,
        parameter_count: u32,
        parameters: *const VdpVideoMixerParameter,
        parameter_values: *const *const ::std::os::raw::c_void,
        mixer: *mut VdpVideoMixer,
    ) -> VdpStatus,
>;
/// \brief Enable or disable features.
/// \param[in] mixer The mixer to manipulate.
/// \param[in] feature_count The number of features to
/// enable/disable.
/// \param[in] features The list of features to enable/disable.
/// \param[in] feature_enables The list of new feature enable
/// values.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerSetFeatureEnables = ::std::option::Option<
    unsafe extern "C" fn(
        mixer: VdpVideoMixer,
        feature_count: u32,
        features: *const VdpVideoMixerFeature,
        feature_enables: *const VdpBool,
    ) -> VdpStatus,
>;
/// \brief Set attribute values
/// \param[in] mixer The mixer to manipulate.
/// \param[in] attribute_count The number of attributes to set.
/// \param[in] attributes The list of attributes to set.
/// \param[in] attribute_values The values for the attributes. Note that each
/// entry in the value array is a pointer to the actual value. In other
/// words, the values themselves are not cast to "void *" and passed
/// "inside" the array. A NULL pointer requests that the default value be
/// set for that attribute.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerSetAttributeValues = ::std::option::Option<
    unsafe extern "C" fn(
        mixer: VdpVideoMixer,
        attribute_count: u32,
        attributes: *const VdpVideoMixerAttribute,
        attribute_values: *const *const ::std::os::raw::c_void,
    ) -> VdpStatus,
>;
/// \brief Retrieve whether features were requested at creation
/// time.
/// \param[in] mixer The mixer to query.
/// \param[in] feature_count The number of features to query.
/// \param[in] features The list of features to query.
/// \param[out] feature_supported A list of values indicating
/// whether the feature was requested, and hence is
/// available.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerGetFeatureSupport = ::std::option::Option<
    unsafe extern "C" fn(
        mixer: VdpVideoMixer,
        feature_count: u32,
        features: *const VdpVideoMixerFeature,
        feature_supports: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief Retrieve whether features are enabled.
/// \param[in] mixer The mixer to manipulate.
/// \param[in] feature_count The number of features to query.
/// \param[in] features The list of features to query.
/// \param[out] feature_enabled A list of values indicating
/// whether the feature is enabled.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerGetFeatureEnables = ::std::option::Option<
    unsafe extern "C" fn(
        mixer: VdpVideoMixer,
        feature_count: u32,
        features: *const VdpVideoMixerFeature,
        feature_enables: *mut VdpBool,
    ) -> VdpStatus,
>;
/// \brief Retrieve parameter values given at creation time.
/// \param[in] mixer The mixer to manipulate.
/// \param[in] parameter_count The number of parameters to query.
/// \param[in] parameters The list of parameters to query.
/// \param[out] parameter_values The list of current values for
/// the parameters. Note that each entry in the value array is a pointer to
/// storage that will receive the actual value. If the attribute's type is
/// a pointer itself, please closely read the documentation for that
/// attribute type for any other data passing requirements.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerGetParameterValues = ::std::option::Option<
    unsafe extern "C" fn(
        mixer: VdpVideoMixer,
        parameter_count: u32,
        parameters: *const VdpVideoMixerParameter,
        parameter_values: *const *const ::std::os::raw::c_void,
    ) -> VdpStatus,
>;
/// \brief Retrieve current attribute values.
/// \param[in] mixer The mixer to manipulate.
/// \param[in] attribute_count The number of attributes to query.
/// \param[in] attributes The list of attributes to query.
/// \param[out] attribute_values The list of current values for
/// the attributes. Note that each entry in the value array is a pointer to
/// storage that will receive the actual value. If the attribute's type is
/// a pointer itself, please closely read the documentation for that
/// attribute type for any other data passing requirements.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerGetAttributeValues = ::std::option::Option<
    unsafe extern "C" fn(
        mixer: VdpVideoMixer,
        attribute_count: u32,
        attributes: *const VdpVideoMixerAttribute,
        attribute_values: *const *const ::std::os::raw::c_void,
    ) -> VdpStatus,
>;
/// \brief Destroy a VdpVideoMixer.
/// \param[in] device The device to destroy.
/// \return VdpStatus The completion status of the operation.
pub type VdpVideoMixerDestroy =
    ::std::option::Option<unsafe extern "C" fn(mixer: VdpVideoMixer) -> VdpStatus>;
pub const VDP_VIDEO_MIXER_PICTURE_STRUCTURE_TOP_FIELD: VdpVideoMixerPictureStructure = 0;
pub const VDP_VIDEO_MIXER_PICTURE_STRUCTURE_BOTTOM_FIELD: VdpVideoMixerPictureStructure = 1;
pub const VDP_VIDEO_MIXER_PICTURE_STRUCTURE_FRAME: VdpVideoMixerPictureStructure = 2;
pub type VdpVideoMixerPictureStructure = ::std::os::raw::c_uint;
/// \brief Definition of an additional \ref VdpOutputSurface
/// "VdpOutputSurface" layer in the composting model.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VdpLayer {
    /// This field must be filled with VDP_LAYER_VERSION
    pub struct_version: u32,
    /// The surface to composite from.
    pub source_surface: VdpOutputSurface,
    /// The sub-rectangle of the source surface to use. If NULL, the
    /// entire source surface will be used.
    pub source_rect: *const VdpRect,
    /// The sub-rectangle of the destination surface to map
    /// this layer into. This rectangle is relative to the entire
    /// destination surface. This rectangle will be clipped by \ref
    /// VdpVideoMixerRender's \b destination_rect. If NULL, the
    /// destination rectangle will be sized to match the source
    /// rectangle, and will be located at the origin.
    pub destination_rect: *const VdpRect,
}
/// \brief Perform a video post-processing and compositing
/// operation.
/// \param[in] mixer The mixer object that will perform the
/// mixing/rendering operation.
/// \param[in] background_surface A background image. If set to any value other
/// than VDP_INVALID_HANDLE, the specific surface will be used instead of
/// the background color as the first layer in the mixer's compositing
/// process.
/// \param[in] background_source_rect When background_surface is specified,
/// this parameter indicates the portion of background_surface that will
/// be used as the background layer. The specified region will be
/// extracted and scaled to match the size of destination_rect. If NULL,
/// the entire background_surface will be used.
/// \param[in] current_picture_structure The picture structure of
/// the field/frame to be processed. This field/frame is
/// presented in the \b video_surface_current parameter. If
/// frame, then all \b video_surface_* parameters are
/// assumed to be frames. If field, then all
/// video_surface_* parameters are assumed to be fields,
/// with alternating top/bottom-ness derived from
/// video_surface_current.
/// \param[in] video_surfaces_past_count The number of provided
/// fields/frames prior to the current picture.
/// \param[in] video_surfaces_past The fields/frames prior to the
/// current field/frame. Note that array index 0 is the
/// field/frame temporally nearest to the current
/// field/frame, with increasing array indices used for
/// older frames. Unavailable entries may be set to
/// \ref VDP_INVALID_HANDLE.
/// \param[in] video_surface_current The field/frame to be
/// processed.
/// \param[in] video_surfaces_future_count The number of provided
/// fields/frames following the current picture.
/// \param[in] video_surfaces_future The fields/frames that
/// follow the current field/frame. Note that array index 0
/// is the field/frame temporally nearest to the current
/// field/frame, with increasing array indices used for
/// newer frames. Unavailable entries may be set to \ref
/// VDP_INVALID_HANDLE.
/// \param[in] video_source_rect The sub-rectangle of the source
/// video surface to extract and process. If NULL, the
/// entire surface will be used. Left/right and/or top/bottom
/// co-ordinates may be swapped to flip the source. Values
/// from outside the video surface are valid and samples
/// at those locations will be taken from the nearest edge.
/// \param[in] destination_surface
/// \param[in] destination_rect The sub-rectangle of the
/// destination surface to modify. Note that rectangle clips
/// all other actions.
/// \param[in] destination_video_rect The sub-rectangle of the
/// destination surface that will contain the processed
/// video. This rectangle is relative to the entire
/// destination surface. This rectangle is clipped by \b
/// destination_rect. If NULL, the destination rectangle
/// will be sized to match the source rectangle, and will
/// be located at the origin.
/// \param[in] layer_count The number of additional layers to
/// composite above the video.
/// \param[in] layers The array of additional layers to composite
/// above the video.
/// \return VdpStatus The completion status of the operation.
///
/// For a complete discussion of how to use this API, please see
/// \ref video_mixer_usage.
pub type VdpVideoMixerRender = ::std::option::Option<
    unsafe extern "C" fn(
        mixer: VdpVideoMixer,
        background_surface: VdpOutputSurface,
        background_source_rect: *const VdpRect,
        current_picture_structure: VdpVideoMixerPictureStructure,
        video_surface_past_count: u32,
        video_surface_past: *const VdpVideoSurface,
        video_surface_current: VdpVideoSurface,
        video_surface_future_count: u32,
        video_surface_future: *const VdpVideoSurface,
        video_source_rect: *const VdpRect,
        destination_surface: VdpOutputSurface,
        destination_rect: *const VdpRect,
        destination_video_rect: *const VdpRect,
        layer_count: u32,
        layers: *const VdpLayer,
    ) -> VdpStatus,
>;
/// \brief The representation of a point in time.
///
/// VdpTime timestamps are intended to be a high-precision timing
/// system, potentially independent from any other time domain in
/// the system.
///
/// Time is represented in units of nanoseconds. The origin
/// (i.e. the time represented by a value of 0) is implementation
/// dependent.
pub type VdpTime = u64;
/// \brief An opaque handle representing the location where
/// video will be presented.
///
/// VdpPresentationQueueTarget are created using a \ref api_winsys
/// specific API, such as \ref
/// VdpPresentationQueueTargetCreateX11.
pub type VdpPresentationQueueTarget = u32;
/// \brief Destroy a VdpPresentationQueueTarget.
/// \param[in] presentation_queue_target The target to destroy.
/// \return VdpStatus The completion status of the operation.
pub type VdpPresentationQueueTargetDestroy = ::std::option::Option<
    unsafe extern "C" fn(presentation_queue_target: VdpPresentationQueueTarget) -> VdpStatus,
>;
/// \brief An opaque handle representing a presentation queue
/// object.
pub type VdpPresentationQueue = u32;
/// \brief Create a VdpPresentationQueue.
/// \param[in] device The device that will contain the queue.
/// \param[in] presentation_queue_target The location to display
/// the content.
/// \param[out] presentation_queue The new queue's handle.
/// \return VdpStatus The completion status of the operation.
///
/// Note: The initial value for the background color will be set to
/// an implementation-defined value.
pub type VdpPresentationQueueCreate = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        presentation_queue_target: VdpPresentationQueueTarget,
        presentation_queue: *mut VdpPresentationQueue,
    ) -> VdpStatus,
>;
/// \brief Destroy a VdpPresentationQueue.
/// \param[in] presentation_queue The queue to destroy.
/// \return VdpStatus The completion status of the operation.
pub type VdpPresentationQueueDestroy = ::std::option::Option<
    unsafe extern "C" fn(presentation_queue: VdpPresentationQueue) -> VdpStatus,
>;
/// \brief Configure the background color setting.
/// \param[in] presentation_queue The queue to manipulate.
/// \param[in] background_color The new background color.
///
/// Note: Implementations may choose whether to apply the
/// new background color value immediately, or defer it until
/// the next surface is presented.
pub type VdpPresentationQueueSetBackgroundColor = ::std::option::Option<
    unsafe extern "C" fn(
        presentation_queue: VdpPresentationQueue,
        background_color: *const VdpColor,
    ) -> VdpStatus,
>;
/// \brief Retrieve the current background color setting.
/// \param[in] presentation_queue The queue to query.
/// \param[out] background_color The current background color.
pub type VdpPresentationQueueGetBackgroundColor = ::std::option::Option<
    unsafe extern "C" fn(presentation_queue: VdpPresentationQueue, background_color: *mut VdpColor)
        -> VdpStatus,
>;
/// \brief Retrieve the presentation queue's "current" time.
/// \param[in] presentation_queue The queue to query.
/// \param[out] current_time The current time, which may
/// represent a point between display VSYNC events.
/// \return VdpStatus The completion status of the operation.
pub type VdpPresentationQueueGetTime = ::std::option::Option<
    unsafe extern "C" fn(presentation_queue: VdpPresentationQueue, current_time: *mut VdpTime)
        -> VdpStatus,
>;
/// \brief Enter a surface into the presentation queue.
/// \param[in] presentation_queue The queue to query.
/// \param[in] surface The surface to enter into the queue.
/// \param[in] clip_width If set to a non-zero value, the presentation queue
/// will display only clip_width pixels of the surface (anchored to the
/// top-left corner of the surface.
/// \param[in] clip_height If set to a non-zero value, the presentation queue
/// will display only clip_height lines of the surface (anchored to the
/// top-left corner of the surface.
/// \param[in] earliest_presentation_time The timestamp
/// associated with the surface. The presentation queue
/// will not display the surface until the presentation
/// queue's current time is at least this value.
/// \return VdpStatus The completion status of the operation.
///
/// Applications may choose to allow resizing of the presentation queue target
/// (which may be e.g. a regular Window when using an X11-based
/// implementation).
///
/// \b clip_width and \b clip_height may be used to limit the size of the
/// displayed region of a surface, in order to match the specific region that
/// was rendered to.
///
/// In turn, this allows the application to allocate over-sized (e.g.
/// screen-sized) surfaces, but render to a region that matches the current
/// size of the video window.
///
/// Using this technique, an application's response to window resizing may
/// simply be to render to, and display, a different region of the surface,
/// rather than de-/re-allocation of surfaces to match the updated window size.
///
/// Implementations may impose an upper bound on the number of entries
/// contained by the presentation queue at a given time. This limit is likely
/// different to the number of \ref VdpOutputSurface "VdpOutputSurface"s that
/// may be allocated at a given time. This limit applies to entries in the
/// QUEUED or VISIBLE state only. In other words, entries that have
/// transitioned from a QUEUED or VISIBLE state to an IDLE state do not count
/// toward this limit.
pub type VdpPresentationQueueDisplay = ::std::option::Option<
    unsafe extern "C" fn(
        presentation_queue: VdpPresentationQueue,
        surface: VdpOutputSurface,
        clip_width: u32,
        clip_height: u32,
        earliest_presentation_time: VdpTime,
    ) -> VdpStatus,
>;
/// \brief Wait for a surface to finish being displayed.
/// \param[in] presentation_queue The queue to query.
/// \param[in] surface The surface to wait for.
/// \param[out] first_presentation_time The timestamp of the
/// VSYNC at which this surface was first displayed. Note
/// that 0 means the surface was never displayed.
/// \return VdpStatus The completion status of the operation.
///
/// Note that this API would block forever if queried about the surface most
/// recently added to a presentation queue. That is because there would be no
/// other surface that could possibly replace that surface as the currently
/// displayed surface, and hence that surface would never become idle. For
/// that reason, this function will return an error in that case.
pub type VdpPresentationQueueBlockUntilSurfaceIdle = ::std::option::Option<
    unsafe extern "C" fn(
        presentation_queue: VdpPresentationQueue,
        surface: VdpOutputSurface,
        first_presentation_time: *mut VdpTime,
    ) -> VdpStatus,
>;
pub const VDP_PRESENTATION_QUEUE_STATUS_IDLE: VdpPresentationQueueStatus = 0;
pub const VDP_PRESENTATION_QUEUE_STATUS_QUEUED: VdpPresentationQueueStatus = 1;
pub const VDP_PRESENTATION_QUEUE_STATUS_VISIBLE: VdpPresentationQueueStatus = 2;
pub type VdpPresentationQueueStatus = ::std::os::raw::c_uint;
/// \brief Poll the current queue status of a surface.
/// \param[in] presentation_queue The queue to query.
/// \param[in] surface The surface to query.
/// \param[out] status The current status of the surface within
/// the queue.
/// \param[out] first_presentation_time The timestamp of the
/// VSYNC at which this surface was first displayed. Note
/// that 0 means the surface was never displayed.
/// \return VdpStatus The completion status of the operation.
pub type VdpPresentationQueueQuerySurfaceStatus = ::std::option::Option<
    unsafe extern "C" fn(
        presentation_queue: VdpPresentationQueue,
        surface: VdpOutputSurface,
        status: *mut VdpPresentationQueueStatus,
        first_presentation_time: *mut VdpTime,
    ) -> VdpStatus,
>;
/// \brief A callback to notify the client application that a
/// device's display has been preempted.
/// \param[in] device The device that had its display preempted.
/// \param[in] context The client-supplied callback context
/// information.
/// \return void No return value
pub type VdpPreemptionCallback = ::std::option::Option<
    unsafe extern "C" fn(device: VdpDevice, context: *mut ::std::os::raw::c_void),
>;
/// \brief Configure the display preemption callback.
/// \param[in] device The device to be monitored for preemption.
/// \param[in] callback The client application's callback
/// function. If NULL, the callback is unregistered.
/// \param[in] context The client-supplied callback context
/// information. This information will be passed to the
/// callback function if/when invoked.
/// \return VdpStatus The completion status of the operation.
pub type VdpPreemptionCallbackRegister = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        callback: VdpPreemptionCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> VdpStatus,
>;
/// \brief A type suitable for \ref VdpGetProcAddress
/// "VdpGetProcAddress"'s \b function_id parameter.
pub type VdpFuncId = u32;
/// \brief Retrieve a VDPAU function pointer.
/// \param[in] device The device that the function will operate
/// against.
/// \param[in] function_id The specific function to retrieve.
/// \param[out] function_pointer The actual pointer for the
/// application to call.
/// \return VdpStatus The completion status of the operation.
pub type VdpGetProcAddress = ::std::option::Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        function_id: VdpFuncId,
        function_pointer: *mut *mut ::std::os::raw::c_void,
    ) -> VdpStatus,
>;


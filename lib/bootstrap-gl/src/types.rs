use std::mem;
use std::ops::BitOr;

// ======================
// OPENGL PRIMITIVE TYPES
// ======================

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boolean {
    False = 0,
    True = 1,
}

pub type Byte = i8;
pub type UByte = u8;
pub type Short = i16;
pub type UShort = u16;
pub type Int = i32;
pub type UInt = u32;
pub type Fixed = i32;
pub type Int64 = i64;
pub type UInt64 = u64;
pub type SizeI = i32;
pub type Enum = u32;
pub type IntPtr = isize;
pub type SizeIPtr = usize;
pub type Sync = usize;
pub type BitField = u32;
pub type Half = u16;
pub type Float = f32;
pub type ClampF = f32;
pub type Double = f64;
pub type ClampD = f64;

#[allow(non_camel_case_types)]
pub type f16 = u16;

// ============================
// OPENGL TYPE AND CUSTOM TYPES
// ============================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeLocation(u32);

impl AttributeLocation {
    pub fn from_index(index: u32) -> AttributeLocation {
        AttributeLocation(index)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferName(u32);

impl BufferName {
    pub fn null() -> BufferName {
        BufferName(0)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferTarget {
    Array = 0x8892,
    AtomicCounter = 0x92C0,
    CopyRead = 0x8F36,
    CopyWrite = 0x8F37,
    Uniform = 0x8A11,
    DispatchIndirect = 0x90EE,
    DrawIndirect = 0x8F3F,
    ElementArray = 0x8893,
    PixelPack = 0x88EB,
    PixelUnpack = 0x88EC,
    Query = 0x9192,
    ShaderStorage = 0x90D2,
    Texture = 0x8C2A,
    TransformFeedback = 0x8C8E,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    StreamDraw = 0x88E0,
    StreamRead = 0x88E1,
    StreamCopy = 0x88E2,
    StaticDraw = 0x88E4,
    StaticRead = 0x88E5,
    StaticCopy = 0x88E6,
    DynamicDraw = 0x88E8,
    DynamicRead = 0x88E9,
    DynamicCopy = 0x88EA,
}

/// TODO: Custom derive for Debug to show which flags are set.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearBufferMask {
    Depth = 0x00000100,
    Stencil = 0x00000400,
    Color = 0x00004000,
}

impl BitOr for ClearBufferMask {
    type Output = ClearBufferMask;

    fn bitor(self, rhs: ClearBufferMask) -> ClearBufferMask {
        unsafe { mem::transmute(self as u32 | rhs as u32) }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    Never = 0x0200,
    Less = 0x0201,
    Equal = 0x0202,
    LessThanOrEqual = 0x0203,
    Greater = 0x0204,
    NotEqual = 0x0205,
    GreaterThanOrEqual = 0x0206,
    Always = 0x0207,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawMode {
    Points = 0x0000,
    Lines = 0x0001,
    LineLoop = 0x0002,
    LineStrip = 0x0003,
    Triangles = 0x0004,
    TriangleStrip = 0x0005,
    TriangleFan = 0x0006,
    Quads = 0x0007,
    // GL_QUAD_STRIP
    // GL_POLYGON
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Front = 0x0404,
    Back = 0x0405,
    FrontAndBack = 0x0408,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlType {
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Float = 0x1406,
    Fixed = 0x140C,
    Int = 0x1404,
    UnsignedInt = 0x1405,
    HalfFloat = 0x140B,
    Double = 0x140A,
    // GL_INT_2_10_10_10_REV
    // GL_UNSIGNED_INT_2_10_10_10_REV
    // GL_UNSIGNED_INT_10F_11F_11F_REV
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexType {
    UnsignedByte = 0x1401,
    UnsignedShort = 0x1403,
    UnsignedInt = 0x1405,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerName {
    // Version 3.0
    MajorVersion  = 0x821B,
    MinorVersion  = 0x821C,
    NumExtensions = 0x821D,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Point = 0x1B00,
    Line = 0x1B01,
    Fill = 0x1B02,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramObject(u32);

impl ProgramObject {
    pub fn null() -> ProgramObject {
        ProgramObject(0)
    }

    pub fn is_null(&self) -> bool {
        *self == ProgramObject(0)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramParam {
    DeleteStatus = 0x8B80,
    LinkStatus = 0x8B82,
    ValidateStatus = 0x8B83,
    InfoLogLength = 0x8B84,
    AttachedShaders = 0x8B85,
    ActiveUniforms = 0x8B86,
    ActiveUniformMaxLength = 0x8B87,
    ActiveAttributes = 0x8B89,
    ActiveAttributeMaxLength = 0x8B8A,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShaderObject(u32);

impl ShaderObject {
    pub fn null() -> ShaderObject {
        ShaderObject(0)
    }

    pub fn is_null(&self) -> bool {
        *self == ShaderObject(0)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderParam {
    ShaderType = 0x8B4F,
    DeleteStatus = 0x8B80,
    CompileStatus = 0x8B81,
    InfoLogLength = 0x8B84,
    ShaderSourceLength = 0x8B88,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    Compute = 0x91B9,
    Fragment = 0x8B30,
    Vertex = 0x8B31,
    Geometry = 0x8DD9,
    TessEvaluation = 0x8E87,
    TessControl = 0x8E88,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UniformLocation(u32);

impl UniformLocation {
    pub fn from_index(index: u32) -> UniformLocation {
        UniformLocation(index)
    }
}

/// TODO: Use NonZero here so that Option<VertexArrayName>::None can be used instead of 0.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VertexArrayName(u32);

impl VertexArrayName {
    pub fn null() -> VertexArrayName {
        VertexArrayName(0)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindingOrder {
    Clockwise = 0x0900,
    CounterClockwise = 0x0901,
}

// =============== OLD UNSORTED TYPES ========================== //

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureObject(u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerCapability {
    Fog                   = 0x0B60,
    Lighting              = 0x0B50,
    Texture2D             = 0x0DE1,
    CullFace              = 0x0B44,
    AlphaTest             = 0x0BC0,
    Blend                 = 0x0BE2,
    ColorLogicOp          = 0x0BF2,
    Dither                = 0x0BD0,
    StencilTest           = 0x0B90,
    DepthTest             = 0x0B71,
    PointSmooth           = 0x0B10,
    LineSmooth            = 0x0B20,
    ScissorTest           = 0x0C11,
    ColorMaterial         = 0x0B57,
    Normalize             = 0x0BA1,
    RescaleNormal         = 0x803A,
    PolygonOffsetFill     = 0x8037,
    VertexArray           = 0x8074,
    NormalArray           = 0x8075,
    ColorArray            = 0x8076,
    TextureCoordArray     = 0x8078,
    Multisample           = 0x809D,
    SampleAlphaToCoverage = 0x809E,
    SampleAlphaToOne      = 0x809F,
    SampleCoverage        = 0x80A0,

    /// Introduced: OpenGL 4.3
    DebugOutput           = 0x92E0,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    NoError          = 0,
    InvalidEnum      = 0x0500,
    InvalidValue     = 0x0501,
    InvalidOperation = 0x0502,
    StackOverflow    = 0x0503,
    StackUnderflow   = 0x0504,
    OutOfMemory      = 0x0505,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureBindTarget {
    // GL_TEXTURE_1D,
    Texture2d = 0x0DE1,
    Texture3d = 0x806F,
    CubeMap   = 0x8513,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Texture2dTarget {
    Texture2d        = 0x0DE1,
    CubeMapPositiveX = 0x8515,
    CubeMapNegativeX = 0x8516,
    CubeMapPositiveY = 0x8517,
    CubeMapNegativeY = 0x8518,
    CubeMapPositiveZ = 0x8519,
    CubeMapNegativeZ = 0x851A,
    // GL_PROXY_TEXTURE_2D,
    // GL_PROXY_TEXTURE_CUBE_MAP,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureDataType {
    Byte          = 0x1400,
    UnsignedByte  = 0x1401,
    // GL_BITMAP,
    Short         = 0x1402,
    UnsignedShort = 0x1403,
    Int           = 0x1404,
    UnsignedInt   = 0x1405,
    Float         = 0x1406,
    // GL_UNSIGNED_BYTE_3_3_2,
    // GL_UNSIGNED_BYTE_2_3_3_REV,
    // GL_UNSIGNED_SHORT_5_6_5,
    // GL_UNSIGNED_SHORT_5_6_5_REV,
    // GL_UNSIGNED_SHORT_4_4_4_4,
    // GL_UNSIGNED_SHORT_4_4_4_4_REV,
    // GL_UNSIGNED_SHORT_5_5_5_1,
    // GL_UNSIGNED_SHORT_1_5_5_5_REV,
    // GL_UNSIGNED_INT_8_8_8_8,
    // GL_UNSIGNED_INT_8_8_8_8_REV,
    // GL_UNSIGNED_INT_10_10_10_2,
    // GL_UNSIGNED_INT_2_10_10_10_REV,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureInternalFormat {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Rgb = 0x1907,
    Rgba = 0x1908,
    // GL_ALPHA,
    // GL_ALPHA4,
    // GL_ALPHA8,
    // GL_ALPHA12,
    // GL_ALPHA16,
    // GL_COMPRESSED_ALPHA,
    // GL_COMPRESSED_LUMINANCE,
    // GL_COMPRESSED_LUMINANCE_ALPHA,
    // GL_COMPRESSED_INTENSITY,
    // GL_COMPRESSED_RGB,
    // GL_COMPRESSED_RGBA,
    // GL_DEPTH_COMPONENT,
    // GL_DEPTH_COMPONENT16,
    // GL_DEPTH_COMPONENT24,
    // GL_DEPTH_COMPONENT32,
    // GL_LUMINANCE,
    // GL_LUMINANCE4,
    // GL_LUMINANCE8,
    // GL_LUMINANCE12,
    // GL_LUMINANCE16,
    // GL_LUMINANCE_ALPHA,
    // GL_LUMINANCE4_ALPHA4,
    // GL_LUMINANCE6_ALPHA2,
    // GL_LUMINANCE8_ALPHA8,
    // GL_LUMINANCE12_ALPHA4,
    // GL_LUMINANCE12_ALPHA12,
    // GL_LUMINANCE16_ALPHA16,
    // GL_INTENSITY,
    // GL_INTENSITY4,
    // GL_INTENSITY8,
    // GL_INTENSITY12,
    // GL_INTENSITY16,
    // GL_R3_G3_B2,
    // GL_RGB4,
    // GL_RGB5,
    // GL_RGB8,
    // GL_RGB10,
    // GL_RGB12,
    // GL_RGB16,
    // GL_RGBA2,
    // GL_RGBA4,
    // GL_RGB5_A1,
    // GL_RGBA8,
    // GL_RGB10_A2,
    // GL_RGBA12,
    // GL_RGBA16,
    // GL_SLUMINANCE,
    // GL_SLUMINANCE8,
    // GL_SLUMINANCE_ALPHA,
    // GL_SLUMINANCE8_ALPHA8,
    // GL_SRGB,
    // GL_SRGB8,
    // GL_SRGB_ALPHA,
    // GL_SRGB8_ALPHA8,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    Rgb  = 0x1907,
    Rgba = 0x1908,
    Bgr  = 0x80E0,
    Bgra = 0x80E1,
    // GL_COLOR_INDEX,
    // GL_RED,
    // GL_GREEN,
    // GL_BLUE,
    // GL_ALPHA,
    // GL_LUMINANCE,
    // GL_LUMINANCE_ALPHA,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestFactor {
    Zero             = 0,
    One              = 1,
    SrcColor         = 0x0300,
    OneMinusSrcColor = 0x0301,
    SrcAlpha         = 0x0302,
    OneMinusSrcAlpha = 0x0303,
    DstAlpha         = 0x0304,
    OneMinusDstAlpha = 0x0305,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceFactor {
    Zero             = 0,
    One              = 1,
    SrcColor         = 0x0300,
    OneMinusSrcColor = 0x0301,
    SrcAlpha         = 0x0302,
    OneMinusSrcAlpha = 0x0303,
    DstAlpha         = 0x0304,
    OneMinusDstAlpha = 0x0305,
    DstColor         = 0x0306,
    OneMinusDstColor = 0x0307,
    SrcAlphaSaturate = 0x0308,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringName {
    Vendor                 = 0x1F00,
    Renderer               = 0x1F01,
    Version                = 0x1F02,
    ShadingLanguageVersion = 0x8B8C,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugSeverity {
    High   = 0x9146,
    Medium = 0x9147,
    Low    = 0x9148,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugSource {
    API            = 0x8246,
    WindowSystem   = 0x8247,
    ShaderCompiler = 0x8248,
    ThirdParty     = 0x8249,
    Application    = 0x824A,
    Other          = 0x824B,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugType {
    Error              = 0x824C,
    DeprecatedBehavior = 0x824D,
    UndefinedBehavior  = 0x824E,
    Portability        = 0x824F,
    Performance        = 0x8250,
    Other              = 0x8251,
    Marker             = 0x8268,
    PushGroup          = 0x8269,
    PopGroup           = 0x826A,
}

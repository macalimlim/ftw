#![allow(clippy::upper_case_acronyms)]

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

macro_rules! generate_ftw_node_types {
    ($($i:ident), *) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FtwNodeType {
            $($i,)*
        }

        impl FromStr for FtwNodeType {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($i) => Ok(FtwNodeType::$i),)*
                    _ => Ok(FtwNodeType::Node),
                }
            }
        }

        impl Display for FtwNodeType {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let node_type: &str = match self {
                    $(FtwNodeType::$i => stringify!($i),)*
                };
                write!(f, "{}", node_type)
            }
        }

        impl Default for FtwNodeType {
            fn default() -> Self {
                FtwNodeType::Node
            }
        }

        #[cfg(test)]
        mod ftw_node_type_tests {
            use super::*;
            use proptest::prelude::{prop_assert, prop_assert_eq, prop_assume, proptest};

            #[test]
            fn test_from_str() -> Result<(), ()> {
                $(assert_eq!(stringify!($i).parse::<FtwNodeType>()?, FtwNodeType::$i);)*
                Ok(())
            }

            #[test]
            fn test_fmt() {
                $(assert_eq!(stringify!($i), format!("{}", FtwNodeType::$i));)*
            }

            #[test]
            fn test_default() {
                assert_eq!(FtwNodeType::default(), FtwNodeType::Node);
            }

            proptest! {
                #[test]
                fn test_from_str_invalid_input(node_type_input in "\\PC*") {
                    $(prop_assume!(node_type_input != stringify!($i));)*
                    let result = node_type_input.parse::<FtwNodeType>();
                    prop_assert!(result.is_ok());
                    prop_assert_eq!(result.unwrap(), FtwNodeType::Node);
                }
            }
        }
    };
}

generate_ftw_node_types![
    AcceptDialog,
    AnimatedSprite,
    AnimatedSprite3D,
    AnimatedTexture,
    Animation,
    AnimationNode,
    AnimationNodeAdd2,
    AnimationNodeAdd3,
    AnimationNodeAnimation,
    AnimationNodeBlend2,
    AnimationNodeBlend3,
    AnimationNodeBlendSpace1D,
    AnimationNodeBlendSpace2D,
    AnimationNodeBlendTree,
    AnimationNodeOneShot,
    AnimationNodeOutput,
    AnimationNodeStateMachine,
    AnimationNodeStateMachinePlayback,
    AnimationNodeStateMachineTransition,
    AnimationNodeTimeScale,
    AnimationNodeTimeSeek,
    AnimationNodeTransition,
    AnimationPlayer,
    AnimationRootNode,
    AnimationTrackEditPlugin,
    AnimationTree,
    AnimationTreePlayer,
    Area,
    Area2D,
    ArrayMesh,
    ARVRAnchor,
    ARVRCamera,
    ARVRController,
    ARVRInterface,
    ARVRInterfaceGDNative,
    ARVROrigin,
    ARVRPositionalTracker,
    ARVRServer,
    AStar,
    AStar2D,
    AtlasTexture,
    AudioBusLayout,
    AudioEffect,
    AudioEffectAmplify,
    AudioEffectBandLimitFilter,
    AudioEffectBandPassFilter,
    AudioEffectChorus,
    AudioEffectCompressor,
    AudioEffectDelay,
    AudioEffectDistortion,
    AudioEffectEQ,
    AudioEffectEQ10,
    AudioEffectEQ21,
    AudioEffectEQ6,
    AudioEffectFilter,
    AudioEffectHighPassFilter,
    AudioEffectHighShelfFilter,
    AudioEffectInstance,
    AudioEffectLimiter,
    AudioEffectLowPassFilter,
    AudioEffectLowShelfFilter,
    AudioEffectNotchFilter,
    AudioEffectPanner,
    AudioEffectPhaser,
    AudioEffectPitchShift,
    AudioEffectRecord,
    AudioEffectReverb,
    AudioEffectSpectrumAnalyzer,
    AudioEffectSpectrumAnalyzerInstance,
    AudioEffectStereoEnhance,
    AudioServer,
    AudioStream,
    AudioStreamGenerator,
    AudioStreamGeneratorPlayback,
    AudioStreamMicrophone,
    AudioStreamOGGVorbis,
    AudioStreamPlayback,
    AudioStreamPlaybackResampled,
    AudioStreamPlayer,
    AudioStreamPlayer2D,
    AudioStreamPlayer3D,
    AudioStreamRandomPitch,
    AudioStreamSample,
    BackBufferCopy,
    BakedLightmap,
    BakedLightmapData,
    BaseButton,
    BitMap,
    BitmapFont,
    Bone2D,
    BoneAttachment,
    BoxContainer,
    BoxShape,
    BulletPhysicsDirectBodyState,
    BulletPhysicsServer,
    Button,
    ButtonGroup,
    Camera,
    Camera2D,
    CameraFeed,
    CameraServer,
    CameraTexture,
    CanvasItem,
    CanvasItemMaterial,
    CanvasLayer,
    CanvasModulate,
    CapsuleMesh,
    CapsuleShape,
    CapsuleShape2D,
    CenterContainer,
    CharFXTransform,
    CheckBox,
    CheckButton,
    CircleShape2D,
    ClassDB,
    ClippedCamera,
    CollisionObject,
    CollisionObject2D,
    CollisionPolygon,
    CollisionPolygon2D,
    CollisionShape,
    CollisionShape2D,
    ColorPicker,
    ColorPickerButton,
    ColorRect,
    ConcavePolygonShape,
    ConcavePolygonShape2D,
    ConeTwistJoint,
    ConfigFile,
    ConfirmationDialog,
    Container,
    Control,
    ConvexPolygonShape,
    ConvexPolygonShape2D,
    CPUParticles,
    CPUParticles2D,
    Crypto,
    CryptoKey,
    CSGBox,
    CSGCombiner,
    CSGCylinder,
    CSGMesh,
    CSGPolygon,
    CSGPrimitive,
    CSGShape,
    CSGSphere,
    CSGTorus,
    CubeMap,
    CubeMesh,
    Curve,
    Curve2D,
    Curve3D,
    CurveTexture,
    CylinderMesh,
    CylinderShape,
    DampedSpringJoint2D,
    DirectionalLight,
    Directory,
    DTLSServer,
    DynamicFont,
    DynamicFontData,
    EditorExportPlugin,
    EditorFeatureProfile,
    EditorFileDialog,
    EditorFileSystem,
    EditorFileSystemDirectory,
    EditorImportPlugin,
    EditorInspector,
    EditorInspectorPlugin,
    EditorInterface,
    EditorNavigationMeshGenerator,
    EditorPlugin,
    EditorProperty,
    EditorResourceConversionPlugin,
    EditorResourcePreview,
    EditorResourcePreviewGenerator,
    EditorSceneImporter,
    EditorSceneImporterAssimp,
    EditorScenePostImport,
    EditorScript,
    EditorSelection,
    EditorSettings,
    EditorSpatialGizmo,
    EditorSpatialGizmoPlugin,
    EditorSpinSlider,
    EditorVCSInterface,
    EncodedObjectAsID,
    Engine,
    Environment,
    Expression,
    ExternalTexture,
    File,
    FileDialog,
    FileSystemDock,
    Font,
    FuncRef,
    GDNative,
    GDNativeLibrary,
    GDScript,
    GDScriptFunctionState,
    Generic6DOFJoint,
    Geometry,
    GeometryInstance,
    GIProbe,
    GIProbeData,
    GlobalConstants,
    Gradient,
    GradientTexture,
    GraphEdit,
    GraphNode,
    GridContainer,
    GridMap,
    GrooveJoint2D,
    HashingContext,
    HBoxContainer,
    HeightMapShape,
    HingeJoint,
    HScrollBar,
    HSeparator,
    HSlider,
    HSplitContainer,
    HTTPClient,
    HTTPRequest,
    Image,
    ImageTexture,
    ImmediateGeometry,
    Input,
    InputDefault,
    InputEvent,
    InputEventAction,
    InputEventGesture,
    InputEventJoypadButton,
    InputEventJoypadMotion,
    InputEventKey,
    InputEventMagnifyGesture,
    InputEventMIDI,
    InputEventMouse,
    InputEventMouseButton,
    InputEventMouseMotion,
    InputEventPanGesture,
    InputEventScreenDrag,
    InputEventScreenTouch,
    InputEventWithModifiers,
    InputMap,
    InstancePlaceholder,
    InterpolatedCamera,
    IP,
    IPUnix,
    ItemList,
    JavaClass,
    JavaClassWrapper,
    JavaScript,
    JNISingleton,
    Joint,
    Joint2D,
    JSON,
    JSONParseResult,
    JSONRPC,
    KinematicBody,
    KinematicBody2D,
    KinematicCollision,
    KinematicCollision2D,
    Label,
    LargeTexture,
    Light,
    Light2D,
    LightOccluder2D,
    Line2D,
    LineEdit,
    LineShape2D,
    LinkButton,
    Listener,
    MainLoop,
    MarginContainer,
    Marshalls,
    Material,
    MenuButton,
    Mesh,
    MeshDataTool,
    MeshInstance,
    MeshInstance2D,
    MeshLibrary,
    MeshTexture,
    MobileVRInterface,
    MultiMesh,
    MultiMeshInstance,
    MultiMeshInstance2D,
    MultiplayerAPI,
    MultiplayerPeerGDNative,
    Mutex,
    NativeScript,
    Navigation,
    Navigation2D,
    NavigationMesh,
    NavigationMeshInstance,
    NavigationPolygon,
    NavigationPolygonInstance,
    NetworkedMultiplayerENet,
    NetworkedMultiplayerPeer,
    NinePatchRect,
    Node,
    Node2D,
    NoiseTexture,
    Object,
    OccluderPolygon2D,
    OmniLight,
    OpenSimplexNoise,
    OptionButton,
    OS,
    PackedDataContainer,
    PackedDataContainerRef,
    PackedScene,
    PacketPeer,
    PacketPeerDTLS,
    PacketPeerGDNative,
    PacketPeerStream,
    PacketPeerUDP,
    Panel,
    PanelContainer,
    PanoramaSky,
    ParallaxBackground,
    ParallaxLayer,
    Particles,
    Particles2D,
    ParticlesMaterial,
    Path,
    Path2D,
    PathFollow,
    PathFollow2D,
    PCKPacker,
    Performance,
    PHashTranslation,
    PhysicalBone,
    Physics2DDirectBodyState,
    Physics2DDirectBodyStateSW,
    Physics2DDirectSpaceState,
    Physics2DServer,
    Physics2DServerSW,
    Physics2DShapeQueryParameters,
    Physics2DShapeQueryResult,
    Physics2DTestMotionResult,
    PhysicsBody,
    PhysicsBody2D,
    PhysicsDirectBodyState,
    PhysicsDirectSpaceState,
    PhysicsMaterial,
    PhysicsServer,
    PhysicsShapeQueryParameters,
    PhysicsShapeQueryResult,
    PinJoint,
    PinJoint2D,
    PlaneMesh,
    PlaneShape,
    PluginScript,
    PointMesh,
    Polygon2D,
    PolygonPathFinder,
    Popup,
    PopupDialog,
    PopupMenu,
    PopupPanel,
    Position2D,
    Position3D,
    PrimitiveMesh,
    PrismMesh,
    ProceduralSky,
    ProgressBar,
    ProjectSettings,
    ProximityGroup,
    ProxyTexture,
    QuadMesh,
    RandomNumberGenerator,
    Range,
    RayCast,
    RayCast2D,
    RayShape,
    RayShape2D,
    RectangleShape2D,
    Reference,
    ReferenceRect,
    ReflectionProbe,
    RegEx,
    RegExMatch,
    RemoteTransform,
    RemoteTransform2D,
    Resource,
    ResourceFormatLoader,
    ResourceFormatSaver,
    ResourceImporter,
    ResourceInteractiveLoader,
    ResourceLoader,
    ResourcePreloader,
    ResourceSaver,
    RichTextEffect,
    RichTextLabel,
    RigidBody,
    RigidBody2D,
    RootMotionView,
    SceneState,
    SceneTree,
    SceneTreeTimer,
    Script,
    ScriptCreateDialog,
    ScriptEditor,
    ScrollBar,
    ScrollContainer,
    SegmentShape2D,
    Semaphore,
    Separator,
    Shader,
    ShaderMaterial,
    Shape,
    Shape2D,
    ShortCut,
    Skeleton,
    Skeleton2D,
    SkeletonIK,
    Skin,
    SkinReference,
    Sky,
    Slider,
    SliderJoint,
    SoftBody,
    Spatial,
    SpatialGizmo,
    SpatialMaterial,
    SpatialVelocityTracker,
    SphereMesh,
    SphereShape,
    SpinBox,
    SplitContainer,
    SpotLight,
    SpringArm,
    Sprite,
    Sprite3D,
    SpriteBase3D,
    SpriteFrames,
    StaticBody,
    StaticBody2D,
    StreamPeer,
    StreamPeerBuffer,
    StreamPeerGDNative,
    StreamPeerSSL,
    StreamPeerTCP,
    StreamTexture,
    StyleBox,
    StyleBoxEmpty,
    StyleBoxFlat,
    StyleBoxLine,
    StyleBoxTexture,
    SurfaceTool,
    TabContainer,
    Tabs,
    TCPServer,
    TextEdit,
    TextFile,
    Texture,
    Texture3D,
    TextureArray,
    TextureButton,
    TextureLayered,
    TextureProgress,
    TextureRect,
    Theme,
    Thread,
    TileMap,
    TileSet,
    Timer,
    ToolButton,
    TouchScreenButton,
    Translation,
    TranslationServer,
    Tree,
    TreeItem,
    TriangleMesh,
    Tween,
    UDPServer,
    UndoRedo,
    UPNP,
    UPNPDevice,
    VBoxContainer,
    VehicleBody,
    VehicleWheel,
    VideoPlayer,
    VideoStream,
    VideoStreamGDNative,
    VideoStreamTheora,
    VideoStreamWebm,
    Viewport,
    ViewportContainer,
    ViewportTexture,
    VisibilityEnabler,
    VisibilityEnabler2D,
    VisibilityNotifier,
    VisibilityNotifier2D,
    VisualInstance,
    VisualScript,
    VisualScriptBasicTypeConstant,
    VisualScriptBuiltinFunc,
    VisualScriptClassConstant,
    VisualScriptComment,
    VisualScriptComposeArray,
    VisualScriptCondition,
    VisualScriptConstant,
    VisualScriptConstructor,
    VisualScriptCustomNode,
    VisualScriptDeconstruct,
    VisualScriptEditor,
    VisualScriptEmitSignal,
    VisualScriptEngineSingleton,
    VisualScriptExpression,
    VisualScriptFunction,
    VisualScriptFunctionCall,
    VisualScriptFunctionState,
    VisualScriptGlobalConstant,
    VisualScriptIndexGet,
    VisualScriptIndexSet,
    VisualScriptInputAction,
    VisualScriptIterator,
    VisualScriptLists,
    VisualScriptLocalVar,
    VisualScriptLocalVarSet,
    VisualScriptMathConstant,
    VisualScriptNode,
    VisualScriptOperator,
    VisualScriptPreload,
    VisualScriptPropertyGet,
    VisualScriptPropertySet,
    VisualScriptResourcePath,
    VisualScriptReturn,
    VisualScriptSceneNode,
    VisualScriptSceneTree,
    VisualScriptSelect,
    VisualScriptSelf,
    VisualScriptSequence,
    VisualScriptSubCall,
    VisualScriptSwitch,
    VisualScriptTypeCast,
    VisualScriptVariableGet,
    VisualScriptVariableSet,
    VisualScriptWhile,
    VisualScriptYield,
    VisualScriptYieldSignal,
    VisualServer,
    VisualShader,
    VisualShaderNode,
    VisualShaderNodeBooleanConstant,
    VisualShaderNodeBooleanUniform,
    VisualShaderNodeColorConstant,
    VisualShaderNodeColorFunc,
    VisualShaderNodeColorOp,
    VisualShaderNodeColorUniform,
    VisualShaderNodeCompare,
    VisualShaderNodeCubeMap,
    VisualShaderNodeCubeMapUniform,
    VisualShaderNodeCustom,
    VisualShaderNodeDeterminant,
    VisualShaderNodeDotProduct,
    VisualShaderNodeExpression,
    VisualShaderNodeFaceForward,
    VisualShaderNodeFresnel,
    VisualShaderNodeGlobalExpression,
    VisualShaderNodeGroupBase,
    VisualShaderNodeIf,
    VisualShaderNodeInput,
    VisualShaderNodeIs,
    VisualShaderNodeOuterProduct,
    VisualShaderNodeOutput,
    VisualShaderNodeScalarClamp,
    VisualShaderNodeScalarConstant,
    VisualShaderNodeScalarDerivativeFunc,
    VisualShaderNodeScalarFunc,
    VisualShaderNodeScalarInterp,
    VisualShaderNodeScalarOp,
    VisualShaderNodeScalarSmoothStep,
    VisualShaderNodeScalarSwitch,
    VisualShaderNodeScalarUniform,
    VisualShaderNodeSwitch,
    VisualShaderNodeTexture,
    VisualShaderNodeTextureUniform,
    VisualShaderNodeTextureUniformTriplanar,
    VisualShaderNodeTransformCompose,
    VisualShaderNodeTransformConstant,
    VisualShaderNodeTransformDecompose,
    VisualShaderNodeTransformFunc,
    VisualShaderNodeTransformMult,
    VisualShaderNodeTransformUniform,
    VisualShaderNodeTransformVecMult,
    VisualShaderNodeUniform,
    VisualShaderNodeVec3Constant,
    VisualShaderNodeVec3Uniform,
    VisualShaderNodeVectorClamp,
    VisualShaderNodeVectorCompose,
    VisualShaderNodeVectorDecompose,
    VisualShaderNodeVectorDerivativeFunc,
    VisualShaderNodeVectorDistance,
    VisualShaderNodeVectorFunc,
    VisualShaderNodeVectorInterp,
    VisualShaderNodeVectorLen,
    VisualShaderNodeVectorOp,
    VisualShaderNodeVectorRefract,
    VisualShaderNodeVectorScalarMix,
    VisualShaderNodeVectorScalarSmoothStep,
    VisualShaderNodeVectorScalarStep,
    VisualShaderNodeVectorSmoothStep,
    VScrollBar,
    VSeparator,
    VSlider,
    VSplitContainer,
    WeakRef,
    WebRTCDataChannel,
    WebRTCDataChannelGDNative,
    WebRTCMultiplayer,
    WebRTCPeerConnection,
    WebRTCPeerConnectionGDNative,
    WebSocketClient,
    WebSocketMultiplayerPeer,
    WebSocketPeer,
    WebSocketServer,
    WindowDialog,
    World,
    World2D,
    WorldEnvironment,
    X509Certificate,
    XMLParser,
    YSort
];

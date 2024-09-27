// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditorBody {
    ///
    #[prost(int32, tag = "1")]
    pub version: i32,
    ///
    #[prost(message, repeated, tag = "2")]
    pub materials: ::prost::alloc::vec::Vec<Material>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub features: ::prost::alloc::vec::Vec<Feature>,
    ///
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<Metadata>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    ///
    #[prost(enumeration = "FeatureFlag", tag = "1")]
    pub flag: i32,
    ///
    #[prost(string, tag = "2")]
    pub ext: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FeatureHighlightExt {
    ///
    #[prost(int32, tag = "1")]
    pub count: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureVtuberExt {
    ///
    #[prost(int32, repeated, tag = "1")]
    pub exp: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(int32, repeated, tag = "2")]
    pub action: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(int32, repeated, tag = "3")]
    pub combine: ::prost::alloc::vec::Vec<i32>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Material {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(enumeration = "MaterialType", tag = "2")]
    pub r#type: i32,
    ///
    #[prost(enumeration = "MaterialForm", tag = "3")]
    pub from: i32,
    ///
    #[prost(int32, tag = "4")]
    pub duration: i32,
    ///
    #[prost(string, tag = "5")]
    pub ext: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    ///
    #[prost(int32, tag = "1")]
    pub duration: i32,
    ///
    #[prost(int32, tag = "2")]
    pub pic_count: i32,
    ///
    #[prost(int32, tag = "3")]
    pub video_count: i32,
    ///
    #[prost(int32, tag = "12")]
    pub video_tracks: i32,
    ///
    #[prost(int32, tag = "13")]
    pub audio_tracks: i32,
    ///
    #[prost(int32, tag = "19")]
    pub story_id: i32,
    ///
    #[prost(int64, tag = "20")]
    pub draft_id: i64,
    ///
    #[prost(string, tag = "21")]
    pub draft_key: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "22")]
    pub local_material_count: i32,
    ///
    #[prost(string, tag = "23")]
    pub sdk_type: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FeatureFlag {
    ///
    FfNone = 0,
    ///
    LinearMask = 1,
    ///
    MirrorMask = 2,
    ///
    RectangularMask = 3,
    ///
    RecognitionTitle = 21,
    ///
    IdentificationRecording = 22,
    ///
    MyMaterialLibrary = 23,
    ///
    TextReading = 24,
    ///
    EditorHighlightRecognition = 25,
    ///
    LinkExtractMusic = 26,
    ///
    BatchRoughShears = 27,
    ///
    MaterialCenter = 28,
    ///
    CommandBarrage = 29,
    ///
    CoverMattingFunction = 30,
    ///
    VideoExtractMusic = 31,
    ///
    LocalMusic = 32,
    ///
    DynamicFeatures = 33,
    ///
    VideoDotFunction = 34,
    ///
    DynamicTracking = 35,
    ///
    MusicalSpeedChange = 36,
    ///
    SoundSpeedChange = 37,
    ///
    TextReadingSpeedChange = 38,
    ///
    RecordingSpeedChange = 39,
    ///
    AvatarsCustomizeParts = 40,
    ///
    CropCustomizeStickers = 41,
    ///
    MattingCustomStickers = 42,
    ///
    VtuberFeature = 43,
    ///
    Matting = 44,
    ///
    Vtuber = 45,
    ///
    Adjust = 46,
    ///
    KichikuRap = 47,
    ///
    PrefabricatedDraft = 48,
    ///
    CircularMask = 49,
    ///
    KeyFrame = 50,
    ///
    VoiceChanger = 51,
    ///
    LolMaterial = 52,
    ///
    AudioSeparation = 53,
    ///
    QuickVtuberMv = 54,
    ///
    VideoInAnimation = 55,
    ///
    VideoOutAnimation = 56,
    ///
    VideoCompositeAnimation = 57,
    ///
    RecommendedTemplateCover = 58,
    ///
    CoverMakesRecommended = 59,
    ///
    GraphicTemplate = 61,
    ///
    HeadMatting = 63,
    ///
    PortraitMatting = 64,
    ///
    CustomMatting = 65,
    ///
    TitleGhostAnimal = 66,
    ///
    PlayVideoBackwards = 67,
    ///
    DraftStorylineGenerated = 70,
    ///
    VtuberRecording = 71,
    ///
    OneClickPacking = 72,
    ///
    DubbingCreation16 = 73,
    ///
    DubbingCreation3 = 74,
    ///
    PictureAudioRecording = 76,
    ///
    Play4D = 77,
    ///
    RecommendedMusic = 78,
    ///
    GraphicIntelligentMapping = 79,
    ///
    Recording = 80,
    ///
    Shoot = 81,
    ///
    VideoSpeedChange = 82,
    ///
    CameraFlip = 83,
    ///
    ScreenRecording = 84,
    ///
    DefaultEnd = 85,
    ///
    WhetherGameHighlights = 86,
    ///
    RecordingInscription = 87,
    ///
    PictureInPicture = 88,
    ///
    VoiceQuickClip = 89,
    ///
    OneKeyMakeVideo = 90,
    ///
    SmartMakeVideo = 91,
    ///
    ShootSpeedChange = 92,
    ///
    NoiseReduction = 93,
    ///
    AiImage2Image = 94,
    ///
    KichikuScene = 95,
    ///
    ShootBeauty = 100,
    ///
    ShootBeautyTemplate = 101,
    ///
    ShootCountdown = 102,
    ///
    ShootFlashlight = 103,
    ///
    IntellectTitle = 104,
    ///
    AiKeywordExtra = 105,
    ///
    PictureTagRecognition = 106,
    ///
    IntellectCover = 107,
    ///
    AiActivity = 108,
    ///
    DigitalHuman = 109,
    ///
    AiStraightTitle = 110,
    ///
    GameFactory = 111,
    ///
    NonSmartVideo = 112,
    ///
    Replace = 113,
    ///
    TtsClone = 114,
    ///
    SmartTitlePink = 115,
    ///
    RecognitionMusic = 116,
}
impl FeatureFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::FfNone => "_ff_none",
            Self::LinearMask => "LinearMask",
            Self::MirrorMask => "MirrorMask",
            Self::RectangularMask => "RectangularMask",
            Self::RecognitionTitle => "RecognitionTitle",
            Self::IdentificationRecording => "IdentificationRecording",
            Self::MyMaterialLibrary => "MyMaterialLibrary",
            Self::TextReading => "TextReading",
            Self::EditorHighlightRecognition => "EditorHighlightRecognition",
            Self::LinkExtractMusic => "LinkExtractMusic",
            Self::BatchRoughShears => "BatchRoughShears",
            Self::MaterialCenter => "MaterialCenter",
            Self::CommandBarrage => "CommandBarrage",
            Self::CoverMattingFunction => "CoverMattingFunction",
            Self::VideoExtractMusic => "VideoExtractMusic",
            Self::LocalMusic => "LocalMusic",
            Self::DynamicFeatures => "DynamicFeatures",
            Self::VideoDotFunction => "VideoDotFunction",
            Self::DynamicTracking => "DynamicTracking",
            Self::MusicalSpeedChange => "MusicalSpeedChange",
            Self::SoundSpeedChange => "SoundSpeedChange",
            Self::TextReadingSpeedChange => "TextReadingSpeedChange",
            Self::RecordingSpeedChange => "RecordingSpeedChange",
            Self::AvatarsCustomizeParts => "AvatarsCustomizeParts",
            Self::CropCustomizeStickers => "CropCustomizeStickers",
            Self::MattingCustomStickers => "MattingCustomStickers",
            Self::VtuberFeature => "VtuberFeature",
            Self::Matting => "Matting",
            Self::Vtuber => "Vtuber",
            Self::Adjust => "Adjust",
            Self::KichikuRap => "KichikuRap",
            Self::PrefabricatedDraft => "PrefabricatedDraft",
            Self::CircularMask => "CircularMask",
            Self::KeyFrame => "KeyFrame",
            Self::VoiceChanger => "VoiceChanger",
            Self::LolMaterial => "LOLMaterial",
            Self::AudioSeparation => "AudioSeparation",
            Self::QuickVtuberMv => "QuickVtuberMV",
            Self::VideoInAnimation => "VideoInAnimation",
            Self::VideoOutAnimation => "VideoOutAnimation",
            Self::VideoCompositeAnimation => "VideoCompositeAnimation",
            Self::RecommendedTemplateCover => "recommendedTemplateCover",
            Self::CoverMakesRecommended => "CoverMakesRecommended",
            Self::GraphicTemplate => "GraphicTemplate",
            Self::HeadMatting => "HeadMatting",
            Self::PortraitMatting => "PortraitMatting",
            Self::CustomMatting => "CustomMatting",
            Self::TitleGhostAnimal => "TitleGhostAnimal",
            Self::PlayVideoBackwards => "PlayVideoBackwards",
            Self::DraftStorylineGenerated => "DraftStorylineGenerated",
            Self::VtuberRecording => "VtuberRecording",
            Self::OneClickPacking => "OneClickPacking",
            Self::DubbingCreation16 => "DubbingCreation1_6",
            Self::DubbingCreation3 => "DubbingCreation3",
            Self::PictureAudioRecording => "PictureAudioRecording",
            Self::Play4D => "Play4D",
            Self::RecommendedMusic => "RecommendedMusic",
            Self::GraphicIntelligentMapping => "GraphicIntelligentMapping",
            Self::Recording => "Recording",
            Self::Shoot => "Shoot",
            Self::VideoSpeedChange => "VideoSpeedChange",
            Self::CameraFlip => "CameraFlip",
            Self::ScreenRecording => "ScreenRecording",
            Self::DefaultEnd => "DefaultEnd",
            Self::WhetherGameHighlights => "WhetherGameHighlights",
            Self::RecordingInscription => "RecordingInscription",
            Self::PictureInPicture => "PictureInPicture",
            Self::VoiceQuickClip => "VoiceQuickClip",
            Self::OneKeyMakeVideo => "OneKeyMakeVideo",
            Self::SmartMakeVideo => "SmartMakeVideo",
            Self::ShootSpeedChange => "ShootSpeedChange",
            Self::NoiseReduction => "NoiseReduction",
            Self::AiImage2Image => "AIImage2Image",
            Self::KichikuScene => "KichikuScene",
            Self::ShootBeauty => "ShootBeauty",
            Self::ShootBeautyTemplate => "ShootBeautyTemplate",
            Self::ShootCountdown => "ShootCountdown",
            Self::ShootFlashlight => "ShootFlashlight",
            Self::IntellectTitle => "IntellectTitle",
            Self::AiKeywordExtra => "AiKeywordExtra",
            Self::PictureTagRecognition => "PictureTagRecognition",
            Self::IntellectCover => "IntellectCover",
            Self::AiActivity => "AIActivity",
            Self::DigitalHuman => "DigitalHuman",
            Self::AiStraightTitle => "AIStraightTitle",
            Self::GameFactory => "GameFactory",
            Self::NonSmartVideo => "NonSmartVideo",
            Self::Replace => "Replace",
            Self::TtsClone => "TTSClone",
            Self::SmartTitlePink => "SmartTitlePink",
            Self::RecognitionMusic => "RecognitionMusic",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "_ff_none" => Some(Self::FfNone),
            "LinearMask" => Some(Self::LinearMask),
            "MirrorMask" => Some(Self::MirrorMask),
            "RectangularMask" => Some(Self::RectangularMask),
            "RecognitionTitle" => Some(Self::RecognitionTitle),
            "IdentificationRecording" => Some(Self::IdentificationRecording),
            "MyMaterialLibrary" => Some(Self::MyMaterialLibrary),
            "TextReading" => Some(Self::TextReading),
            "EditorHighlightRecognition" => Some(Self::EditorHighlightRecognition),
            "LinkExtractMusic" => Some(Self::LinkExtractMusic),
            "BatchRoughShears" => Some(Self::BatchRoughShears),
            "MaterialCenter" => Some(Self::MaterialCenter),
            "CommandBarrage" => Some(Self::CommandBarrage),
            "CoverMattingFunction" => Some(Self::CoverMattingFunction),
            "VideoExtractMusic" => Some(Self::VideoExtractMusic),
            "LocalMusic" => Some(Self::LocalMusic),
            "DynamicFeatures" => Some(Self::DynamicFeatures),
            "VideoDotFunction" => Some(Self::VideoDotFunction),
            "DynamicTracking" => Some(Self::DynamicTracking),
            "MusicalSpeedChange" => Some(Self::MusicalSpeedChange),
            "SoundSpeedChange" => Some(Self::SoundSpeedChange),
            "TextReadingSpeedChange" => Some(Self::TextReadingSpeedChange),
            "RecordingSpeedChange" => Some(Self::RecordingSpeedChange),
            "AvatarsCustomizeParts" => Some(Self::AvatarsCustomizeParts),
            "CropCustomizeStickers" => Some(Self::CropCustomizeStickers),
            "MattingCustomStickers" => Some(Self::MattingCustomStickers),
            "VtuberFeature" => Some(Self::VtuberFeature),
            "Matting" => Some(Self::Matting),
            "Vtuber" => Some(Self::Vtuber),
            "Adjust" => Some(Self::Adjust),
            "KichikuRap" => Some(Self::KichikuRap),
            "PrefabricatedDraft" => Some(Self::PrefabricatedDraft),
            "CircularMask" => Some(Self::CircularMask),
            "KeyFrame" => Some(Self::KeyFrame),
            "VoiceChanger" => Some(Self::VoiceChanger),
            "LOLMaterial" => Some(Self::LolMaterial),
            "AudioSeparation" => Some(Self::AudioSeparation),
            "QuickVtuberMV" => Some(Self::QuickVtuberMv),
            "VideoInAnimation" => Some(Self::VideoInAnimation),
            "VideoOutAnimation" => Some(Self::VideoOutAnimation),
            "VideoCompositeAnimation" => Some(Self::VideoCompositeAnimation),
            "recommendedTemplateCover" => Some(Self::RecommendedTemplateCover),
            "CoverMakesRecommended" => Some(Self::CoverMakesRecommended),
            "GraphicTemplate" => Some(Self::GraphicTemplate),
            "HeadMatting" => Some(Self::HeadMatting),
            "PortraitMatting" => Some(Self::PortraitMatting),
            "CustomMatting" => Some(Self::CustomMatting),
            "TitleGhostAnimal" => Some(Self::TitleGhostAnimal),
            "PlayVideoBackwards" => Some(Self::PlayVideoBackwards),
            "DraftStorylineGenerated" => Some(Self::DraftStorylineGenerated),
            "VtuberRecording" => Some(Self::VtuberRecording),
            "OneClickPacking" => Some(Self::OneClickPacking),
            "DubbingCreation1_6" => Some(Self::DubbingCreation16),
            "DubbingCreation3" => Some(Self::DubbingCreation3),
            "PictureAudioRecording" => Some(Self::PictureAudioRecording),
            "Play4D" => Some(Self::Play4D),
            "RecommendedMusic" => Some(Self::RecommendedMusic),
            "GraphicIntelligentMapping" => Some(Self::GraphicIntelligentMapping),
            "Recording" => Some(Self::Recording),
            "Shoot" => Some(Self::Shoot),
            "VideoSpeedChange" => Some(Self::VideoSpeedChange),
            "CameraFlip" => Some(Self::CameraFlip),
            "ScreenRecording" => Some(Self::ScreenRecording),
            "DefaultEnd" => Some(Self::DefaultEnd),
            "WhetherGameHighlights" => Some(Self::WhetherGameHighlights),
            "RecordingInscription" => Some(Self::RecordingInscription),
            "PictureInPicture" => Some(Self::PictureInPicture),
            "VoiceQuickClip" => Some(Self::VoiceQuickClip),
            "OneKeyMakeVideo" => Some(Self::OneKeyMakeVideo),
            "SmartMakeVideo" => Some(Self::SmartMakeVideo),
            "ShootSpeedChange" => Some(Self::ShootSpeedChange),
            "NoiseReduction" => Some(Self::NoiseReduction),
            "AIImage2Image" => Some(Self::AiImage2Image),
            "KichikuScene" => Some(Self::KichikuScene),
            "ShootBeauty" => Some(Self::ShootBeauty),
            "ShootBeautyTemplate" => Some(Self::ShootBeautyTemplate),
            "ShootCountdown" => Some(Self::ShootCountdown),
            "ShootFlashlight" => Some(Self::ShootFlashlight),
            "IntellectTitle" => Some(Self::IntellectTitle),
            "AiKeywordExtra" => Some(Self::AiKeywordExtra),
            "PictureTagRecognition" => Some(Self::PictureTagRecognition),
            "IntellectCover" => Some(Self::IntellectCover),
            "AIActivity" => Some(Self::AiActivity),
            "DigitalHuman" => Some(Self::DigitalHuman),
            "AIStraightTitle" => Some(Self::AiStraightTitle),
            "GameFactory" => Some(Self::GameFactory),
            "NonSmartVideo" => Some(Self::NonSmartVideo),
            "Replace" => Some(Self::Replace),
            "TTSClone" => Some(Self::TtsClone),
            "SmartTitlePink" => Some(Self::SmartTitlePink),
            "RecognitionMusic" => Some(Self::RecognitionMusic),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MaterialForm {
    ///
    Normal = 0,
    ///
    AiRecommend = 1,
    ///
    TacticsRecommend = 2,
    ///
    Search = 3,
}
impl MaterialForm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::AiRecommend => "AIRecommend",
            Self::TacticsRecommend => "TacticsRecommend",
            Self::Search => "Search",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Normal" => Some(Self::Normal),
            "AIRecommend" => Some(Self::AiRecommend),
            "TacticsRecommend" => Some(Self::TacticsRecommend),
            "Search" => Some(Self::Search),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MaterialType {
    ///
    Subtitles = 0,
    ///
    Fonts = 1,
    ///
    Filters = 2,
    ///
    Bgms = 3,
    ///
    ShootStickers = 5,
    ///
    VideoupStickers = 7,
    ///
    Trans = 8,
    ///
    Cooperates = 9,
    ///
    Themes = 10,
    ///
    Makeups = 11,
    ///
    Surgerys = 12,
    ///
    Videofxs = 13,
    ///
    Rhythms = 14,
    ///
    BSticker = 15,
    ///
    Particle = 16,
    ///
    Effects = 17,
    ///
    Backgrounds = 18,
    ///
    Videos = 19,
    ///
    Sounds = 20,
    ///
    Flower = 21,
    ///
    BcutHotWord = 22,
    ///
    VuPerBg = 25,
    ///
    CoverTemplates = 26,
    ///
    Tts = 27,
    ///
    Openings = 28,
    ///
    Vupers = 30,
    ///
    InstructBarrageTemplate = 31,
    ///
    LovelySticker = 32,
    ///
    LovelyStickerHotWord = 33,
    ///
    QuickBeauty = 34,
    ///
    MakeupBeauty = 35,
    ///
    Textmotion = 36,
    ///
    BcutAudioHotWord = 37,
    ///
    VideoTemplatesFull = 38,
    ///
    VideoTemplatesStart = 39,
    ///
    HotTopicEffects = 40,
    ///
    CoverTemplate = 41,
    ///
    InteractiveBarrage = 42,
    ///
    VideoHotWord = 43,
    ///
    TextToVideoTemplate = 44,
    ///
    OpeningTemplate = 45,
    ///
    VideoTemplate = 46,
    ///
    VupTemplate = 47,
    ///
    RemixTemplate = 49,
    ///
    GameTemplate = 50,
    ///
    ProcessBar = 51,
    ///
    VtuberProperty = 52,
    ///
    VtuberBackground = 53,
    ///
    DubbingTool = 55,
    ///
    HotTopicVideos = 60,
    ///
    Asr = 61,
    ///
    WebCoverTemplate = 62,
    ///
    CameraFilter = 63,
    ///
    CameraEffect = 64,
    ///
    VideoWave = 66,
    ///
    VirtualBg = 67,
    ///
    VirtualProp = 68,
    ///
    VoiceChange = 71,
    ///
    SmartTemplate = 72,
    ///
    WzryGameTemplate = 73,
    ///
    AiDrawingModel = 74,
    ///
    AiPlayModel = 75,
    ///
    VideoGenMaterialUse = 79,
}
impl MaterialType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Subtitles => "Subtitles",
            Self::Fonts => "Fonts",
            Self::Filters => "Filters",
            Self::Bgms => "Bgms",
            Self::ShootStickers => "ShootStickers",
            Self::VideoupStickers => "VideoupStickers",
            Self::Trans => "Trans",
            Self::Cooperates => "Cooperates",
            Self::Themes => "Themes",
            Self::Makeups => "Makeups",
            Self::Surgerys => "Surgerys",
            Self::Videofxs => "Videofxs",
            Self::Rhythms => "Rhythms",
            Self::BSticker => "BSticker",
            Self::Particle => "Particle",
            Self::Effects => "Effects",
            Self::Backgrounds => "Backgrounds",
            Self::Videos => "Videos",
            Self::Sounds => "Sounds",
            Self::Flower => "Flower",
            Self::BcutHotWord => "BcutHotWord",
            Self::VuPerBg => "VUPerBG",
            Self::CoverTemplates => "CoverTemplates",
            Self::Tts => "TTS",
            Self::Openings => "Openings",
            Self::Vupers => "Vupers",
            Self::InstructBarrageTemplate => "InstructBarrageTemplate",
            Self::LovelySticker => "LovelySticker",
            Self::LovelyStickerHotWord => "LovelyStickerHotWord",
            Self::QuickBeauty => "QuickBeauty",
            Self::MakeupBeauty => "MakeupBeauty",
            Self::Textmotion => "Textmotion",
            Self::BcutAudioHotWord => "BcutAudioHotWord",
            Self::VideoTemplatesFull => "VideoTemplatesFull",
            Self::VideoTemplatesStart => "VideoTemplatesStart",
            Self::HotTopicEffects => "HotTopicEffects",
            Self::CoverTemplate => "CoverTemplate",
            Self::InteractiveBarrage => "InteractiveBarrage",
            Self::VideoHotWord => "VideoHotWord",
            Self::TextToVideoTemplate => "TextToVideoTemplate",
            Self::OpeningTemplate => "OpeningTemplate",
            Self::VideoTemplate => "VideoTemplate",
            Self::VupTemplate => "VupTemplate",
            Self::RemixTemplate => "RemixTemplate",
            Self::GameTemplate => "GameTemplate",
            Self::ProcessBar => "ProcessBar",
            Self::VtuberProperty => "VtuberProperty",
            Self::VtuberBackground => "VtuberBackground",
            Self::DubbingTool => "DubbingTool",
            Self::HotTopicVideos => "HotTopicVideos",
            Self::Asr => "ASR",
            Self::WebCoverTemplate => "WebCoverTemplate",
            Self::CameraFilter => "CameraFilter",
            Self::CameraEffect => "CameraEffect",
            Self::VideoWave => "VideoWave",
            Self::VirtualBg => "VirtualBg",
            Self::VirtualProp => "VirtualProp",
            Self::VoiceChange => "VoiceChange",
            Self::SmartTemplate => "SmartTemplate",
            Self::WzryGameTemplate => "WzryGameTemplate",
            Self::AiDrawingModel => "AiDrawingModel",
            Self::AiPlayModel => "AiPlayModel",
            Self::VideoGenMaterialUse => "VideoGenMaterialUse",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Subtitles" => Some(Self::Subtitles),
            "Fonts" => Some(Self::Fonts),
            "Filters" => Some(Self::Filters),
            "Bgms" => Some(Self::Bgms),
            "ShootStickers" => Some(Self::ShootStickers),
            "VideoupStickers" => Some(Self::VideoupStickers),
            "Trans" => Some(Self::Trans),
            "Cooperates" => Some(Self::Cooperates),
            "Themes" => Some(Self::Themes),
            "Makeups" => Some(Self::Makeups),
            "Surgerys" => Some(Self::Surgerys),
            "Videofxs" => Some(Self::Videofxs),
            "Rhythms" => Some(Self::Rhythms),
            "BSticker" => Some(Self::BSticker),
            "Particle" => Some(Self::Particle),
            "Effects" => Some(Self::Effects),
            "Backgrounds" => Some(Self::Backgrounds),
            "Videos" => Some(Self::Videos),
            "Sounds" => Some(Self::Sounds),
            "Flower" => Some(Self::Flower),
            "BcutHotWord" => Some(Self::BcutHotWord),
            "VUPerBG" => Some(Self::VuPerBg),
            "CoverTemplates" => Some(Self::CoverTemplates),
            "TTS" => Some(Self::Tts),
            "Openings" => Some(Self::Openings),
            "Vupers" => Some(Self::Vupers),
            "InstructBarrageTemplate" => Some(Self::InstructBarrageTemplate),
            "LovelySticker" => Some(Self::LovelySticker),
            "LovelyStickerHotWord" => Some(Self::LovelyStickerHotWord),
            "QuickBeauty" => Some(Self::QuickBeauty),
            "MakeupBeauty" => Some(Self::MakeupBeauty),
            "Textmotion" => Some(Self::Textmotion),
            "BcutAudioHotWord" => Some(Self::BcutAudioHotWord),
            "VideoTemplatesFull" => Some(Self::VideoTemplatesFull),
            "VideoTemplatesStart" => Some(Self::VideoTemplatesStart),
            "HotTopicEffects" => Some(Self::HotTopicEffects),
            "CoverTemplate" => Some(Self::CoverTemplate),
            "InteractiveBarrage" => Some(Self::InteractiveBarrage),
            "VideoHotWord" => Some(Self::VideoHotWord),
            "TextToVideoTemplate" => Some(Self::TextToVideoTemplate),
            "OpeningTemplate" => Some(Self::OpeningTemplate),
            "VideoTemplate" => Some(Self::VideoTemplate),
            "VupTemplate" => Some(Self::VupTemplate),
            "RemixTemplate" => Some(Self::RemixTemplate),
            "GameTemplate" => Some(Self::GameTemplate),
            "ProcessBar" => Some(Self::ProcessBar),
            "VtuberProperty" => Some(Self::VtuberProperty),
            "VtuberBackground" => Some(Self::VtuberBackground),
            "DubbingTool" => Some(Self::DubbingTool),
            "HotTopicVideos" => Some(Self::HotTopicVideos),
            "ASR" => Some(Self::Asr),
            "WebCoverTemplate" => Some(Self::WebCoverTemplate),
            "CameraFilter" => Some(Self::CameraFilter),
            "CameraEffect" => Some(Self::CameraEffect),
            "VideoWave" => Some(Self::VideoWave),
            "VirtualBg" => Some(Self::VirtualBg),
            "VirtualProp" => Some(Self::VirtualProp),
            "VoiceChange" => Some(Self::VoiceChange),
            "SmartTemplate" => Some(Self::SmartTemplate),
            "WzryGameTemplate" => Some(Self::WzryGameTemplate),
            "AiDrawingModel" => Some(Self::AiDrawingModel),
            "AiPlayModel" => Some(Self::AiPlayModel),
            "VideoGenMaterialUse" => Some(Self::VideoGenMaterialUse),
            _ => None,
        }
    }
}

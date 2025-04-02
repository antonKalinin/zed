use std::sync::Arc;

use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};

#[derive(
    Debug, PartialEq, Eq, Copy, Clone, EnumIter, EnumString, IntoStaticStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum IconName {
    Ai,
    AiAnthropic,
    AiBedrock,
    AiAnthropicHosted,
    AiDeepSeek,
    AiEdit,
    AiGoogle,
    AiLmStudio,
    AiMistral,
    AiOllama,
    AiOpenAi,
    AiZed,
    ArrowCircle,
    ArrowDown,
    ArrowDownFromLine,
    ArrowLeft,
    ArrowRight,
    ArrowRightLeft,
    ArrowUp,
    ArrowUpFromLine,
    ArrowUpRight,
    ArrowUpRightAlt,
    AtSign,
    AudioOff,
    AudioOn,
    Backspace,
    Bell,
    BellDot,
    BellOff,
    BellRing,
    Blocks,
    Bolt,
    Book,
    BookCopy,
    BookPlus,
    Brain,
    CaseSensitive,
    Check,
    ChevronDown,
    /// This chevron indicates a popover menu.
    ChevronDownSmall,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronUpDown,
    Circle,
    Clipboard,
    Close,
    Code,
    Cog,
    Command,
    Context,
    Control,
    Copilot,
    CopilotDisabled,
    CopilotError,
    CopilotInit,
    Copy,
    CountdownTimer,
    CursorIBeam,
    Dash,
    DebugBreakpoint,
    DebugDisabledBreakpoint,
    DebugDisabledLogBreakpoint,
    DebugIgnoreBreakpoints,
    DebugPause,
    DebugContinue,
    DebugStepOver,
    DebugStepInto,
    DebugStepOut,
    DebugStepBack,
    DebugRestart,
    Debug,
    DebugStop,
    DebugDisconnect,
    DebugLogBreakpoint,
    DatabaseZap,
    Delete,
    Diff,
    Disconnected,
    Download,
    Ellipsis,
    EllipsisVertical,
    Envelope,
    Eraser,
    Escape,
    ExpandVertical,
    Exit,
    ExternalLink,
    ExpandUp,
    ExpandDown,
    Eye,
    File,
    FileCode,
    FileCreate,
    FileDelete,
    FileDoc,
    FileDiff,
    FileGeneric,
    FileGit,
    FileLock,
    FileRust,
    FileSearch,
    FileText,
    FileToml,
    FileTree,
    Filter,
    Folder,
    FolderOpen,
    FolderX,
    Font,
    FontSize,
    FontWeight,
    GenericClose,
    GenericMaximize,
    GenericMinimize,
    GenericRestore,
    Github,
    Globe,
    GitBranch,
    GitBranchSmall,
    Hash,
    HistoryRerun,
    Indicator,
    Info,
    InlayHint,
    Keyboard,
    Library,
    LineHeight,
    Link,
    ListTree,
    ListX,
    LockOutlined,
    MagnifyingGlass,
    MailOpen,
    Maximize,
    Menu,
    MessageBubbles,
    Cloud,
    Mic,
    MicMute,
    Microscope,
    Minimize,
    Option,
    PageDown,
    PageUp,
    PanelLeft,
    PanelRight,
    Pencil,
    Person,
    PersonCircle,
    PhoneIncoming,
    Pin,
    Play,
    Plus,
    PocketKnife,
    Public,
    PullRequest,
    Quote,
    RefreshTitle,
    Regex,
    ReplNeutral,
    Replace,
    ReplaceAll,
    ReplaceNext,
    ReplyArrowRight,
    Rerun,
    Return,
    Reveal,
    RotateCcw,
    RotateCw,
    Route,
    Save,
    Screen,
    SearchCode,
    SearchSelection,
    SelectAll,
    Server,
    Settings,
    SettingsAlt,
    Shift,
    Slash,
    SlashSquare,
    Sliders,
    SlidersVertical,
    Snip,
    Space,
    Sparkle,
    SparkleAlt,
    SparkleFilled,
    Spinner,
    Split,
    SquareDot,
    SquareMinus,
    SquarePlus,
    Star,
    StarFilled,
    Stop,
    Strikethrough,
    Supermaven,
    SupermavenDisabled,
    SupermavenError,
    SupermavenInit,
    SwatchBook,
    Tab,
    Terminal,
    TextSnippet,
    ThumbsUp,
    ThumbsDown,
    Trash,
    TrashAlt,
    Triangle,
    TriangleRight,
    Undo,
    Unpin,
    Update,
    UserGroup,
    UserRoundPen,
    Visible,
    Wand,
    Warning,
    WholeWord,
    X,
    XCircle,
    ZedAssistant,
    ZedAssistantFilled,
    ZedPredict,
    ZedPredictUp,
    ZedPredictDown,
    ZedPredictDisabled,
    ZedPredictError,
    ZedXCopilot,
}

impl IconName {
    /// Returns the path to this icon.
    pub fn path(&self) -> Arc<str> {
        let file_stem: &'static str = self.into();
        format!("icons/{file_stem}.svg").into()
    }
}

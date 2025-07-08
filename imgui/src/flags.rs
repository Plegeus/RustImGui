

mod bindings {
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  include!("bindings/imgui_h.rs");
}



pub(crate) trait GuiFlag {
  fn as_i32(&self) -> i32;
  fn default_i32() -> i32;
}

macro_rules! impl_gui_flag {
  ($T:ty, $def:tt) => {
    impl crate::GuiFlag for $T {
      fn as_i32(&self) -> i32 {
        *self as i32
      } 
      fn default_i32() -> i32 {
        <$T>::$def.as_i32()
      }
    }
    impl GuiFlag for [$T] {
      fn as_i32(&self) -> i32 {
        self.into_iter()
          .fold(0, |l, r| l | r.as_i32())
      }
      fn default_i32() -> i32 {
        <$T>::default_i32()
      }
    }
  };
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cond {
  None = bindings::ImGuiCond__ImGuiCond_None as isize,
  Always = bindings::ImGuiCond__ImGuiCond_Always as isize,
  Once = bindings::ImGuiCond__ImGuiCond_Once as isize,
  FirstUseEver = bindings::ImGuiCond__ImGuiCond_FirstUseEver as isize,
  Appearing = bindings::ImGuiCond__ImGuiCond_Appearing as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleVar {
  Alpha = bindings::ImGuiStyleVar__ImGuiStyleVar_Alpha as isize,
  DisabledAlpha = bindings::ImGuiStyleVar__ImGuiStyleVar_DisabledAlpha as isize,
  WindowPadding = bindings::ImGuiStyleVar__ImGuiStyleVar_WindowPadding as isize,
  WindowRounding = bindings::ImGuiStyleVar__ImGuiStyleVar_WindowRounding as isize,
  WindowBorderSize = bindings::ImGuiStyleVar__ImGuiStyleVar_WindowBorderSize as isize,
  WindowMinSize = bindings::ImGuiStyleVar__ImGuiStyleVar_WindowMinSize as isize,
  WindowTitleAlign = bindings::ImGuiStyleVar__ImGuiStyleVar_WindowTitleAlign as isize,
  ChildRounding = bindings::ImGuiStyleVar__ImGuiStyleVar_ChildRounding as isize,
  ChildBorderSize = bindings::ImGuiStyleVar__ImGuiStyleVar_ChildBorderSize as isize,
  PopupRounding = bindings::ImGuiStyleVar__ImGuiStyleVar_PopupRounding as isize,
  PopupBorderSize = bindings::ImGuiStyleVar__ImGuiStyleVar_PopupBorderSize as isize,
  FramePadding = bindings::ImGuiStyleVar__ImGuiStyleVar_FramePadding as isize,
  FrameRounding = bindings::ImGuiStyleVar__ImGuiStyleVar_FrameRounding as isize,
  FrameBorderSize = bindings::ImGuiStyleVar__ImGuiStyleVar_FrameBorderSize as isize,
  ItemSpacing = bindings::ImGuiStyleVar__ImGuiStyleVar_ItemSpacing as isize,
  ItemInnerSpacing = bindings::ImGuiStyleVar__ImGuiStyleVar_ItemInnerSpacing as isize,
  IndentSpacing = bindings::ImGuiStyleVar__ImGuiStyleVar_IndentSpacing as isize,
  CellPadding = bindings::ImGuiStyleVar__ImGuiStyleVar_CellPadding as isize,
  ScrollbarSize = bindings::ImGuiStyleVar__ImGuiStyleVar_ScrollbarSize as isize,
  ScrollbarRounding = bindings::ImGuiStyleVar__ImGuiStyleVar_ScrollbarRounding as isize,
  GrabMinSize = bindings::ImGuiStyleVar__ImGuiStyleVar_GrabMinSize as isize,
  GrabRounding = bindings::ImGuiStyleVar__ImGuiStyleVar_GrabRounding as isize,
  ImageBorderSize = bindings::ImGuiStyleVar__ImGuiStyleVar_ImageBorderSize as isize,
  TabRounding = bindings::ImGuiStyleVar__ImGuiStyleVar_TabRounding as isize,
  TabBorderSize = bindings::ImGuiStyleVar__ImGuiStyleVar_TabBorderSize as isize,
  TabBarBorderSize = bindings::ImGuiStyleVar__ImGuiStyleVar_TabBarBorderSize as isize,
  TabBarOverlineSize = bindings::ImGuiStyleVar__ImGuiStyleVar_TabBarOverlineSize as isize,
  TableAngledHeadersAngle = bindings::ImGuiStyleVar__ImGuiStyleVar_TableAngledHeadersAngle as isize,
  TableAngledHeadersTextAlign = bindings::ImGuiStyleVar__ImGuiStyleVar_TableAngledHeadersTextAlign as isize,
  TreeLinesSize = bindings::ImGuiStyleVar__ImGuiStyleVar_TreeLinesSize as isize,
  TreeLinesRounding = bindings::ImGuiStyleVar__ImGuiStyleVar_TreeLinesRounding as isize,
  ButtonTextAlign = bindings::ImGuiStyleVar__ImGuiStyleVar_ButtonTextAlign as isize,
  SelectableTextAlign = bindings::ImGuiStyleVar__ImGuiStyleVar_SelectableTextAlign as isize,
  SeparatorTextBorderSize = bindings::ImGuiStyleVar__ImGuiStyleVar_SeparatorTextBorderSize as isize,
  SeparatorTextAlign = bindings::ImGuiStyleVar__ImGuiStyleVar_SeparatorTextAlign as isize,
  SeparatorTextPadding = bindings::ImGuiStyleVar__ImGuiStyleVar_SeparatorTextPadding as isize,
  DockingSeparatorSize = bindings::ImGuiStyleVar__ImGuiStyleVar_DockingSeparatorSize as isize,
  COUNT = bindings::ImGuiStyleVar__ImGuiStyleVar_COUNT as isize,
}
// ImGuiCol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorVar {
  Text = bindings::ImGuiCol__ImGuiCol_Text as isize,
  TextDisabled = bindings::ImGuiCol__ImGuiCol_TextDisabled as isize,
  WindowBg = bindings::ImGuiCol__ImGuiCol_WindowBg as isize,
  ChildBg = bindings::ImGuiCol__ImGuiCol_ChildBg as isize,
  PopupBg = bindings::ImGuiCol__ImGuiCol_PopupBg as isize,
  Border = bindings::ImGuiCol__ImGuiCol_Border as isize,
  BorderShadow = bindings::ImGuiCol__ImGuiCol_BorderShadow as isize,
  FrameBg = bindings::ImGuiCol__ImGuiCol_FrameBg as isize,
  FrameBgHovered = bindings::ImGuiCol__ImGuiCol_FrameBgHovered as isize,
  FrameBgActive = bindings::ImGuiCol__ImGuiCol_FrameBgActive as isize,
  TitleBg = bindings::ImGuiCol__ImGuiCol_TitleBg as isize,
  TitleBgActive = bindings::ImGuiCol__ImGuiCol_TitleBgActive as isize,
  TitleBgCollapsed = bindings::ImGuiCol__ImGuiCol_TitleBgCollapsed as isize,
  MenuBarBg = bindings::ImGuiCol__ImGuiCol_MenuBarBg as isize,
  ScrollbarBg = bindings::ImGuiCol__ImGuiCol_ScrollbarBg as isize,
  ScrollbarGrab = bindings::ImGuiCol__ImGuiCol_ScrollbarGrab as isize,
  ScrollbarGrabHovered = bindings::ImGuiCol__ImGuiCol_ScrollbarGrabHovered as isize,
  ScrollbarGrabActive = bindings::ImGuiCol__ImGuiCol_ScrollbarGrabActive as isize,
  CheckMark = bindings::ImGuiCol__ImGuiCol_CheckMark as isize,
  SliderGrab = bindings::ImGuiCol__ImGuiCol_SliderGrab as isize,
  SliderGrabActive = bindings::ImGuiCol__ImGuiCol_SliderGrabActive as isize,
  Button = bindings::ImGuiCol__ImGuiCol_Button as isize,
  ButtonHovered = bindings::ImGuiCol__ImGuiCol_ButtonHovered as isize,
  ButtonActive = bindings::ImGuiCol__ImGuiCol_ButtonActive as isize,
  Header = bindings::ImGuiCol__ImGuiCol_Header as isize,
  HeaderHovered = bindings::ImGuiCol__ImGuiCol_HeaderHovered as isize,
  HeaderActive = bindings::ImGuiCol__ImGuiCol_HeaderActive as isize,
  Separator = bindings::ImGuiCol__ImGuiCol_Separator as isize,
  SeparatorHovered = bindings::ImGuiCol__ImGuiCol_SeparatorHovered as isize,
  SeparatorActive = bindings::ImGuiCol__ImGuiCol_SeparatorActive as isize,
  ResizeGrip = bindings::ImGuiCol__ImGuiCol_ResizeGrip as isize,
  ResizeGripHovered = bindings::ImGuiCol__ImGuiCol_ResizeGripHovered as isize,
  ResizeGripActive = bindings::ImGuiCol__ImGuiCol_ResizeGripActive as isize,
  InputTextCursor = bindings::ImGuiCol__ImGuiCol_InputTextCursor as isize,
  TabHovered = bindings::ImGuiCol__ImGuiCol_TabHovered as isize,
  Tab = bindings::ImGuiCol__ImGuiCol_Tab as isize,
  TabSelected = bindings::ImGuiCol__ImGuiCol_TabSelected as isize,
  TabSelectedOverline = bindings::ImGuiCol__ImGuiCol_TabSelectedOverline as isize,
  TabDimmed = bindings::ImGuiCol__ImGuiCol_TabDimmed as isize,
  TabDimmedSelected = bindings::ImGuiCol__ImGuiCol_TabDimmedSelected as isize,
  TabDimmedSelectedOverline = bindings::ImGuiCol__ImGuiCol_TabDimmedSelectedOverline as isize,
  DockingPreview = bindings::ImGuiCol__ImGuiCol_DockingPreview as isize,
  DockingEmptyBg = bindings::ImGuiCol__ImGuiCol_DockingEmptyBg as isize,
  PlotLines = bindings::ImGuiCol__ImGuiCol_PlotLines as isize,
  PlotLinesHovered = bindings::ImGuiCol__ImGuiCol_PlotLinesHovered as isize,
  PlotHistogram = bindings::ImGuiCol__ImGuiCol_PlotHistogram as isize,
  PlotHistogramHovered = bindings::ImGuiCol__ImGuiCol_PlotHistogramHovered as isize,
  TableHeaderBg = bindings::ImGuiCol__ImGuiCol_TableHeaderBg as isize,
  TableBorderStrong = bindings::ImGuiCol__ImGuiCol_TableBorderStrong as isize,
  TableBorderLight = bindings::ImGuiCol__ImGuiCol_TableBorderLight as isize,
  TableRowBg = bindings::ImGuiCol__ImGuiCol_TableRowBg as isize,
  TableRowBgAlt = bindings::ImGuiCol__ImGuiCol_TableRowBgAlt as isize,
  TextLink = bindings::ImGuiCol__ImGuiCol_TextLink as isize,
  TextSelectedBg = bindings::ImGuiCol__ImGuiCol_TextSelectedBg as isize,
  TreeLines = bindings::ImGuiCol__ImGuiCol_TreeLines as isize,
  DragDropTarget = bindings::ImGuiCol__ImGuiCol_DragDropTarget as isize,
  NavCursor = bindings::ImGuiCol__ImGuiCol_NavCursor as isize,
  NavWindowingHighlight = bindings::ImGuiCol__ImGuiCol_NavWindowingHighlight as isize,
  NavWindowingDimBg = bindings::ImGuiCol__ImGuiCol_NavWindowingDimBg as isize,
  ModalWindowDimBg = bindings::ImGuiCol__ImGuiCol_ModalWindowDimBg as isize,
  COUNT = bindings::ImGuiCol__ImGuiCol_COUNT as isize,
  //TabActive = bindings::ImGuiCol__ImGuiCol_TabActive as isize, == // TabSelected
  //TabUnfocused = bindings::ImGuiCol__ImGuiCol_TabUnfocused as isize, // == TabDimmed
  //TabUnfocusedActive = bindings::ImGuiCol__ImGuiCol_TabUnfocusedActive as isize, // == TabDimmedSelected
  //NavHighlight = bindings::ImGuiCol__ImGuiCol_NavHighlight as isize, // == NavCursor
}

pub enum MouseButton {
  Left = bindings::ImGuiMouseButton__ImGuiMouseButton_Left as isize,
  Right = bindings::ImGuiMouseButton__ImGuiMouseButton_Right as isize,
  Middle = bindings::ImGuiMouseButton__ImGuiMouseButton_Middle as isize,
  COUNT = bindings::ImGuiMouseButton__ImGuiMouseButton_COUNT as isize,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoveredFlags {
  None = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_None as isize,
  ChildWindows = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_ChildWindows as isize,
  RootWindow = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_RootWindow as isize,
  AnyWindow = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_AnyWindow as isize,
  NoPopupHierarchy = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_NoPopupHierarchy as isize,
  DockHierarchy = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_DockHierarchy as isize,
  RootAndChildWindows = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_RootAndChildWindows as isize,
}
impl Default for HoveredFlags {
  fn default() -> Self {
    HoveredFlags::None
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedFlags {
  None = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_None as isize,
  ChildWindows = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_ChildWindows as isize,
  RootWindow = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_RootWindow as isize,
  AnyWindow = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_AnyWindow as isize,
  NoPopupHierarchy = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_NoPopupHierarchy as isize,
  DockHierarchy = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_DockHierarchy as isize,
  RootAndChildWindows = bindings::ImGuiFocusedFlags__ImGuiFocusedFlags_RootAndChildWindows as isize,
}
impl Default for FocusedFlags {
  fn default() -> Self {
    FocusedFlags::None
  }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowFlags {
  None = bindings::ImGuiWindowFlags__ImGuiWindowFlags_None as isize,
  NoTitleBar = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoTitleBar as isize,
  NoResize = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoResize as isize,
  NoMove = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoMove as isize,
  NoScrollbar = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoScrollbar as isize,
  NoScrollWithMouse = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoScrollWithMouse as isize,
  NoCollapse = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoCollapse as isize,
  AlwaysAutoResize = bindings::ImGuiWindowFlags__ImGuiWindowFlags_AlwaysAutoResize as isize,
  NoBackground = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoBackground as isize,
  NoSavedSettings = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoSavedSettings as isize,
  NoMouseInputs = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoMouseInputs as isize,
  MenuBar = bindings::ImGuiWindowFlags__ImGuiWindowFlags_MenuBar as isize,
  HorizontalScrollbar = bindings::ImGuiWindowFlags__ImGuiWindowFlags_HorizontalScrollbar as isize,
  NoFocusOnAppearing = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoFocusOnAppearing as isize,
  NoBringToFrontOnFocus = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoBringToFrontOnFocus as isize,
  AlwaysVerticalScrollbar = bindings::ImGuiWindowFlags__ImGuiWindowFlags_AlwaysVerticalScrollbar as isize,
  AlwaysHorizontalScrollbar = bindings::ImGuiWindowFlags__ImGuiWindowFlags_AlwaysHorizontalScrollbar as isize,
  NoNavInputs = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoNavInputs as isize,
  NoNavFocus = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoNavFocus as isize,
  UnsavedDocument = bindings::ImGuiWindowFlags__ImGuiWindowFlags_UnsavedDocument as isize,
  NoDocking = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoDocking as isize,
  NoNav = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoNav as isize,
  NoDecoration = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoDecoration as isize,
  NoInputs = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NoInputs as isize,
  DockNodeHost = bindings::ImGuiWindowFlags__ImGuiWindowFlags_DockNodeHost as isize,
  ChildWindow = bindings::ImGuiWindowFlags__ImGuiWindowFlags_ChildWindow as isize,
  Tooltip = bindings::ImGuiWindowFlags__ImGuiWindowFlags_Tooltip as isize,
  Popup = bindings::ImGuiWindowFlags__ImGuiWindowFlags_Popup as isize,
  Modal = bindings::ImGuiWindowFlags__ImGuiWindowFlags_Modal as isize,
  ChildMenu = bindings::ImGuiWindowFlags__ImGuiWindowFlags_ChildMenu as isize,
  NavFlattened = bindings::ImGuiWindowFlags__ImGuiWindowFlags_NavFlattened as isize,
  AlwaysUseWindowPadding = bindings::ImGuiWindowFlags__ImGuiWindowFlags_AlwaysUseWindowPadding as isize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildFlags {
  None = bindings::ImGuiChildFlags__ImGuiChildFlags_None as isize,
  Borders = bindings::ImGuiChildFlags__ImGuiChildFlags_Borders as isize,
  AlwaysUseWindowPadding = bindings::ImGuiChildFlags__ImGuiChildFlags_AlwaysUseWindowPadding as isize,
  ResizeX = bindings::ImGuiChildFlags__ImGuiChildFlags_ResizeX as isize,
  ResizeY = bindings::ImGuiChildFlags__ImGuiChildFlags_ResizeY as isize,
  AutoResizeX = bindings::ImGuiChildFlags__ImGuiChildFlags_AutoResizeX as isize,
  AutoResizeY = bindings::ImGuiChildFlags__ImGuiChildFlags_AutoResizeY as isize,
  AlwaysAutoResize = bindings::ImGuiChildFlags__ImGuiChildFlags_AlwaysAutoResize as isize,
  FrameStyle = bindings::ImGuiChildFlags__ImGuiChildFlags_FrameStyle as isize,
  NavFlattened = bindings::ImGuiChildFlags__ImGuiChildFlags_NavFlattened as isize,
  //Border = bindings::ImGuiChildFlags__ImGuiChildFlags_Border as isize, // == Borders
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupFlags {
  //None = bindings::ImGuiPopupFlags__ImGuiPopupFlags_None as isize, // == MouseButtonLeft
  MouseButtonLeft = bindings::ImGuiPopupFlags__ImGuiPopupFlags_MouseButtonLeft as isize,
  MouseButtonRight = bindings::ImGuiPopupFlags__ImGuiPopupFlags_MouseButtonRight as isize,
  MouseButtonMiddle = bindings::ImGuiPopupFlags__ImGuiPopupFlags_MouseButtonMiddle as isize,
  MouseButtonMask_ = bindings::ImGuiPopupFlags__ImGuiPopupFlags_MouseButtonMask_ as isize,
  //MouseButtonDefault_ = bindings::ImGuiPopupFlags__ImGuiPopupFlags_MouseButtonDefault_ as isize, // == MouseButtonRight
  NoReopen = bindings::ImGuiPopupFlags__ImGuiPopupFlags_NoReopen as isize,
  NoOpenOverExistingPopup = bindings::ImGuiPopupFlags__ImGuiPopupFlags_NoOpenOverExistingPopup as isize,
  NoOpenOverItems = bindings::ImGuiPopupFlags__ImGuiPopupFlags_NoOpenOverItems as isize,
  AnyPopupId = bindings::ImGuiPopupFlags__ImGuiPopupFlags_AnyPopupId as isize,
  AnyPopupLevel = bindings::ImGuiPopupFlags__ImGuiPopupFlags_AnyPopupLevel as isize,
  AnyPopup = bindings::ImGuiPopupFlags__ImGuiPopupFlags_AnyPopup as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarFlags {
  None = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_None as isize,
  Reorderable = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_Reorderable as isize,
  AutoSelectNewTabs = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_AutoSelectNewTabs as isize,
  TabListPopupButton = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_TabListPopupButton as isize,
  NoCloseWithMiddleMouseButton = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_NoCloseWithMiddleMouseButton as isize,
  NoTabListScrollingButtons = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_NoTabListScrollingButtons as isize,
  NoTooltip = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_NoTooltip as isize,
  DrawSelectedOverline = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_DrawSelectedOverline as isize,
  FittingPolicyResizeDown = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_FittingPolicyResizeDown as isize,
  FittingPolicyScroll = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_FittingPolicyScroll as isize,
  FittingPolicyMask_ = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_FittingPolicyMask_ as isize,
  //FittingPolicyDefault_ = bindings::ImGuiTabBarFlags__ImGuiTabBarFlags_FittingPolicyDefault_ as isize, // == FittingPolicyResizeDown
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabItemFlags {
  None = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_None as isize,
  UnsavedDocument = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_UnsavedDocument as isize,
  SetSelected = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_SetSelected as isize,
  NoCloseWithMiddleMouseButton = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_NoCloseWithMiddleMouseButton as isize,
  NoPushId = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_NoPushId as isize,
  NoTooltip = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_NoTooltip as isize,
  NoReorder = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_NoReorder as isize,
  Leading = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_Leading as isize,
  Trailing = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_Trailing as isize,
  NoAssumedClosure = bindings::ImGuiTabItemFlags__ImGuiTabItemFlags_NoAssumedClosure as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableColumnFlags {
  None = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_None as isize,
  Disabled = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_Disabled as isize,
  DefaultHide = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_DefaultHide as isize,
  DefaultSort = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_DefaultSort as isize,
  WidthStretch = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_WidthStretch as isize,
  WidthFixed = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_WidthFixed as isize,
  NoResize = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoResize as isize,
  NoReorder = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoReorder as isize,
  NoHide = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoHide as isize,
  NoClip = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoClip as isize,
  NoSort = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoSort as isize,
  NoSortAscending = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoSortAscending as isize,
  NoSortDescending = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoSortDescending as isize,
  NoHeaderLabel = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoHeaderLabel as isize,
  NoHeaderWidth = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoHeaderWidth as isize,
  PreferSortAscending = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_PreferSortAscending as isize,
  PreferSortDescending = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_PreferSortDescending as isize,
  IndentEnable = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_IndentEnable as isize,
  IndentDisable = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_IndentDisable as isize,
  AngledHeader = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_AngledHeader as isize,
  IsEnabled = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_IsEnabled as isize,
  IsVisible = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_IsVisible as isize,
  IsSorted = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_IsSorted as isize,
  IsHovered = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_IsHovered as isize,
  WidthMask_ = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_WidthMask_ as isize,
  IndentMask_ = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_IndentMask_ as isize,
  StatusMask_ = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_StatusMask_ as isize,
  NoDirectResize_ = bindings::ImGuiTableColumnFlags__ImGuiTableColumnFlags_NoDirectResize_ as isize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableFlags {
  None = bindings::ImGuiTableFlags__ImGuiTableFlags_None as isize,
  Resizable = bindings::ImGuiTableFlags__ImGuiTableFlags_Resizable as isize,
  Reorderable = bindings::ImGuiTableFlags__ImGuiTableFlags_Reorderable as isize,
  Hideable = bindings::ImGuiTableFlags__ImGuiTableFlags_Hideable as isize,
  Sortable = bindings::ImGuiTableFlags__ImGuiTableFlags_Sortable as isize,
  NoSavedSettings = bindings::ImGuiTableFlags__ImGuiTableFlags_NoSavedSettings as isize,
  ContextMenuInBody = bindings::ImGuiTableFlags__ImGuiTableFlags_ContextMenuInBody as isize,
  RowBg = bindings::ImGuiTableFlags__ImGuiTableFlags_RowBg as isize,
  BordersInnerH = bindings::ImGuiTableFlags__ImGuiTableFlags_BordersInnerH as isize,
  BordersOuterH = bindings::ImGuiTableFlags__ImGuiTableFlags_BordersOuterH as isize,
  BordersInnerV = bindings::ImGuiTableFlags__ImGuiTableFlags_BordersInnerV as isize,
  BordersOuterV = bindings::ImGuiTableFlags__ImGuiTableFlags_BordersOuterV as isize,
  BordersH = bindings::ImGuiTableFlags__ImGuiTableFlags_BordersH as isize,
  BordersV = bindings::ImGuiTableFlags__ImGuiTableFlags_BordersV as isize,
  BordersInner = bindings::ImGuiTableFlags__ImGuiTableFlags_BordersInner as isize,
  BordersOuter = bindings::ImGuiTableFlags__ImGuiTableFlags_BordersOuter as isize,
  Borders = bindings::ImGuiTableFlags__ImGuiTableFlags_Borders as isize,
  NoBordersInBody = bindings::ImGuiTableFlags__ImGuiTableFlags_NoBordersInBody as isize,
  NoBordersInBodyUntilResize = bindings::ImGuiTableFlags__ImGuiTableFlags_NoBordersInBodyUntilResize as isize,
  SizingFixedFit = bindings::ImGuiTableFlags__ImGuiTableFlags_SizingFixedFit as isize,
  SizingFixedSame = bindings::ImGuiTableFlags__ImGuiTableFlags_SizingFixedSame as isize,
  SizingStretchProp = bindings::ImGuiTableFlags__ImGuiTableFlags_SizingStretchProp as isize,
  SizingStretchSame = bindings::ImGuiTableFlags__ImGuiTableFlags_SizingStretchSame as isize,
  NoHostExtendX = bindings::ImGuiTableFlags__ImGuiTableFlags_NoHostExtendX as isize,
  NoHostExtendY = bindings::ImGuiTableFlags__ImGuiTableFlags_NoHostExtendY as isize,
  NoKeepColumnsVisible = bindings::ImGuiTableFlags__ImGuiTableFlags_NoKeepColumnsVisible as isize,
  PreciseWidths = bindings::ImGuiTableFlags__ImGuiTableFlags_PreciseWidths as isize,
  NoClip = bindings::ImGuiTableFlags__ImGuiTableFlags_NoClip as isize,
  PadOuterX = bindings::ImGuiTableFlags__ImGuiTableFlags_PadOuterX as isize,
  NoPadOuterX = bindings::ImGuiTableFlags__ImGuiTableFlags_NoPadOuterX as isize,
  NoPadInnerX = bindings::ImGuiTableFlags__ImGuiTableFlags_NoPadInnerX as isize,
  ScrollX = bindings::ImGuiTableFlags__ImGuiTableFlags_ScrollX as isize,
  ScrollY = bindings::ImGuiTableFlags__ImGuiTableFlags_ScrollY as isize,
  SortMulti = bindings::ImGuiTableFlags__ImGuiTableFlags_SortMulti as isize,
  SortTristate = bindings::ImGuiTableFlags__ImGuiTableFlags_SortTristate as isize,
  HighlightHoveredColumn = bindings::ImGuiTableFlags__ImGuiTableFlags_HighlightHoveredColumn as isize,
  SizingMask_ = bindings::ImGuiTableFlags__ImGuiTableFlags_SizingMask_ as isize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableRowFlags {
  None = bindings::ImGuiTableRowFlags__ImGuiTableRowFlags_None as isize,
  Headers = bindings::ImGuiTableRowFlags__ImGuiTableRowFlags_Headers as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputTextFlags {
  None = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_None as isize,
  CharsDecimal = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CharsDecimal as isize,
  CharsHexadecimal = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CharsHexadecimal as isize,
  CharsScientific = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CharsScientific as isize,
  CharsUppercase = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CharsUppercase as isize,
  CharsNoBlank = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CharsNoBlank as isize,
  AllowTabInput = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_AllowTabInput as isize,
  EnterReturnsTrue = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_EnterReturnsTrue as isize,
  EscapeClearsAll = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_EscapeClearsAll as isize,
  CtrlEnterForNewLine = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CtrlEnterForNewLine as isize,
  ReadOnly = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_ReadOnly as isize,
  Password = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_Password as isize,
  AlwaysOverwrite = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_AlwaysOverwrite as isize,
  AutoSelectAll = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_AutoSelectAll as isize,
  ParseEmptyRefVal = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_ParseEmptyRefVal as isize,
  DisplayEmptyRefVal = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_DisplayEmptyRefVal as isize,
  NoHorizontalScroll = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_NoHorizontalScroll as isize,
  NoUndoRedo = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_NoUndoRedo as isize,
  ElideLeft = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_ElideLeft as isize,
  CallbackCompletion = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CallbackCompletion as isize,
  CallbackHistory = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CallbackHistory as isize,
  CallbackAlways = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CallbackAlways as isize,
  CallbackCharFilter = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CallbackCharFilter as isize,
  CallbackResize = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CallbackResize as isize,
  CallbackEdit = bindings::ImGuiInputTextFlags__ImGuiInputTextFlags_CallbackEdit as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderFlags {
  None = bindings::ImGuiSliderFlags__ImGuiSliderFlags_None as isize,
  Logarithmic = bindings::ImGuiSliderFlags__ImGuiSliderFlags_Logarithmic as isize,
  NoRoundToFormat = bindings::ImGuiSliderFlags__ImGuiSliderFlags_NoRoundToFormat as isize,
  NoInput = bindings::ImGuiSliderFlags__ImGuiSliderFlags_NoInput as isize,
  WrapAround = bindings::ImGuiSliderFlags__ImGuiSliderFlags_WrapAround as isize,
  ClampOnInput = bindings::ImGuiSliderFlags__ImGuiSliderFlags_ClampOnInput as isize,
  ClampZeroRange = bindings::ImGuiSliderFlags__ImGuiSliderFlags_ClampZeroRange as isize,
  NoSpeedTweaks = bindings::ImGuiSliderFlags__ImGuiSliderFlags_NoSpeedTweaks as isize,
  AlwaysClamp = bindings::ImGuiSliderFlags__ImGuiSliderFlags_AlwaysClamp as isize,
  InvalidMask_ = bindings::ImGuiSliderFlags__ImGuiSliderFlags_InvalidMask_ as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorEditFlags {
  None = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_None as isize,
  NoAlpha = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoAlpha as isize,
  NoPicker = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoPicker as isize,
  NoOptions = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoOptions as isize,
  NoSmallPreview = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoSmallPreview as isize,
  NoInputs = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoInputs as isize,
  NoTooltip = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoTooltip as isize,
  NoLabel = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoLabel as isize,
  NoSidePreview = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoSidePreview as isize,
  NoDragDrop = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoDragDrop as isize,
  NoBorder = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_NoBorder as isize,
  AlphaOpaque = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_AlphaOpaque as isize,
  AlphaNoBg = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_AlphaNoBg as isize,
  AlphaPreviewHal = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_AlphaPreviewHalf as isize,
  AlphaBar = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_AlphaBar as isize,
  HDR = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_HDR as isize,
  DisplayRGB = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_DisplayRGB as isize,
  DisplayHSV = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_DisplayHSV as isize,
  DisplayHex = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_DisplayHex as isize,
  Uint8 = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_Uint8 as isize,
  Float = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_Float as isize,
  PickerHueBar = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_PickerHueBar as isize,
  PickerHueWheel = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_PickerHueWheel as isize,
  InputRGB = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_InputRGB as isize,
  InputHSV = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_InputHSV as isize,
  DefaultOptions_ = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_DefaultOptions_ as isize,
  AlphaMask_ = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_AlphaMask_ as isize,
  DisplayMask_ = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_DisplayMask_ as isize,
  DataTypeMask_ = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_DataTypeMask_ as isize,
  PickerMask_ = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_PickerMask_ as isize,
  InputMask_ = bindings::ImGuiColorEditFlags__ImGuiColorEditFlags_InputMask_ as isize,
}



impl_gui_flag!(FocusedFlags, None);
impl_gui_flag!(HoveredFlags, None);

impl_gui_flag!(WindowFlags, None);
impl_gui_flag!(ChildFlags, None);
impl_gui_flag!(PopupFlags, MouseButtonLeft);

impl_gui_flag!(TabBarFlags, None);
impl_gui_flag!(TabItemFlags, None);

impl_gui_flag!(TableColumnFlags, None);
impl_gui_flag!(TableFlags, None);
impl_gui_flag!(TableRowFlags, None);

impl_gui_flag!(InputTextFlags, None);
impl_gui_flag!(SliderFlags, None);
impl_gui_flag!(ColorEditFlags, None);







#![allow(non_upper_case_globals)]

// ALL CONTENTS OF THIS FILE ARE DIRECTLY COPIED FROM THE 
// IMGUI REPOSITORY IN ORDER TO PROVIDE RUST BINDINGS
// ALL CONTENTS OF THIS FILE HAVE BEEN SLIGHTLY MODIFIED
// TO FIT THE RUST PROGRAMMING LANGUAGE

// UNDER NO CIRCUMSTANCES DO I A CLAIM THE CODE IN THIS
// FILE TO BE OF MY OWN

/* 
 * window flags
 */
pub const ImGuiWindowFlags_None: i32                      = 0;
pub const ImGuiWindowFlags_NoTitleBar: i32                = 1 << 0;   // Disable title-bar
pub const ImGuiWindowFlags_NoResize: i32                  = 1 << 1;   // Disable user resizing with the lower-right grip
pub const ImGuiWindowFlags_NoMove: i32                    = 1 << 2;   // Disable user moving the window
pub const ImGuiWindowFlags_NoScrollbar: i32               = 1 << 3;   // Disable scrollbars (window can still scroll with mouse or programmatically)
pub const ImGuiWindowFlags_NoScrollWithMouse: i32         = 1 << 4;   // Disable user vertically scrolling with mouse wheel. On child window, mouse wheel will be forwarded to the parent unless NoScrollbar is also set.
pub const ImGuiWindowFlags_NoCollapse: i32                = 1 << 5;   // Disable user collapsing window by double-clicking on it. Also referred to as Window Menu Button (e.g. within a docking node).
pub const ImGuiWindowFlags_AlwaysAutoResize: i32          = 1 << 6;   // Resize every window to its content every frame
pub const ImGuiWindowFlags_NoBackground: i32              = 1 << 7;   // Disable drawing background color (WindowBg, etc.) and outside border. Similar as using SetNextWindowBgAlpha(0.0f).
pub const ImGuiWindowFlags_NoSavedSettings: i32           = 1 << 8;   // Never load/save settings in .ini file
pub const ImGuiWindowFlags_NoMouseInputs: i32             = 1 << 9;   // Disable catching mouse, hovering test with pass through.
pub const ImGuiWindowFlags_MenuBar: i32                   = 1 << 10;  // Has a menu-bar
pub const ImGuiWindowFlags_HorizontalScrollbar: i32       = 1 << 11;  // Allow horizontal scrollbar to appear (off by default). You may use SetNextWindowContentSize(ImVec2(width,0.0f)); prior to calling Begin() to specify width. Read code in imgui_demo in the "Horizontal Scrolling" section.
pub const ImGuiWindowFlags_NoFocusOnAppearing: i32        = 1 << 12;  // Disable taking focus when transitioning from hidden to visible state
pub const ImGuiWindowFlags_NoBringToFrontOnFocus: i32     = 1 << 13;  // Disable bringing window to front when taking focus (e.g. clicking on it or programmatically giving it focus)
pub const ImGuiWindowFlags_AlwaysVerticalScrollbar: i32   = 1 << 14;  // Always show vertical scrollbar (even if ContentSize.y < Size.y)
pub const ImGuiWindowFlags_AlwaysHorizontalScrollbar: i32 = 1 << 15;  // Always show horizontal scrollbar (even if ContentSize.x < Size.x)
pub const ImGuiWindowFlags_NoNavInputs: i32               = 1 << 16;  // No gamepad/keyboard navigation within the window
pub const ImGuiWindowFlags_NoNavFocus: i32                = 1 << 17;  // No focusing toward this window with gamepad/keyboard navigation (e.g. skipped by CTRL+TAB)
pub const ImGuiWindowFlags_UnsavedDocument: i32           = 1 << 18;  // Display a dot next to the title. When used in a tab/docking context, tab is selected when clicking the X + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the X, so if you keep submitting the tab may reappear at end of tab bar.
pub const ImGuiWindowFlags_NoDocking: i32                 = 1 << 19;  // Disable docking of this window
pub const ImGuiWindowFlags_NoNav: i32                     = ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus;
pub const ImGuiWindowFlags_NoDecoration: i32              = ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoCollapse;
pub const ImGuiWindowFlags_NoInputs: i32                  = ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus;

/*
 * child flags
 */
pub const ImGuiChildFlags_None: i32                       = 0;
pub const ImGuiChildFlags_Border: i32                     = 1 << 0;   // Show an outer border and enable WindowPadding. (IMPORTANT: this is always == 1 == true for legacy reason)
pub const ImGuiChildFlags_AlwaysUseWindowPadding: i32     = 1 << 1;   // Pad with style.WindowPadding even if no border are drawn (no padding by default for non-bordered child windows because it makes more sense)
pub const ImGuiChildFlags_ResizeX: i32                    = 1 << 2;   // Allow resize from right border (layout direction). Enable .ini saving (unless ImGuiWindowFlags_NoSavedSettings passed to window flags)
pub const ImGuiChildFlags_ResizeY: i32                    = 1 << 3;   // Allow resize from bottom border (layout direction). "
pub const ImGuiChildFlags_AutoResizeX: i32                = 1 << 4;   // Enable auto-resizing width. Read "IMPORTANT: Size measurement" details above.
pub const ImGuiChildFlags_AutoResizeY: i32                = 1 << 5;   // Enable auto-resizing height. Read "IMPORTANT: Size measurement" details above.
pub const ImGuiChildFlags_AlwaysAutoResize: i32           = 1 << 6;   // Combined with AutoResizeX/AutoResizeY. Always measure size even when child is hidden, always return true, always disable clipping optimization! NOT RECOMMENDED.
pub const ImGuiChildFlags_FrameStyle: i32                 = 1 << 7;   // Style the child window like a framed item: use FrameBg, FrameRounding, FrameBorderSize, FramePadding instead of ChildBg, ChildRounding, ChildBorderSize, WindowPadding.

/*
 * column flags
 */
// Input configuration flags
pub const ImGuiTableColumnFlags_None: i32                 = 0;
pub const ImGuiTableColumnFlags_Disabled: i32             = 1 << 0;   // Overriding/master disable flag: hide column, won't show in context menu (unlike calling TableSetColumnEnabled() which manipulates the user accessible state)
pub const ImGuiTableColumnFlags_DefaultHide: i32          = 1 << 1;   // Default as a hidden/disabled column.
pub const ImGuiTableColumnFlags_DefaultSort: i32          = 1 << 2;   // Default as a sorting column.
pub const ImGuiTableColumnFlags_WidthStretch: i32         = 1 << 3;   // Column will stretch. Preferable with horizontal scrolling disabled (default if table sizing policy is _SizingStretchSame or _SizingStretchProp).
pub const ImGuiTableColumnFlags_WidthFixed: i32           = 1 << 4;   // Column will not stretch. Preferable with horizontal scrolling enabled (default if table sizing policy is _SizingFixedFit and table is resizable).
pub const ImGuiTableColumnFlags_NoResize: i32             = 1 << 5;   // Disable manual resizing.
pub const ImGuiTableColumnFlags_NoReorder: i32            = 1 << 6;   // Disable manual reordering this column, this will also prevent other columns from crossing over this column.
pub const ImGuiTableColumnFlags_NoHide: i32               = 1 << 7;   // Disable ability to hide/disable this column.
pub const ImGuiTableColumnFlags_NoClip: i32               = 1 << 8;   // Disable clipping for this column (all NoClip columns will render in a same draw command).
pub const ImGuiTableColumnFlags_NoSort: i32               = 1 << 9;   // Disable ability to sort on this field (even if ImGuiTableFlags_Sortable is set on the table).
pub const ImGuiTableColumnFlags_NoSortAscending: i32      = 1 << 10;  // Disable ability to sort in the ascending direction.
pub const ImGuiTableColumnFlags_NoSortDescending: i32     = 1 << 11;  // Disable ability to sort in the descending direction.
pub const ImGuiTableColumnFlags_NoHeaderLabel: i32        = 1 << 12;  // TableHeadersRow() will not submit horizontal label for this column. Convenient for some small columns. Name will still appear in context menu or in angled headers.
pub const ImGuiTableColumnFlags_NoHeaderWidth: i32        = 1 << 13;  // Disable header text width contribution to automatic column width.
pub const ImGuiTableColumnFlags_PreferSortAscending: i32  = 1 << 14;  // Make the initial sort direction Ascending when first sorting on this column (default).
pub const ImGuiTableColumnFlags_PreferSortDescending: i32 = 1 << 15;  // Make the initial sort direction Descending when first sorting on this column.
pub const ImGuiTableColumnFlags_IndentEnable: i32         = 1 << 16;  // Use current Indent value when entering cell (default for column 0).
pub const ImGuiTableColumnFlags_IndentDisable: i32        = 1 << 17;  // Ignore current Indent value when entering cell (default for columns > 0). Indentation changes _within_ the cell will still be honored.
pub const ImGuiTableColumnFlags_AngledHeader: i32         = 1 << 18;  // TableHeadersRow() will submit an angled header row for this column. Note this will add an extra row.

// Output status flags, read-only via TableGetColumnFlags()
pub const ImGuiTableColumnFlags_IsEnabled: i32            = 1 << 24;  // Status: is enabled == not hidden by user/api (referred to as "Hide" in _DefaultHide and _NoHide) flags.
pub const ImGuiTableColumnFlags_IsVisible: i32            = 1 << 25;  // Status: is visible == is enabled AND not clipped by scrolling.
pub const ImGuiTableColumnFlags_IsSorted: i32             = 1 << 26;  // Status: is currently part of the sort specs
pub const ImGuiTableColumnFlags_IsHovered: i32            = 1 << 27;  // Status: is hovered by mouse

// [Internal] Combinations and masks
pub const ImGuiTableColumnFlags_WidthMask_: i32           = ImGuiTableColumnFlags_WidthStretch | ImGuiTableColumnFlags_WidthFixed;
pub const ImGuiTableColumnFlags_IndentMask_: i32          = ImGuiTableColumnFlags_IndentEnable | ImGuiTableColumnFlags_IndentDisable;
pub const ImGuiTableColumnFlags_StatusMask_: i32          = ImGuiTableColumnFlags_IsEnabled | ImGuiTableColumnFlags_IsVisible | ImGuiTableColumnFlags_IsSorted | ImGuiTableColumnFlags_IsHovered;
pub const ImGuiTableColumnFlags_NoDirectResize_: i32      = 1 << 30;  // [Internal] Disable user resizing this column directly (it may however we resized indirectly from its left edge)

/*
 * table flags
 */
// Features
pub const ImGuiTableFlags_None: i32                       = 0;
pub const ImGuiTableFlags_Resizable: i32                  = 1 << 0;   // Enable resizing columns.
pub const ImGuiTableFlags_Reorderable: i32                = 1 << 1;   // Enable reordering columns in header row (need calling TableSetupColumn() + TableHeadersRow() to display headers)
pub const ImGuiTableFlags_Hideable: i32                   = 1 << 2;   // Enable hiding/disabling columns in context menu.
pub const ImGuiTableFlags_Sortable: i32                   = 1 << 3;   // Enable sorting. Call TableGetSortSpecs() to obtain sort specs. Also see ImGuiTableFlags_SortMulti and ImGuiTableFlags_SortTristate.
pub const ImGuiTableFlags_NoSavedSettings: i32            = 1 << 4;   // Disable persisting columns order, width and sort settings in the .ini file.
pub const ImGuiTableFlags_ContextMenuInBody: i32          = 1 << 5;   // Right-click on columns body/contents will display table context menu. By default it is available in TableHeadersRow().
// Decorations
pub const ImGuiTableFlags_RowBg: i32                      = 1 << 6;   // Set each RowBg color with ImGuiCol_TableRowBg or ImGuiCol_TableRowBgAlt (equivalent of calling TableSetBgColor with ImGuiTableBgFlags_RowBg0 on each row manually)
pub const ImGuiTableFlags_BordersInnerH: i32              = 1 << 7;   // Draw horizontal borders between rows.
pub const ImGuiTableFlags_BordersOuterH: i32              = 1 << 8;   // Draw horizontal borders at the top and bottom.
pub const ImGuiTableFlags_BordersInnerV: i32              = 1 << 9;   // Draw vertical borders between columns.
pub const ImGuiTableFlags_BordersOuterV: i32              = 1 << 10;  // Draw vertical borders on the left and right sides.
pub const ImGuiTableFlags_BordersH: i32                   = ImGuiTableFlags_BordersInnerH | ImGuiTableFlags_BordersOuterH; // Draw horizontal borders.
pub const ImGuiTableFlags_BordersV: i32                   = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersOuterV; // Draw vertical borders.
pub const ImGuiTableFlags_BordersInner: i32               = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersInnerH; // Draw inner borders.
pub const ImGuiTableFlags_BordersOuter: i32               = ImGuiTableFlags_BordersOuterV | ImGuiTableFlags_BordersOuterH; // Draw outer borders.
pub const ImGuiTableFlags_Borders: i32                    = ImGuiTableFlags_BordersInner | ImGuiTableFlags_BordersOuter;   // Draw all borders.
pub const ImGuiTableFlags_NoBordersInBody: i32            = 1 << 11;  // [ALPHA] Disable vertical borders in columns Body (borders will always appear in Headers). -> May move to style
pub const ImGuiTableFlags_NoBordersInBodyUntilResize: i32 = 1 << 12;  // [ALPHA] Disable vertical borders in columns Body until hovered for resize (borders will always appear in Headers). -> May move to style
// Sizing Policy (read above for defaults)
pub const ImGuiTableFlags_SizingFixedFit: i32             = 1 << 13;  // Columns default to _WidthFixed or _WidthAuto (if resizable or not resizable), matching contents width.
pub const ImGuiTableFlags_SizingFixedSame: i32            = 2 << 13;  // Columns default to _WidthFixed or _WidthAuto (if resizable or not resizable), matching the maximum contents width of all columns. Implicitly enable ImGuiTableFlags_NoKeepColumnsVisible.
pub const ImGuiTableFlags_SizingStretchProp: i32          = 3 << 13;  // Columns default to _WidthStretch with default weights proportional to each columns contents widths.
pub const ImGuiTableFlags_SizingStretchSame: i32          = 4 << 13;  // Columns default to _WidthStretch with default weights all equal, unless overridden by TableSetupColumn().
// Sizing Extra Options
pub const ImGuiTableFlags_NoHostExtendX: i32              = 1 << 16;  // Make outer width auto-fit to columns, overriding outer_size.x value. Only available when ScrollX/ScrollY are disabled and Stretch columns are not used.
pub const ImGuiTableFlags_NoHostExtendY: i32              = 1 << 17;  // Make outer height stop exactly at outer_size.y (prevent auto-extending table past the limit). Only available when ScrollX/ScrollY are disabled. Data below the limit will be clipped and not visible.
pub const ImGuiTableFlags_NoKeepColumnsVisible: i32       = 1 << 18;  // Disable keeping column always minimally visible when ScrollX is off and table gets too small. Not recommended if columns are resizable.
pub const ImGuiTableFlags_PreciseWidths: i32              = 1 << 19;  // Disable distributing remainder width to stretched columns (width allocation on a 100-wide table with 3 columns: Without this flag: 33,33,34. With this flag: 33,33,33). With larger number of columns, resizing will appear to be less smooth.
// Clipping
pub const ImGuiTableFlags_NoClip: i32                     = 1 << 20;  // Disable clipping rectangle for every individual columns (reduce draw command count, items will be able to overflow into other columns). Generally incompatible with TableSetupScrollFreeze().
// Padding
pub const ImGuiTableFlags_PadOuterX: i32                  = 1 << 21;  // Default if BordersOuterV is on. Enable outermost padding. Generally desirable if you have headers.
pub const ImGuiTableFlags_NoPadOuterX: i32                = 1 << 22;  // Default if BordersOuterV is off. Disable outermost padding.
pub const ImGuiTableFlags_NoPadInnerX: i32                = 1 << 23;  // Disable inner padding between columns (double inner padding if BordersOuterV is on, single inner padding if BordersOuterV is off).
// Scrolling
pub const ImGuiTableFlags_ScrollX: i32                    = 1 << 24;  // Enable horizontal scrolling. Require 'outer_size' parameter of BeginTable() to specify the container size. Changes default sizing policy. Because this creates a child window, ScrollY is currently generally recommended when using ScrollX.
pub const ImGuiTableFlags_ScrollY: i32                    = 1 << 25;  // Enable vertical scrolling. Require 'outer_size' parameter of BeginTable() to specify the container size.
// Sorting
pub const ImGuiTableFlags_SortMulti: i32                  = 1 << 26;  // Hold shift when clicking headers to sort on multiple column. TableGetSortSpecs() may return specs where (SpecsCount > 1).
pub const ImGuiTableFlags_SortTristate: i32               = 1 << 27;  // Allow no sorting, disable default sorting. TableGetSortSpecs() may return specs where (SpecsCount == 0).
// Miscellaneous
pub const ImGuiTableFlags_HighlightHoveredColumn: i32     = 1 << 28;  // Highlight column headers when hovered (may evolve into a fuller highlight)
// [Internal] Combinations and masks
pub const ImGuiTableFlags_SizingMask_: i32                = ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_SizingFixedSame | ImGuiTableFlags_SizingStretchProp | ImGuiTableFlags_SizingStretchSame;

// row flags 
pub const ImGuiTableRowFlags_None: i32                    = 0;
pub const ImGuiTableRowFlags_Headers: i32                 = 1 << 0;   // Identify header row (set default background color + width of its contents accounted differently for auto column width)


/*
 * style vars
 */
pub enum ImGuiStyleVar {
  Alpha = 0,                // float    Alpha
  DisabledAlpha,            // float    DisabledAlpha
  WindowPadding,            // ImVec2   WindowPadding
  WindowRounding,           // float    WindowRounding
  WindowBorderSize,         // float    WindowBorderSize
  WindowMinSize,            // ImVec2   WindowMinSize
  WindowTitleAlign,         // ImVec2   WindowTitleAlign
  ChildRounding,            // float    ChildRounding
  ChildBorderSize,          // float    ChildBorderSize
  PopupRounding,            // float    PopupRounding
  PopupBorderSize,          // float    PopupBorderSize
  FramePadding,             // ImVec2   FramePadding
  FrameRounding,            // float    FrameRounding
  FrameBorderSize,          // float    FrameBorderSize
  ItemSpacing,              // ImVec2   ItemSpacing
  ItemInnerSpacing,         // ImVec2   ItemInnerSpacing
  IndentSpacing,            // float    IndentSpacing
  CellPadding,              // ImVec2   CellPadding
  ScrollbarSize,            // float    ScrollbarSize
  ScrollbarRounding,        // float    ScrollbarRounding
  GrabMinSize,              // float    GrabMinSize
  GrabRounding,             // float    GrabRounding
  TabRounding,              // float    TabRounding
  TabBorderSize,            // float    TabBorderSize
  TabBarBorderSize,         // float    TabBarBorderSize
  TableAngledHeadersAngle,  // float    TableAngledHeadersAngle
  ButtonTextAlign,          // ImVec2   ButtonTextAlign
  SelectableTextAlign,      // ImVec2   SelectableTextAlign
  SeparatorTextBorderSize,  // float    SeparatorTextBorderSize
  SeparatorTextAlign,       // ImVec2   SeparatorTextAlign
  SeparatorTextPadding,     // ImVec2   SeparatorTextPadding
  DockingSeparatorSize,     // float    DockingSeparatorSize
  COUNT,
}

/*
 * input text flags
 */
pub const ImGuiInputTextFlags_None: i32                = 0;
pub const ImGuiInputTextFlags_CharsDecimal: i32        = 1 << 0;   // Allow 0123456789.+-*/
pub const ImGuiInputTextFlags_CharsHexadecimal: i32    = 1 << 1;   // Allow 0123456789ABCDEFabcdef
pub const ImGuiInputTextFlags_CharsUppercase: i32      = 1 << 2;   // Turn a..z into A..Z
pub const ImGuiInputTextFlags_CharsNoBlank: i32        = 1 << 3;   // Filter out spaces, tabs
pub const ImGuiInputTextFlags_AutoSelectAll: i32       = 1 << 4;   // Select entire text when first taking mouse focus
pub const ImGuiInputTextFlags_EnterReturnsTrue: i32    = 1 << 5;   // Return 'true' when Enter is pressed (as opposed to every time the value was modified). Consider looking at the IsItemDeactivatedAfterEdit() function.
pub const ImGuiInputTextFlags_CallbackCompletion: i32  = 1 << 6;   // Callback on pressing TAB (for completion handling)
pub const ImGuiInputTextFlags_CallbackHistory: i32     = 1 << 7;   // Callback on pressing Up/Down arrows (for history handling)
pub const ImGuiInputTextFlags_CallbackAlways: i32      = 1 << 8;   // Callback on each iteration. User code may query cursor position, modify text buffer.
pub const ImGuiInputTextFlags_CallbackCharFilter: i32  = 1 << 9;   // Callback on character inputs to replace or discard them. Modify 'EventChar' to replace or discard, or return 1 in callback to discard.
pub const ImGuiInputTextFlags_AllowTabInput: i32       = 1 << 10;  // Pressing TAB input a '\t' character into the text field
pub const ImGuiInputTextFlags_CtrlEnterForNewLine: i32 = 1 << 11;  // In multi-line mode, unfocus with Enter, add new line with Ctrl+Enter (default is opposite: unfocus with Ctrl+Enter, add line with Enter).
pub const ImGuiInputTextFlags_NoHorizontalScroll: i32  = 1 << 12;  // Disable following the cursor horizontally
pub const ImGuiInputTextFlags_AlwaysOverwrite: i32     = 1 << 13;  // Overwrite mode
pub const ImGuiInputTextFlags_ReadOnly: i32            = 1 << 14;  // Read-only mode
pub const ImGuiInputTextFlags_Password: i32            = 1 << 15;  // Password mode, display all characters as '*'
pub const ImGuiInputTextFlags_NoUndoRedo: i32          = 1 << 16;  // Disable undo/redo. Note that input text owns the text data while active, if you want to provide your own undo/redo stack you need e.g. to call ClearActiveID().
pub const ImGuiInputTextFlags_CharsScientific: i32     = 1 << 17;  // Allow 0123456789.+-*/eE (Scientific notation input)
pub const ImGuiInputTextFlags_CallbackResize: i32      = 1 << 18;  // Callback on buffer capacity changes request (beyond 'buf_size' parameter value), allowing the string to grow. Notify when the string wants to be resized (for string types which hold a cache of their Size). You will be provided a new BufSize in the callback and NEED to honor it. (see misc/cpp/imgui_stdlib.h for an example of using this)
pub const ImGuiInputTextFlags_CallbackEdit: i32        = 1 << 19;  // Callback on any edit (note that InputText() already returns true on edit, the callback is useful mainly to manipulate the underlying buffer while focus is active)
pub const ImGuiInputTextFlags_EscapeClearsAll: i32     = 1 << 20;  // Escape key clears content if not empty, and deactivate otherwise (contrast to default behavior of Escape to revert)

/*
 * popup flags
 */
pub const ImGuiPopupFlags_None: i32                    = 0;
pub const ImGuiPopupFlags_MouseButtonLeft: i32         = 0;        // For BeginPopupContext*(): open on Left Mouse release. Guaranteed to always be == 0 (same as ImGuiMouseButton_Left)
pub const ImGuiPopupFlags_MouseButtonRight: i32        = 1;        // For BeginPopupContext*(): open on Right Mouse release. Guaranteed to always be == 1 (same as ImGuiMouseButton_Right)
pub const ImGuiPopupFlags_MouseButtonMiddle: i32       = 2;        // For BeginPopupContext*(): open on Middle Mouse release. Guaranteed to always be == 2 (same as ImGuiMouseButton_Middle)
pub const ImGuiPopupFlags_MouseButtonMask_: i32        = 0x1F;
pub const ImGuiPopupFlags_MouseButtonDefault_: i32     = 1;
pub const ImGuiPopupFlags_NoReopen: i32                = 1 << 5;   // For OpenPopup*(), BeginPopupContext*(): don't reopen same popup if already open (won't reposition, won't reinitialize navigation)
//pub const ImGuiPopupFlags_NoReopenAlwaysNavInit: i32   = 1 << 6;   // For OpenPopup*(), BeginPopupContext*(): focus and initialize navigation even when not reopening.
pub const ImGuiPopupFlags_NoOpenOverExistingPopup: i32 = 1 << 7;   // For OpenPopup*(), BeginPopupContext*(): don't open if there's already a popup at the same level of the popup stack
pub const ImGuiPopupFlags_NoOpenOverItems: i32         = 1 << 8;   // For BeginPopupContextWindow(): don't return true when hovering items, only when hovering empty space
pub const ImGuiPopupFlags_AnyPopupId: i32              = 1 << 10;  // For IsPopupOpen(): ignore the ImGuiID parameter and test for any popup.
pub const ImGuiPopupFlags_AnyPopupLevel: i32           = 1 << 11;  // For IsPopupOpen(): search/test at any level of the popup stack (default test in the current level)
pub const ImGuiPopupFlags_AnyPopup: i32                = ImGuiPopupFlags_AnyPopupId | ImGuiPopupFlags_AnyPopupLevel;

/*
 * style color
 */
pub enum ImGuiCol {
  Text = 0,
  TextDisabled,
  WindowBg,              // Background of normal windows
  ChildBg,               // Background of child windows
  PopupBg,               // Background of popups, menus, tooltips windows
  Border,
  BorderShadow,
  FrameBg,               // Background of checkbox, radio button, plot, slider, text input
  FrameBgHovered,
  FrameBgActive,
  TitleBg,               // Title bar
  TitleBgActive,         // Title bar when focused
  TitleBgCollapsed,      // Title bar when collapsed
  MenuBarBg,
  ScrollbarBg,
  ScrollbarGrab,
  ScrollbarGrabHovered,
  ScrollbarGrabActive,
  CheckMark,             // Checkbox tick and RadioButton circle
  SliderGrab,
  SliderGrabActive,
  Button,
  ButtonHovered,
  ButtonActive,
  Header,                // Header* colors are used for CollapsingHeader, TreeNode, Selectable, MenuItem
  HeaderHovered,
  HeaderActive,
  Separator,
  SeparatorHovered,
  SeparatorActive,
  ResizeGrip,            // Resize grip in lower-right and lower-left corners of windows.
  ResizeGripHovered,
  ResizeGripActive,
  Tab,                   // TabItem in a TabBar
  TabHovered,
  TabActive,
  TabUnfocused,
  TabUnfocusedActive,
  DockingPreview,        // Preview overlay color when about to docking something
  DockingEmptyBg,        // Background color for empty node (e.g. CentralNode with no window docked into it)
  PlotLines,
  PlotLinesHovered,
  PlotHistogram,
  PlotHistogramHovered,
  TableHeaderBg,         // Table header background
  TableBorderStrong,     // Table outer and header borders (prefer using Alpha=1.0 here)
  TableBorderLight,      // Table inner borders (prefer using Alpha=1.0 here)
  TableRowBg,            // Table row background (even rows)
  TableRowBgAlt,         // Table row background (odd rows)
  TextSelectedBg,
  DragDropTarget,        // Rectangle highlighting a drop target
  NavHighlight,          // Gamepad/keyboard: current highlighted item
  NavWindowingHighlight, // Highlight window when using CTRL+TAB
  NavWindowingDimBg,     // Darken/colorize entire screen behind the CTRL+TAB window list, when active
  ModalWindowDimBg,      // Darken/colorize entire screen behind a modal window, when one is active
  COUNT,
}

/*
 * tab bars
 */
pub const ImGuiTabBarFlags_None: i32                          = 0;
pub const ImGuiTabBarFlags_Reorderable: i32                   = 1 << 0;   // Allow manually dragging tabs to re-order them + New tabs are appended at the end of list
pub const ImGuiTabBarFlags_AutoSelectNewTabs: i32             = 1 << 1;   // Automatically select new tabs when they appear
pub const ImGuiTabBarFlags_TabListPopupButton: i32            = 1 << 2;   // Disable buttons to open the tab list popup
pub const ImGuiTabBarFlags_NoCloseWithMiddleMouseButton: i32  = 1 << 3;   // Disable behavior of closing tabs (that are submitted with p_open != NULL) with middle mouse button. You may handle this behavior manually on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
pub const ImGuiTabBarFlags_NoTabListScrollingButtons: i32     = 1 << 4;   // Disable scrolling buttons (apply when fitting policy is ImGuiTabBarFlags_FittingPolicyScroll)
pub const ImGuiTabBarFlags_NoTooltip: i32                     = 1 << 5;   // Disable tooltips when hovering a tab
pub const ImGuiTabBarFlags_FittingPolicyResizeDown: i32       = 1 << 6;   // Resize tabs when they don't fit
pub const ImGuiTabBarFlags_FittingPolicyScroll: i32           = 1 << 7;   // Add scroll buttons when tabs don't fit
pub const ImGuiTabBarFlags_FittingPolicyMask_: i32            = ImGuiTabBarFlags_FittingPolicyResizeDown | ImGuiTabBarFlags_FittingPolicyScroll;
pub const ImGuiTabBarFlags_FittingPolicyDefault_: i32         = ImGuiTabBarFlags_FittingPolicyResizeDown;

/*
 * tab items
 */
pub const ImGuiTabItemFlags_None: i32                         = 0;
pub const ImGuiTabItemFlags_UnsavedDocument: i32              = 1 << 0;   // Display a dot next to the title + set ImGuiTabItemFlags_NoAssumedClosure.
pub const ImGuiTabItemFlags_SetSelected: i32                  = 1 << 1;   // Trigger flag to programmatically make the tab selected when calling BeginTabItem()
pub const ImGuiTabItemFlags_NoCloseWithMiddleMouseButton: i32 = 1 << 2;   // Disable behavior of closing tabs (that are submitted with p_open != NULL) with middle mouse button. You may handle this behavior manually on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
pub const ImGuiTabItemFlags_NoPushId: i32                     = 1 << 3;   // Don't call PushID()/PopID() on BeginTabItem()/EndTabItem()
pub const ImGuiTabItemFlags_NoTooltip: i32                    = 1 << 4;   // Disable tooltip for the given tab
pub const ImGuiTabItemFlags_NoReorder: i32                    = 1 << 5;   // Disable reordering this tab or having another tab cross over this tab
pub const ImGuiTabItemFlags_Leading: i32                      = 1 << 6;   // Enforce the tab position to the left of the tab bar (after the tab list popup button)
pub const ImGuiTabItemFlags_Trailing: i32                     = 1 << 7;   // Enforce the tab position to the right of the tab bar (before the scrolling buttons)
pub const ImGuiTabItemFlags_NoAssumedClosure: i32             = 1 << 8;   // Tab is selected when trying to close + closure is not immediately assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the X, so if you keep submitting the tab may reappear at end of tab bar.


/* 
 * conditions 
 */
// Enumeration for ImGui::SetNextWindow***(), SetWindow***(), SetNextItem***() functions
pub enum ImGuiCond {
  None          = 0,        // No condition (always set the variable), same as _Always
  Always        = 1 << 0,   // No condition (always set the variable), same as _None
  Once          = 1 << 1,   // Set the variable once per runtime session (only the first call will succeed)
  FirstUseEver  = 1 << 2,   // Set the variable if the object/window has no persistently saved data (no entry in .ini file)
  Appearing     = 1 << 3,   // Set the variable if the object/window is appearing after being hidden/inactive (or the first time)
}





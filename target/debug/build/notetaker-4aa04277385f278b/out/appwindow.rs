# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] # [allow (clippy :: overly_complex_bool_expr)] mod slint_generatedAppWindow {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TextStyle {
         pub r#font_size : f32 , pub r#font_weight : i32 }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_54 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerColorSchemeSelector_54 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFluentPalette_55 {
         r#accent_control_border : sp :: Property < slint :: Brush > , r#control_border : sp :: Property < slint :: Brush > , r#text_control_border : sp :: Property < slint :: Brush > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerFluentPalette_55 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_55 :: FIELD_OFFSETS . r#accent_control_border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (352321535f64 as u32) , position : ((90.67f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (603979776f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ])) }
                     else {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (352321535f64 as u32) , position : ((90.67f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (1711276032f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ]))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_55 :: FIELD_OFFSETS . r#control_border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (402653183f64 as u32) , position : ((0f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (301989888f64 as u32) , position : ((8.33f64 as f64) * (0.01f64 as f64)) as _ }
                        ])) }
                     else {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (251658240f64 as u32) , position : ((90.58f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (687865856f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ]))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_55 :: FIELD_OFFSETS . r#text_control_border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (352321535f64 as u32) , position : ((99.98f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (2332033023f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (2332033023f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ])) }
                     else {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (251658240f64 as u32) , position : ((99.99f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (1929379840f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (1929379840f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ]))) as _ }
                    ) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_56 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerStyleMetrics_56 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFocusBorder_root_9 {
         r#root_9 : sp :: r#BasicBorderRectangle , r#rectangle_10 : sp :: r#BasicBorderRectangle , r#root_9_height : sp :: Property < sp :: LogicalLength > , r#root_9_width : sp :: Property < sp :: LogicalLength > , r#root_9_x : sp :: Property < sp :: LogicalLength > , r#root_9_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerFocusBorder_root_9 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFocusBorder_root_9 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#rectangle_10 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (3003121664f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#rectangle_10 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
                     + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . get () . get () as f64) - (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#rectangle_10 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#rectangle_10 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#rectangle_10 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_height }
                ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as sp :: Coord , ((({
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_width }
                ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as sp :: Coord , 2f64 as sp :: Coord , 2f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerButton_root_11 {
         r#root_11 : sp :: r#Empty , r#i_background_12 : sp :: r#BasicBorderRectangle , r#i_border_13 : sp :: r#BasicBorderRectangle , r#i_touch_area_19 : sp :: r#TouchArea , r#i_focus_scope_20 : sp :: r#FocusScope , r#root_11_checkable : sp :: Property < bool > , r#root_11_checked : sp :: Property < bool > , r#root_11_has_focus : sp :: Property < bool > , r#root_11_height : sp :: Property < sp :: LogicalLength > , r#root_11_i_background_12_width : sp :: Property < sp :: LogicalLength > , r#root_11_i_background_12_x : sp :: Property < sp :: LogicalLength > , r#root_11_i_background_12_y : sp :: Property < sp :: LogicalLength > , r#root_11_i_border_13_x : sp :: Property < sp :: LogicalLength > , r#root_11_i_border_13_y : sp :: Property < sp :: LogicalLength > , r#root_11_i_focus_scope_20_y : sp :: Property < sp :: LogicalLength > , r#root_11_i_layout_14_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_11_i_layout_14_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_11_i_layout_14_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_11_i_touch_area_19_x : sp :: Property < sp :: LogicalLength > , r#root_11_i_touch_area_19_y : sp :: Property < sp :: LogicalLength > , r#root_11_icon : sp :: Property < sp :: Image > , r#root_11_pressed : sp :: Property < bool > , r#root_11_state : sp :: Property < i32 > , r#root_11_text : sp :: Property < sp :: SharedString > , r#root_11_text_color : sp :: Property < slint :: Brush > , r#root_11_width : sp :: Property < sp :: LogicalLength > , r#root_11_x : sp :: Property < sp :: LogicalLength > , r#root_11_y : sp :: Property < sp :: LogicalLength > , r#root_11_clicked : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent_image_15 > , repeater1 : sp :: Repeater < InnerComponent_text_17 > , repeater2 : sp :: Repeater < InnerComponent_focusborder_21 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_11 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_11 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ({
                         let r#tmp_root_11_icon = sp :: Image :: default () ;
                         ;
                         (((((r#tmp_root_11_icon . clone () . size ()) . r#width as f64) > (0f64 as f64))) && ((((r#tmp_root_11_icon . clone () . size ()) . r#height as f64) > (0f64 as f64)))) }
                     as bool)) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
                    ) . apply_pin (_self) . get ()) != (sp :: SharedString :: from (""))) as bool)) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_has_focus }
                    ) . apply_pin (_self) . get ()) && (({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) as bool)) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way (({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_11 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_11 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 12f64 as _ ;
                                 the_struct . r#end = 12f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_11 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_11 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_11 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerButton_root_11 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 5f64 as _ ;
                             the_struct . r#end = 5f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_pressed }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) && (({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_pressed }
                        ) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (if ({
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                                 3f64 }
                             else {
                                 (if ({
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                                ) . apply_pin (_self) . get () {
                                     4f64 }
                                 else {
                                     (0f64) as _ }
                                ) as _ }
                            ) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_11_state = ({
                             * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_11_state . clone () as f64) == (1f64 as f64)) {
                             if ({
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                            ) . apply_pin (_self) . get () {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (2281701375f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                                )) as _ }
                             }
                         else {
                             (if ((r#tmp_root_11_state . clone () as f64) == (2f64 as f64)) {
                                 if ({
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                                ) . apply_pin (_self) . get () {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (2147483648f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                                    )) as _ }
                                 }
                             else {
                                 (if ((r#tmp_root_11_state . clone () as f64) == (4f64 as f64)) {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (if ({
                                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                                    ) . apply_pin (_self) . get () {
                                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                        ) }
                                     else {
                                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                                        )) as _ }
                                    ) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_background_12 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_11_state = ({
                             * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_11_state . clone () as f64) == (1f64 as f64)) {
                             if ({
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                            ) . apply_pin (_self) . get () {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (704643071f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (939524096f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (184549375f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1308228089f64 as u32)) as _ }
                                )) as _ }
                             }
                         else {
                             (if ((r#tmp_root_11_state . clone () as f64) == (2f64 as f64)) {
                                 if ({
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                                ) . apply_pin (_self) . get () {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (3428896255f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (3422576568f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (150994943f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (1308228089f64 as u32)) as _ }
                                    )) as _ }
                                 }
                             else {
                                 (if ((r#tmp_root_11_state . clone () as f64) == (3f64 as f64)) {
                                     if ({
                                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                                    ) . apply_pin (_self) . get () {
                                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (3865103871f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (3858784184f64 as u32)) as _ }
                                        ) }
                                     else {
                                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (2163866105f64 as u32)) as _ }
                                        )) as _ }
                                     }
                                 else {
                                     (if ((r#tmp_root_11_state . clone () as f64) == (4f64 as f64)) {
                                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (4284534271f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (4278214584f64 as u32)) as _ }
                                        ) }
                                     else {
                                         (if false {
                                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                                 sp :: Color :: from_argb_encoded (4284534271f64 as u32) }
                                             else {
                                                 (sp :: Color :: from_argb_encoded (4278214584f64 as u32)) as _ }
                                            ) }
                                         else {
                                             (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                                 sp :: Color :: from_argb_encoded (268435455f64 as u32) }
                                             else {
                                                 (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                                            )) as _ }
                                        ) as _ }
                                    ) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_background_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_11_state = ({
                             * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_11_state . clone () as f64) == (1f64 as f64)) {
                             if ({
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                            ) . apply_pin (_self) . get () {
                                 slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32)) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                                )) as _ }
                             }
                         else {
                             (if ((r#tmp_root_11_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                                ) }
                             else {
                                 (if ((r#tmp_root_11_state . clone () as f64) == (4f64 as f64)) {
                                     InnerFluentPalette_55 :: FIELD_OFFSETS . r#accent_control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_FluentPalette_55 . as_ref ()) . get () }
                                 else {
                                     (if false {
                                         InnerFluentPalette_55 :: FIELD_OFFSETS . r#accent_control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_FluentPalette_55 . as_ref ()) . get () }
                                     else {
                                         (InnerFluentPalette_55 :: FIELD_OFFSETS . r#control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_FluentPalette_55 . as_ref ()) . get ()) as _ }
                                    ) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ({
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checkable }
                            ) . apply_pin (_self) . get () {
                                 {
                                     ({
                                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                                    ) . apply_pin (_self) . set (! ({
                                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checked }
                                    ) . apply_pin (_self) . get () as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             ({
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_clicked }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ! (((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from (" ")))) || ((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\n"))))) {
                                     (true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((false , {
                                         ({
                                             * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& () . into ()) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     (false , {
                                         sp :: r#EventResult :: r#Reject }
                                     ,) }
                                 else {
                                     ((false , (r#return_check_merge0 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression0 . clone ()) . 0 {
                                 sp :: r#EventResult :: r#Reject }
                             else {
                                 ((r#returned_expression0 . clone ()) . 1) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_focus_scope_20_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_background_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_background_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_background_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_11 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_focus_scope_20_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
                ) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_15 {
         r#image_15 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_15 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_15 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             sp :: Property :: link_two_way (({
                 * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , (InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_icon) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ())) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if false {
                         (InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 20f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 20f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord , 20f64 as sp :: Coord , {
                     let cache = (InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_image_15 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_15 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_image_15 :: FIELD_OFFSETS . r#image_15 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_15) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_15 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_image_15 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_15 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_15 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_15 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_15 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_17 {
         r#text_17 : sp :: r#Text , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_17 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_17 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1.0766f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord , {
                     let cache = (InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , {
                     let cache = (InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => (InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_text_17 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_17 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_text_17 :: FIELD_OFFSETS . r#text_17 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_17) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_17 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_text_17 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_17 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_17 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_17 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_17 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_21 {
         r#focusborder_21 : InnerFocusBorder_root_9 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_focusborder_21 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_21 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_9 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_21 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             ({
                 InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
             + {
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
                 + {
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
                 + {
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
             + {
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
             + {
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
             + {
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
             + {
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
             + {
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_9 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_21 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
                 + {
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
                 + {
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
                 + {
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
                 + {
                     * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 ..= 1u32 => return {
                     * & Self :: FIELD_OFFSETS . r#focusborder_21 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_21 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_21 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_21 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_21 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_focusborder_21 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_11 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_focusborder_21 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
             + {
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#root_9 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_focusborder_21 :: FIELD_OFFSETS . r#focusborder_21 }
             + {
                 * & InnerFocusBorder_root_9 :: FIELD_OFFSETS . r#rectangle_10 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_focusborder_21) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_focusborder_21 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_focusborder_21 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_focusborder_21 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_21 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_focusborder_21 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_21 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerLineEditBase_root_23 {
         r#root_23 : sp :: r#Empty , r#root_clip_24 : sp :: r#Clip , r#i_placeholder_25 : sp :: r#Text , r#i_text_input_26 : sp :: r#TextInput , r#root_23_has_focus : sp :: Property < bool > , r#root_23_height : sp :: Property < sp :: LogicalLength > , r#root_23_i_placeholder_25_horizontal_stretch : sp :: Property < f32 > , r#root_23_i_placeholder_25_max_height : sp :: Property < sp :: LogicalLength > , r#root_23_i_placeholder_25_max_width : sp :: Property < sp :: LogicalLength > , r#root_23_i_placeholder_25_min_height : sp :: Property < sp :: LogicalLength > , r#root_23_i_placeholder_25_min_width : sp :: Property < sp :: LogicalLength > , r#root_23_i_placeholder_25_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_23_i_placeholder_25_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_23_i_placeholder_25_vertical_stretch : sp :: Property < f32 > , r#root_23_i_placeholder_25_x : sp :: Property < sp :: LogicalLength > , r#root_23_i_placeholder_25_y : sp :: Property < sp :: LogicalLength > , r#root_23_i_text_input_26_computed_x : sp :: Property < sp :: LogicalLength > , r#root_23_i_text_input_26_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_23_i_text_input_26_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_23_i_text_input_26_x : sp :: Property < sp :: LogicalLength > , r#root_23_i_text_input_26_y : sp :: Property < sp :: LogicalLength > , r#root_23_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_23_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_23_margin : sp :: Property < sp :: LogicalLength > , r#root_23_placeholder_color : sp :: Property < slint :: Brush > , r#root_23_placeholder_text : sp :: Property < sp :: SharedString > , r#root_23_root_clip_24_x : sp :: Property < sp :: LogicalLength > , r#root_23_root_clip_24_y : sp :: Property < sp :: LogicalLength > , r#root_23_text_color : sp :: Property < slint :: Brush > , r#root_23_width : sp :: Property < sp :: LogicalLength > , r#root_23_x : sp :: Property < sp :: LogicalLength > , r#root_23_y : sp :: Property < sp :: LogicalLength > , r#root_23_accepted : sp :: Callback < (sp :: SharedString ,) , () > , r#root_23_edited : sp :: Callback < (sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEditBase_root_23 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEditBase_root_23 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((((({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_computed_x }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_clip_24 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_placeholder_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from ("")))) && (((({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#preedit_text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))))) {
                         ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_placeholder_text }
                        ) . apply_pin (_self) . get () }
                     else {
                         (sp :: SharedString :: from ("")) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#accepted) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_accepted }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_text_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_computed_x }
                            ) . apply_pin (_self) . get () . get () as f64)) as f64) < (({
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_margin }
                            ) . apply_pin (_self) . get () . get () as f64)) {
                                 {
                                     ({
                                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_computed_x }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- (args . 0 . clone ()) . r#x as f64) + (({
                                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_margin }
                                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_computed_x }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (({
                                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_margin }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_computed_x }
                                        ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((((((({
                                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (({
                                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_margin }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Color :: from_argb_encoded (4286611584f64 as u32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_root_clip_24_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_root_clip_24_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_clip_24 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_clip_24 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_clip_24 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_clip_24 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_clip_24 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_min_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_root_clip_24_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_root_clip_24_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ((1f64 as f64) * (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (2u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_selection (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                ) . apply_pin (_self) . r#clear_selection ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_copy (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                ) . apply_pin (_self) . r#copy ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_cut (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                ) . apply_pin (_self) . r#cut ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_paste (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                ) . apply_pin (_self) . r#paste ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_select_all (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                ) . apply_pin (_self) . r#select_all ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_set_selection_offsets (self : :: core :: pin :: Pin < & Self > , arg_0 : i32 , arg_1 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             ({
                 ({
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                ) . apply_pin (_self) . set_selection_offsets ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , args . 0 . clone () as i32 , args . 1 . clone () as i32) }
            ) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerLineEdit_root_27 {
         r#root_27 : sp :: r#Empty , r#i_background_28 : sp :: r#BasicBorderRectangle , r#i_focus_border_31 : sp :: r#Rectangle , r#i_base_30 : InnerLineEditBase_root_23 , r#root_27_height : sp :: Property < sp :: LogicalLength > , r#root_27_i_background_28_width : sp :: Property < sp :: LogicalLength > , r#root_27_i_background_28_x : sp :: Property < sp :: LogicalLength > , r#root_27_i_background_28_y : sp :: Property < sp :: LogicalLength > , r#root_27_i_layout_29_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_27_i_layout_29_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_27_i_layout_29_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_27_state : sp :: Property < i32 > , r#root_27_vertical_stretch : sp :: Property < f32 > , r#root_27_width : sp :: Property < sp :: LogicalLength > , r#root_27_x : sp :: Property < sp :: LogicalLength > , r#root_27_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEdit_root_27 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEdit_root_27 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEditBase_root_23 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_base_30 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 4u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_background_28_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                                 + {
                                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                                         InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                                     + {
                                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_min_width }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_background_28_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                             + {
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                                 + {
                                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_min_width }
                                ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                             + {
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                                 + {
                                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_preferred_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                     + {
                         * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                         + {
                             * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_has_focus }
                        ) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_vertical_stretch }
            ) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_background_28 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_27_state = ({
                             * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_27_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (184549375f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1308228089f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_27_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (3005095454f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (268435455f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_background_28 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_27_state = ({
                             * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_27_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_27_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                                ) }
                             else {
                                 (InnerFluentPalette_55 :: FIELD_OFFSETS . r#text_control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_FluentPalette_55 . as_ref ()) . get ()) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_background_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_background_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1.0766f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_margin }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_placeholder_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_27_state = ({
                             * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_27_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_27_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (2332033023f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4278221012f64 as u32)) . color ()) as sp :: Color }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (2281701375f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                        ) . color () }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                        ) . color ()) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_focus_border_31 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_state }
                    ) . apply_pin (_self) . get () as f64) == (2f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4284534271f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278214584f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_background_28_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_background_28_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_background_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_background_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_placeholder_25_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_i_text_input_26_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_margin }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_root_clip_24_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_root_clip_24_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerLineEditBase_root_23 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_base_30 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_background_28_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_background_28_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (2f64 as sp :: Coord , ((({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_width }
                ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as sp :: Coord , 4f64 as sp :: Coord , ((({
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_height }
                ) . apply_pin (_self) . get () . get () as f64) - (2f64 as f64)) as sp :: Coord ,) , 4u32 ..= 6u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_base_30 }
                 . apply_pin (_self) . item_geometry (index - 4u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_base_30 }
                 . apply_pin (_self) . accessible_role (0) , 4u32 ..= 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_base_30 }
                 . apply_pin (_self) . accessible_role (index - 4u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_base_30 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (4u32 ..= 6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_base_30 }
                 . apply_pin (_self) . accessible_string_property (index - 4u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerScrollBar_root_32 {
         r#root_32 : sp :: r#BasicBorderRectangle , r#i_thumb_33 : sp :: r#BasicBorderRectangle , r#i_touch_area_34 : sp :: r#TouchArea , r#i_up_scroll_button_opacity_35 : sp :: r#Opacity , r#i_up_scroll_button_36 : sp :: r#TouchArea , r#i_icon_opacity_37 : sp :: r#Opacity , r#i_icon_38 : sp :: r#ImageItem , r#i_down_scroll_button_opacity_39 : sp :: r#Opacity , r#i_down_scroll_button_40 : sp :: r#TouchArea , r#i_icon_opacity_41 : sp :: r#Opacity , r#i_icon_42 : sp :: r#ImageItem , r#root_32_height : sp :: Property < sp :: LogicalLength > , r#root_32_horizontal : sp :: Property < bool > , r#root_32_i_down_scroll_button_40_icon : sp :: Property < sp :: Image > , r#root_32_i_down_scroll_button_40_state : sp :: Property < i32 > , r#root_32_i_down_scroll_button_40_x : sp :: Property < sp :: LogicalLength > , r#root_32_i_down_scroll_button_40_y : sp :: Property < sp :: LogicalLength > , r#root_32_i_down_scroll_button_opacity_39_dummy : sp :: Property < sp :: LogicalLength > , r#root_32_i_icon_opacity_37_dummy : sp :: Property < sp :: LogicalLength > , r#root_32_i_icon_opacity_41_dummy : sp :: Property < sp :: LogicalLength > , r#root_32_i_thumb_33_height : sp :: Property < sp :: LogicalLength > , r#root_32_i_thumb_33_width : sp :: Property < sp :: LogicalLength > , r#root_32_i_thumb_33_x : sp :: Property < sp :: LogicalLength > , r#root_32_i_thumb_33_y : sp :: Property < sp :: LogicalLength > , r#root_32_i_touch_area_34_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_32_i_touch_area_34_x : sp :: Property < sp :: LogicalLength > , r#root_32_i_touch_area_34_y : sp :: Property < sp :: LogicalLength > , r#root_32_i_up_scroll_button_36_icon : sp :: Property < sp :: Image > , r#root_32_i_up_scroll_button_36_state : sp :: Property < i32 > , r#root_32_i_up_scroll_button_36_x : sp :: Property < sp :: LogicalLength > , r#root_32_i_up_scroll_button_36_y : sp :: Property < sp :: LogicalLength > , r#root_32_i_up_scroll_button_opacity_35_dummy : sp :: Property < sp :: LogicalLength > , r#root_32_maximum : sp :: Property < sp :: LogicalLength > , r#root_32_page_size : sp :: Property < sp :: LogicalLength > , r#root_32_size : sp :: Property < sp :: LogicalLength > , r#root_32_state : sp :: Property < i32 > , r#root_32_track_size : sp :: Property < sp :: LogicalLength > , r#root_32_value : sp :: Property < sp :: LogicalLength > , r#root_32_width : sp :: Property < sp :: LogicalLength > , r#root_32_x : sp :: Property < sp :: LogicalLength > , r#root_32_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerScrollBar_root_32 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerScrollBar_root_32 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4281084972f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4293980400f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_icon }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) }
                     else {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_size }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         ({
                             let r#tmp_root_32_maximum = ({
                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_root_32_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_root_32_page_size = ({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_page_size }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((16f64 as sp :: Coord) . max ((((({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_root_32_page_size . clone () as f64) / (((r#tmp_root_32_maximum . clone () as f64) + (r#tmp_root_32_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_size }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         ({
                             let r#tmp_root_32_maximum = ({
                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_root_32_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_root_32_page_size = ({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_page_size }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((16f64 as sp :: Coord) . max ((((((({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (r#tmp_root_32_page_size . clone () as f64)) as f64) / (((r#tmp_root_32_maximum . clone () as f64) + (r#tmp_root_32_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_width }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (((16f64 as f64) + (((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                        ) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_height }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (((16f64 as f64) + (((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                        ) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_touch_area_34_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_touch_area_34_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_icon }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) }
                     else {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (4f64) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (4f64) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         6f64 }
                     else {
                         (2f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ()) || (({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ()))) || (({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_track_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (32f64 as f64)) }
                     else {
                         (((({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (32f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_thumb_33 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_thumb_33 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_height }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_width }
                        ) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- (0f64 as sp :: Coord) . max (((({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_touch_area_34_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if ({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                                    ) . apply_pin (_self) . get () {
                                         ((((({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     else {
                                         (((((({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_touch_area_34_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- ({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression1 = {
                                 let r#return_check_merge1 = if ((({
                                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                                ) . apply_pin (_self) . get ()) && ((((args . 0 . clone ()) . r#delta_x as f64) != (0f64 as f64)))) {
                                     (false , {
                                         ({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                                        ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                                        ) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,) }
                                 else {
                                     (if ! ((! ({
                                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                                    ) . apply_pin (_self) . get ()) && ((((args . 0 . clone ()) . r#delta_y as f64) != (0f64 as f64)))) {
                                         {
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                         }
                                     else {
                                         ((false , {
                                             ({
                                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                                            ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                                            ) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                    ) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge1 . clone ()) . 0 {
                                     ({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((sp :: r#EventResult :: r#Reject , false , (r#return_check_merge1 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression1 . clone ()) . 1 {
                                 (r#returned_expression1 . clone ()) . 0 }
                             else {
                                 ((r#returned_expression1 . clone ()) . 2) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_opacity_35 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                            ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min ((((({
                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                            ) . apply_pin (_self) . get () . get () as f64) + (10f64 as f64)) as sp :: Coord)) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_opacity_37 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_state }
                    ) . apply_pin (_self) . get () as f64) == (2f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_icon }
                        ) . apply_pin (_self) . get () . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) }
                     else {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         6f64 }
                     else {
                         (8f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_opacity_39 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                            ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((((({
                                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
                            ) . apply_pin (_self) . get () . get () as f64) - (10f64 as f64)) as sp :: Coord)) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_opacity_41 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_state }
                    ) . apply_pin (_self) . get () as f64) == (2f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = ({
                             * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_icon }
                        ) . apply_pin (_self) . get () . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
                    ) . apply_pin (_self) . get () {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) }
                     else {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         6f64 }
                     else {
                         (8f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_opacity_39_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_37_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_41_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_opacity_35_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_thumb_33 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_thumb_33 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_thumb_33_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_touch_area_34_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_touch_area_34_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (8f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_36_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (8f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_40_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (8f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_opacity_35_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_opacity_35_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((8f64 as f64) - (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((8f64 as f64) - (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 7u32 => (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_37_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_37_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (8f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_opacity_39_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_opacity_39_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 9u32 => (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((8f64 as f64) - (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((8f64 as f64) - (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 10u32 => (({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_41_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_41_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerTextEdit_root_43 {
         r#root_43 : sp :: r#Empty , r#i_background_44 : sp :: r#BasicBorderRectangle , r#i_scroll_view_45 : sp :: r#Empty , r#i_flickable_46 : sp :: r#Flickable , r#i_flickable_viewport_47 : sp :: r#Empty , r#i_text_input_48 : sp :: r#TextInput , r#i_vertical_bar_visibility_49 : sp :: r#Clip , r#i_horizontal_bar_visibility_51 : sp :: r#Clip , r#i_focus_border_53 : sp :: r#Rectangle , r#i_vertical_bar_50 : InnerScrollBar_root_32 , r#i_horizontal_bar_52 : InnerScrollBar_root_32 , r#root_43_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_background_44_x : sp :: Property < sp :: LogicalLength > , r#root_43_i_background_44_y : sp :: Property < sp :: LogicalLength > , r#root_43_i_flickable_46_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_flickable_46_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_flickable_46_x : sp :: Property < sp :: LogicalLength > , r#root_43_i_flickable_46_y : sp :: Property < sp :: LogicalLength > , r#root_43_i_horizontal_bar_52_visible : sp :: Property < bool > , r#root_43_i_horizontal_bar_visibility_51_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_horizontal_bar_visibility_51_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_horizontal_bar_visibility_51_x : sp :: Property < sp :: LogicalLength > , r#root_43_i_horizontal_bar_visibility_51_y : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_48_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_48_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_48_x : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_48_y : sp :: Property < sp :: LogicalLength > , r#root_43_i_vertical_bar_50_visible : sp :: Property < bool > , r#root_43_i_vertical_bar_visibility_49_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_vertical_bar_visibility_49_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_vertical_bar_visibility_49_x : sp :: Property < sp :: LogicalLength > , r#root_43_i_vertical_bar_visibility_49_y : sp :: Property < sp :: LogicalLength > , r#root_43_state : sp :: Property < i32 > , r#root_43_width : sp :: Property < sp :: LogicalLength > , r#root_43_x : sp :: Property < sp :: LogicalLength > , r#root_43_y : sp :: Property < sp :: LogicalLength > , r#root_43_edited : sp :: Callback < (sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerTextEdit_root_43 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerTextEdit_root_43 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerScrollBar_root_32 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 9u32 - 1 , tree_index_of_first_child + 10u32 - 1) ;
             InnerScrollBar_root_32 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 20u32 - 1 , tree_index_of_first_child + 21u32 - 1) ;
             sp :: Property :: link_two_way (({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
            ) . apply_pin (_self) , ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self)) ;
             sp :: Property :: link_two_way (({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_value }
            ) . apply_pin (_self) , ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64 as f64) * (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (24f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64 as f64) * (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (24f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((1f64 as f64) * (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (24f64 as f64)) as f64) - (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((1f64 as f64) * (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (24f64 as f64)) as f64) - (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_52_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_50_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                         + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_background_44 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_43_state = ({
                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_43_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (184549375f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1308228089f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_43_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (3005095454f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (268435455f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_background_44 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_43_state = ({
                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_43_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_43_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                                ) }
                             else {
                                 (InnerFluentPalette_55 :: FIELD_OFFSETS . r#text_control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_FluentPalette_55 . as_ref ()) . get ()) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_background_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_background_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . get ()) == (sp :: r#TextWrap :: r#WordWrap)) {
                         ({
                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         ((({
                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_preferred_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64)) as f64) < (12f64 as f64)) {
                                 {
                                     ({
                                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                                    ) . apply_pin (_self) . get () . get () as f64) - (({
                                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((- (args . 0 . clone ()) . r#x as f64) + (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64)) as f64) > (((({
                                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (12f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((((({
                                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             ;
                             if (((((args . 0 . clone ()) . r#y as f64) + (({
                                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64)) as f64) < (12f64 as f64)) {
                                 {
                                     ({
                                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                                    ) . apply_pin (_self) . get () . get () as f64) - (({
                                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((- (args . 0 . clone ()) . r#y as f64) + (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#y as f64) + (({
                                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                                ) . apply_pin (_self) . get () . get () as f64) - (12f64 as f64)) as f64) - (20f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((((((({
                                             * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#y as f64)) as f64) - (12f64 as f64)) as f64) - (20f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1.0766f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4278221012f64 as u32)) . color ()) as sp :: Color }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (2281701375f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                        ) . color () }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                        ) . color ()) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#TextWrap :: r#WordWrap) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_49 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_50_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_page_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_x }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (14f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_y }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_51 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_52_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_page_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_x }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                    ) . apply_pin (_self) . get () . get () as f64) + (({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_y }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (14f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_focus_border_53 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_state }
                    ) . apply_pin (_self) . get () as f64) == (2f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4284534271f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278214584f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_background_44_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_background_44_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_background_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_background_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#input_type) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_49 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_49 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_49 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_49 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_49 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_opacity_39_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_37_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_41_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_opacity_35_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_horizontal }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_down_scroll_button_opacity_39_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_37_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_icon_opacity_41_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_i_up_scroll_button_opacity_35_dummy }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerScrollBar_root_32 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             . apply_pin (x)) ,) ;
             InnerScrollBar_root_32 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((1f64 as f64) * (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ((1f64 as f64) * (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_background_44_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_background_44_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (((((1f64 as f64) * (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (24f64 as f64)) as sp :: Coord , ((((1f64 as f64) * (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (24f64 as f64)) as sp :: Coord , 12f64 as sp :: Coord , 12f64 as sp :: Coord ,) , 3u32 => (2f64 as sp :: Coord , ((((1f64 as f64) * (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (8f64 as f64)) as sp :: Coord , 4f64 as sp :: Coord , ((((1f64 as f64) * (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (2f64 as f64)) as sp :: Coord ,) , 4u32 => (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 7u32 => (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 9u32 => (({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 14f64 as sp :: Coord , ({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 20u32 => (14f64 as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_flickable_46_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 + {
                     * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 10u32 ..= 19u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 . apply_pin (_self) . item_geometry (index - 10u32 + 1) , 21u32 ..= 30u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 . apply_pin (_self) . item_geometry (index - 21u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 . apply_pin (_self) . accessible_role (0) , 10u32 ..= 19u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 . apply_pin (_self) . accessible_role (index - 10u32 + 1) , 20u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 . apply_pin (_self) . accessible_role (0) , 21u32 ..= 30u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 . apply_pin (_self) . accessible_role (index - 21u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (10u32 ..= 19u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_50 }
                 . apply_pin (_self) . accessible_string_property (index - 10u32 + 1 , what) , (20u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (21u32 ..= 30u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
                 . apply_pin (_self) . accessible_string_property (index - 21u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_1 : sp :: r#WindowItem , r#empty_3 : sp :: r#Empty , r#button_4 : InnerButton_root_11 , r#button_5 : InnerButton_root_11 , r#button_6 : InnerButton_root_11 , r#title_7 : InnerLineEdit_root_27 , r#currentNote_8 : InnerTextEdit_root_43 , r#root_1_empty_2_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_2_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_file_contents : sp :: Property < sp :: SharedString > , r#root_1_file_title : sp :: Property < sp :: SharedString > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_new_note : sp :: Callback < () , () > , r#root_1_open_note : sp :: Callback < () , () > , r#root_1_save_note : sp :: Callback < (sp :: SharedString , sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_AppWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerAppWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerButton_root_11 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_4 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 4u32 - 1 , tree_index_of_first_child + 7u32 - 1) ;
             InnerButton_root_11 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_5 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 5u32 - 1 , tree_index_of_first_child + 14u32 - 1) ;
             InnerButton_root_11 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_6 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 6u32 - 1 , tree_index_of_first_child + 21u32 - 1) ;
             InnerLineEdit_root_27 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#title_7 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 28u32 - 1) ;
             InnerTextEdit_root_43 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#currentNote_8 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 3u32 - 1 , tree_index_of_first_child + 34u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_54 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_54 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4280032284f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294638330f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                                 + {
                                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_h }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                                     + {
                                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 16f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 10f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 1f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                                 + {
                                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_v }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                                     + {
                                         * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 2f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 16f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 10f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                             + {
                                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_h }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                                 + {
                                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 1f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                             + {
                                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_v }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                                 + {
                                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_layout_29_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 2f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                                     + {
                                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                                     + {
                                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                                     + {
                                         * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                             + {
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                             + {
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                             + {
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                             + {
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                             + {
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                             + {
                                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                                 + {
                                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_layout_14_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"png"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("NoteTaker")) as sp :: SharedString }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_open_note }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Open")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_save_note }
                            ) . apply_pin (_self) . call (& (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                             + {
                                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                             + {
                                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ , ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
                             + {
                                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Save")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_new_note }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("New")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                 + {
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23_placeholder_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Title goes here")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                 + {
                     InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
                 + {
                     * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_file_title }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_vertical_stretch }
            ) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                 + {
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                 + {
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
                 + {
                     * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
                 + {
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
                 + {
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_file_contents }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
                 + {
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
                 + {
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
                 + {
                     * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_focus_scope_20_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_focus_scope_20_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_background_12_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_border_13_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_focus_scope_20_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_i_touch_area_19_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_background_28_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_i_background_28_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27_vertical_stretch }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_background_44_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_background_44_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_horizontal_bar_visibility_51_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_48_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43_i_vertical_bar_visibility_49_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerButton_root_11 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_4 }
             . apply_pin (x)) ,) ;
             InnerButton_root_11 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_5 }
             . apply_pin (x)) ,) ;
             InnerButton_root_11 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_6 }
             . apply_pin (x)) ,) ;
             InnerLineEdit_root_27 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#title_7 }
             . apply_pin (x)) ,) ;
             InnerTextEdit_root_43 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#currentNote_8 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_4 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 3u32 ..= 5u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_5 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 3u32 , order , visitor) }
                 6u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_6 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 6u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_4 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 3u32 ..= 5u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_5 }
                     . apply_pin (_self) . subtree_range (dyn_index - 3u32) }
                 6u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_6 }
                     . apply_pin (_self) . subtree_range (dyn_index - 6u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_4 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 3u32 ..= 5u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_5 }
                     . apply_pin (_self) . subtree_component (dyn_index - 3u32 , subtree_index , result) }
                 6u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_6 }
                     . apply_pin (_self) . subtree_component (dyn_index - 6u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 4u32 => ({
                     let r#tmp_empty_3_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 8f64 as sp :: Coord ,) , 5u32 => ({
                     let r#tmp_empty_3_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 8f64 as sp :: Coord ,) , 6u32 => ({
                     let r#tmp_empty_3_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 8f64 as sp :: Coord ,) , 7u32 ..= 13u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_4 }
                 . apply_pin (_self) . item_geometry (index - 7u32 + 1) , 14u32 ..= 20u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_5 }
                 . apply_pin (_self) . item_geometry (index - 14u32 + 1) , 21u32 ..= 27u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . item_geometry (index - 21u32 + 1) , 28u32 ..= 33u32 => return {
                     * & Self :: FIELD_OFFSETS . r#title_7 }
                 . apply_pin (_self) . item_geometry (index - 28u32 + 1) , 34u32 ..= 63u32 => return {
                     * & Self :: FIELD_OFFSETS . r#currentNote_8 }
                 . apply_pin (_self) . item_geometry (index - 34u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 => sp :: r#AccessibleRole :: r#Button , 5u32 => sp :: r#AccessibleRole :: r#Button , 6u32 => sp :: r#AccessibleRole :: r#Button , 4u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_4 }
                 . apply_pin (_self) . accessible_role (0) , 7u32 ..= 13u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_4 }
                 . apply_pin (_self) . accessible_role (index - 7u32 + 1) , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_5 }
                 . apply_pin (_self) . accessible_role (0) , 14u32 ..= 20u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_5 }
                 . apply_pin (_self) . accessible_role (index - 14u32 + 1) , 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . accessible_role (0) , 21u32 ..= 27u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . accessible_role (index - 21u32 + 1) , 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#title_7 }
                 . apply_pin (_self) . accessible_role (0) , 28u32 ..= 33u32 => {
                     * & Self :: FIELD_OFFSETS . r#title_7 }
                 . apply_pin (_self) . accessible_role (index - 28u32 + 1) , 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#currentNote_8 }
                 . apply_pin (_self) . accessible_role (0) , 34u32 ..= 63u32 => {
                     * & Self :: FIELD_OFFSETS . r#currentNote_8 }
                 . apply_pin (_self) . accessible_role (index - 34u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (4u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
                ) . apply_pin (_self) . get () , (5u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
                ) . apply_pin (_self) . get () , (6u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11_text }
                ) . apply_pin (_self) . get () , (4u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_4 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (7u32 ..= 13u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_4 }
                 . apply_pin (_self) . accessible_string_property (index - 7u32 + 1 , what) , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_5 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (14u32 ..= 20u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_5 }
                 . apply_pin (_self) . accessible_string_property (index - 14u32 + 1 , what) , (6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (21u32 ..= 27u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . accessible_string_property (index - 21u32 + 1 , what) , (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#title_7 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (28u32 ..= 33u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#title_7 }
                 . apply_pin (_self) . accessible_string_property (index - 28u32 + 1 , what) , (3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#currentNote_8 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (34u32 ..= 63u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#currentNote_8 }
                 . apply_pin (_self) . accessible_string_property (index - 34u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerAppWindow {
         pub fn new () -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & self_rc) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             64usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 28u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 34u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 14u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 21u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 11u32 , parent_index : 4u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 4u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 4u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 7u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 7u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 18u32 , parent_index : 5u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 5u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 5u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 5u32 , parent_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 14u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 14u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 4u32 , parent_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 25u32 , parent_index : 6u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 6u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 6u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 8u32 , parent_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 21u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 6u32 , parent_index : 21u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 7u32 , parent_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 29u32 , parent_index : 2u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 31u32 , parent_index : 28u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 34u32 , parent_index : 28u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 32u32 , parent_index : 29u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 34u32 , parent_index : 31u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 34u32 , parent_index : 31u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 35u32 , parent_index : 3u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 37u32 , parent_index : 34u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 64u32 , parent_index : 34u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 40u32 , parent_index : 35u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 42u32 , parent_index : 35u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 53u32 , parent_index : 35u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 41u32 , parent_index : 37u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 42u32 , parent_index : 40u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 43u32 , parent_index : 38u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 47u32 , parent_index : 42u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 47u32 , parent_index : 42u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 47u32 , parent_index : 42u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 50u32 , parent_index : 42u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 48u32 , parent_index : 45u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 49u32 , parent_index : 47u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 50u32 , parent_index : 48u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 51u32 , parent_index : 46u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 52u32 , parent_index : 50u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 53u32 , parent_index : 51u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 54u32 , parent_index : 39u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 58u32 , parent_index : 53u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 58u32 , parent_index : 53u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 58u32 , parent_index : 53u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 61u32 , parent_index : 53u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 59u32 , parent_index : 56u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 60u32 , parent_index : 58u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 61u32 , parent_index : 59u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 62u32 , parent_index : 57u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 63u32 , parent_index : 61u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 64u32 , parent_index : 62u32 , item_array_index : 54u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             55usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#root_27 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#root_43 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#root_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_background_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_4 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_background_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_5 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_background_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_touch_area_19 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_focus_scope_20 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_11 :: FIELD_OFFSETS . r#i_border_13 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_background_28 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_23 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 * & InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_focus_border_31 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#root_clip_24 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_placeholder_25 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#title_7 }
             + {
                 InnerLineEdit_root_27 :: FIELD_OFFSETS . r#i_base_30 }
             + {
                 * & InnerLineEditBase_root_23 :: FIELD_OFFSETS . r#i_text_input_26 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_background_44 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_scroll_view_45 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_focus_border_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_46 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_49 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_51 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_flickable_viewport_47 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 * & InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_text_input_48 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_thumb_33 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_opacity_35 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_opacity_39 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_opacity_37 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_opacity_41 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_vertical_bar_50 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#root_32 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_thumb_33 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_touch_area_34 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_opacity_35 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_opacity_39 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_up_scroll_button_36 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_opacity_37 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_38 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_down_scroll_button_40 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_opacity_41 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#currentNote_8 }
             + {
                 InnerTextEdit_root_43 :: FIELD_OFFSETS . r#i_horizontal_bar_52 }
             + {
                 * & InnerScrollBar_root_32 :: FIELD_OFFSETS . r#i_icon_42 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self ,) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter_ . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let self_rc = sp :: VRcMapped :: origin (& self . self_weak . get () . unwrap () . upgrade () . unwrap () ,) ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& self_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter_ . get () . cloned () }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerAppWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerAppWindow {
         fn drop (self : core :: pin :: Pin < & mut InnerAppWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAppWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAppWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AppWindow (sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) ;
     impl r#AppWindow {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . global_ColorSchemeSelector_54 . clone () . init (& inner) ;
             inner . globals . global_FluentPalette_55 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_56 . clone () . init (& inner) ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_file_contents (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_file_contents }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_file_contents (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_file_contents }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_file_title (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_file_title }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_file_title (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_file_title }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_new_note (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_new_note }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_new_note (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_new_note }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_open_note (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_open_note }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_open_note (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_open_note }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_save_note (& self , arg_0 : sp :: SharedString , arg_1 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_save_note }
            ) . apply_pin (_self) . call (& (arg_0 , arg_1 ,)) }
         # [allow (dead_code)] pub fn on_save_note (& self , mut f : impl FnMut (sp :: SharedString , sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_save_note }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone ())) }
         }
     impl From < r#AppWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > {
         fn from (value : r#AppWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#AppWindow {
         type Inner = InnerAppWindow ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_AppWindow {
         global_ColorSchemeSelector_54 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_54 >> , global_FluentPalette_55 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_55 >> , global_StyleMetrics_56 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_56 >> , }
     impl :: core :: default :: Default for Globals_AppWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_54 : InnerColorSchemeSelector_54 :: new () , global_FluentPalette_55 : InnerFluentPalette_55 :: new () , global_StyleMetrics_56 : InnerStyleMetrics_56 :: new () , }
             }
         }
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 1C0 0.864583 0.0260417 0.735677 0.078125 0.613281C0.130208 0.490885 0.200521 0.385417 0.289062 0.296875C0.380208 0.205729 0.485677 0.134115 0.605469 0.0820312C0.72526 0.0273438 0.854167 0 0.992188 0H7.01172C7.14714 0 7.27474 0.0260417 7.39453 0.078125C7.51693 0.130208 7.6224 0.201823 7.71094 0.292969C7.79948 0.384115 7.86979 0.490885 7.92188 0.613281C7.97396 0.733073 8 0.860677 8 0.996094C8 1.10547 7.98438 1.20573 7.95312 1.29688C7.92448 1.38802 7.88021 1.47917 7.82031 1.57031L5.21875 5.35547C5.08073 5.55599 4.90365 5.71354 4.6875 5.82812C4.47396 5.94271 4.24479 6 4 6C3.75521 6 3.52474 5.94271 3.30859 5.82812C3.09505 5.71354 2.91927 5.55599 2.78125 5.35547L0.179688 1.57031C0.119792 1.48177 0.0742188 1.39193 0.0429688 1.30078C0.0143229 1.20964 0 1.10938 0 1Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_4 : & 'static [u8] = :: core :: include_bytes ! ("C:\\Users\\900ra\\Documents\\programming\\rust\\notetaker\\ui\\icon.png") ;
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 7.00781L0 0.992187C0 0.854167 0.0260417 0.72526 0.078125 0.605469C0.132812 0.485677 0.204427 0.38151 0.292969 0.292969C0.384115 0.201823 0.489583 0.130208 0.609375 0.078125C0.731771 0.0260417 0.861979 0 1 0C1.20573 0 1.39583 0.0598958 1.57031 0.179687L5.35547 2.78125C5.55859 2.92187 5.71615 3.09896 5.82813 3.3125C5.94271 3.52604 6 3.75521 6 4C6 4.24479 5.94271 4.47396 5.82813 4.6875C5.71615 4.90104 5.55859 5.07812 5.35547 5.21875L1.57031 7.82031C1.39583 7.9401 1.20573 8 1 8C0.861979 8 0.731771 7.97396 0.609375 7.92188C0.489583 7.86979 0.384115 7.79948 0.292969 7.71094C0.204427 7.61979 0.132813 7.51432 0.078125 7.39453C0.0260417 7.27474 0 7.14583 0 7.00781Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 4C0 3.75521 0.0572917 3.52604 0.171875 3.3125C0.286458 3.09635 0.44401 2.91927 0.644531 2.78125L4.42969 0.179687C4.51823 0.119792 4.60807 0.0755208 4.69922 0.046875C4.79036 0.015625 4.89062 0 5 0C5.13542 0 5.26432 0.0260417 5.38672 0.078125C5.50911 0.130208 5.61458 0.201823 5.70312 0.292969C5.79427 0.38151 5.86589 0.485677 5.91797 0.605469C5.97266 0.72526 6 0.854167 6 0.992187L6 7.00781C6 7.14583 5.97266 7.27474 5.91797 7.39453C5.86589 7.51432 5.79427 7.61979 5.70313 7.71094C5.61458 7.79948 5.50911 7.86979 5.38672 7.92187C5.26432 7.97396 5.13542 8 5 8C4.79427 8 4.60417 7.9401 4.42969 7.82031L0.644531 5.21875C0.44401 5.08073 0.286458 4.90495 0.171875 4.69141C0.0572917 4.47526 0 4.24479 0 4Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0.992188 6C0.854167 6 0.72526 5.97396 0.605469 5.92188C0.485677 5.86719 0.380208 5.79557 0.289062 5.70703C0.200521 5.61589 0.130208 5.51042 0.078125 5.39062C0.0260417 5.26823 0 5.13802 0 5C0 4.89062 0.0143229 4.79036 0.0429688 4.69922C0.0742188 4.60807 0.119792 4.51823 0.179688 4.42969L2.78125 0.644531C2.84896 0.545573 2.92839 0.458333 3.01953 0.382812C3.11068 0.307292 3.20833 0.244792 3.3125 0.195312C3.41927 0.143229 3.53125 0.104167 3.64844 0.078125C3.76562 0.0520833 3.88281 0.0390625 4 0.0390625C4.11719 0.0390625 4.23438 0.0520833 4.35156 0.078125C4.46875 0.104167 4.57943 0.143229 4.68359 0.195312C4.79036 0.244792 4.88932 0.307292 4.98047 0.382812C5.07161 0.458333 5.15104 0.545573 5.21875 0.644531L7.82031 4.42969C7.88021 4.51823 7.92448 4.60807 7.95312 4.69922C7.98438 4.79036 8 4.89062 8 5C8 5.13802 7.97396 5.26823 7.92188 5.39062C7.86979 5.51042 7.79948 5.61589 7.71094 5.70703C7.6224 5.79557 7.51693 5.86719 7.39453 5.92188C7.27474 5.97396 7.14714 6 7.01172 6H0.992188Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_5_1 = slint :: VersionCheck_1_5_1 ;
     }
 # [allow (unused_imports)] pub use slint_generatedAppWindow :: {
     r#AppWindow , r#TextStyle }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;

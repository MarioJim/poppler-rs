/* automatically generated by rust-bindgen */

pub const PopplerAnnotType_POPPLER_ANNOT_UNKNOWN: PopplerAnnotType = 0;
pub const PopplerAnnotType_POPPLER_ANNOT_TEXT: PopplerAnnotType = 1;
pub const PopplerAnnotType_POPPLER_ANNOT_LINK: PopplerAnnotType = 2;
pub const PopplerAnnotType_POPPLER_ANNOT_FREE_TEXT: PopplerAnnotType = 3;
pub const PopplerAnnotType_POPPLER_ANNOT_LINE: PopplerAnnotType = 4;
pub const PopplerAnnotType_POPPLER_ANNOT_SQUARE: PopplerAnnotType = 5;
pub const PopplerAnnotType_POPPLER_ANNOT_CIRCLE: PopplerAnnotType = 6;
pub const PopplerAnnotType_POPPLER_ANNOT_POLYGON: PopplerAnnotType = 7;
pub const PopplerAnnotType_POPPLER_ANNOT_POLY_LINE: PopplerAnnotType = 8;
pub const PopplerAnnotType_POPPLER_ANNOT_HIGHLIGHT: PopplerAnnotType = 9;
pub const PopplerAnnotType_POPPLER_ANNOT_UNDERLINE: PopplerAnnotType = 10;
pub const PopplerAnnotType_POPPLER_ANNOT_SQUIGGLY: PopplerAnnotType = 11;
pub const PopplerAnnotType_POPPLER_ANNOT_STRIKE_OUT: PopplerAnnotType = 12;
pub const PopplerAnnotType_POPPLER_ANNOT_STAMP: PopplerAnnotType = 13;
pub const PopplerAnnotType_POPPLER_ANNOT_CARET: PopplerAnnotType = 14;
pub const PopplerAnnotType_POPPLER_ANNOT_INK: PopplerAnnotType = 15;
pub const PopplerAnnotType_POPPLER_ANNOT_POPUP: PopplerAnnotType = 16;
pub const PopplerAnnotType_POPPLER_ANNOT_FILE_ATTACHMENT: PopplerAnnotType = 17;
pub const PopplerAnnotType_POPPLER_ANNOT_SOUND: PopplerAnnotType = 18;
pub const PopplerAnnotType_POPPLER_ANNOT_MOVIE: PopplerAnnotType = 19;
pub const PopplerAnnotType_POPPLER_ANNOT_WIDGET: PopplerAnnotType = 20;
pub const PopplerAnnotType_POPPLER_ANNOT_SCREEN: PopplerAnnotType = 21;
pub const PopplerAnnotType_POPPLER_ANNOT_PRINTER_MARK: PopplerAnnotType = 22;
pub const PopplerAnnotType_POPPLER_ANNOT_TRAP_NET: PopplerAnnotType = 23;
pub const PopplerAnnotType_POPPLER_ANNOT_WATERMARK: PopplerAnnotType = 24;
pub const PopplerAnnotType_POPPLER_ANNOT_3D: PopplerAnnotType = 25;
pub type PopplerAnnotType = u32;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_UNKNOWN: PopplerAnnotFlag = 0;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_INVISIBLE: PopplerAnnotFlag = 1;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_HIDDEN: PopplerAnnotFlag = 2;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_PRINT: PopplerAnnotFlag = 4;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_NO_ZOOM: PopplerAnnotFlag = 8;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_NO_ROTATE: PopplerAnnotFlag = 16;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_NO_VIEW: PopplerAnnotFlag = 32;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_READ_ONLY: PopplerAnnotFlag = 64;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_LOCKED: PopplerAnnotFlag = 128;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_TOGGLE_NO_VIEW: PopplerAnnotFlag = 256;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_LOCKED_CONTENTS: PopplerAnnotFlag = 512;
pub type PopplerAnnotFlag = u32;
pub const PopplerAnnotMarkupReplyType_POPPLER_ANNOT_MARKUP_REPLY_TYPE_R:
    PopplerAnnotMarkupReplyType = 0;
pub const PopplerAnnotMarkupReplyType_POPPLER_ANNOT_MARKUP_REPLY_TYPE_GROUP:
    PopplerAnnotMarkupReplyType = 1;
pub type PopplerAnnotMarkupReplyType = u32;
pub const PopplerAnnotExternalDataType_POPPLER_ANNOT_EXTERNAL_DATA_MARKUP_3D:
    PopplerAnnotExternalDataType = 0;
pub const PopplerAnnotExternalDataType_POPPLER_ANNOT_EXTERNAL_DATA_MARKUP_UNKNOWN:
    PopplerAnnotExternalDataType = 1;
pub type PopplerAnnotExternalDataType = u32;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_MARKED: PopplerAnnotTextState = 0;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_UNMARKED: PopplerAnnotTextState = 1;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_ACCEPTED: PopplerAnnotTextState = 2;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_REJECTED: PopplerAnnotTextState = 3;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_CANCELLED: PopplerAnnotTextState = 4;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_COMPLETED: PopplerAnnotTextState = 5;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_NONE: PopplerAnnotTextState = 6;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_UNKNOWN: PopplerAnnotTextState = 7;
pub type PopplerAnnotTextState = u32;
pub const PopplerAnnotFreeTextQuadding_POPPLER_ANNOT_FREE_TEXT_QUADDING_LEFT_JUSTIFIED:
    PopplerAnnotFreeTextQuadding = 0;
pub const PopplerAnnotFreeTextQuadding_POPPLER_ANNOT_FREE_TEXT_QUADDING_CENTERED:
    PopplerAnnotFreeTextQuadding = 1;
pub const PopplerAnnotFreeTextQuadding_POPPLER_ANNOT_FREE_TEXT_QUADDING_RIGHT_JUSTIFIED:
    PopplerAnnotFreeTextQuadding = 2;
pub type PopplerAnnotFreeTextQuadding = u32;
#[repr(C)]
pub struct _PopplerAnnotCalloutLine {
    pub multiline: gboolean,
    pub x1: gdouble,
    pub y1: gdouble,
    pub x2: gdouble,
    pub y2: gdouble,
    pub x3: gdouble,
    pub y3: gdouble,
}
#[test]
fn bindgen_test_layout__PopplerAnnotCalloutLine() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerAnnotCalloutLine>(),
        56usize,
        concat!("Size of: ", stringify!(_PopplerAnnotCalloutLine))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerAnnotCalloutLine>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerAnnotCalloutLine))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).multiline as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(multiline)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).x1 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(x1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).y1 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(y1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).x2 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(x2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).y2 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(y2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).x3 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(x3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).y3 as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(y3)
        )
    );
}
extern "C" {
    pub fn poppler_annot_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_get_annot_type(poppler_annot: *mut PopplerAnnot) -> PopplerAnnotType;
}
extern "C" {
    pub fn poppler_annot_get_contents(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_set_contents(poppler_annot: *mut PopplerAnnot, contents: *const gchar);
}
extern "C" {
    pub fn poppler_annot_get_name(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_get_modified(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_get_flags(poppler_annot: *mut PopplerAnnot) -> PopplerAnnotFlag;
}
extern "C" {
    pub fn poppler_annot_set_flags(poppler_annot: *mut PopplerAnnot, flags: PopplerAnnotFlag);
}
extern "C" {
    pub fn poppler_annot_get_color(poppler_annot: *mut PopplerAnnot) -> *mut PopplerColor;
}
extern "C" {
    pub fn poppler_annot_set_color(
        poppler_annot: *mut PopplerAnnot,
        poppler_color: *mut PopplerColor,
    );
}
extern "C" {
    pub fn poppler_annot_get_page_index(poppler_annot: *mut PopplerAnnot) -> gint;
}
extern "C" {
    pub fn poppler_annot_get_rectangle(
        poppler_annot: *mut PopplerAnnot,
        poppler_rect: *mut PopplerRectangle,
    );
}
extern "C" {
    pub fn poppler_annot_set_rectangle(
        poppler_annot: *mut PopplerAnnot,
        poppler_rect: *mut PopplerRectangle,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_markup_get_label(poppler_annot: *mut PopplerAnnotMarkup) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_markup_set_label(
        poppler_annot: *mut PopplerAnnotMarkup,
        label: *const gchar,
    );
}
extern "C" {
    pub fn poppler_annot_markup_has_popup(poppler_annot: *mut PopplerAnnotMarkup) -> gboolean;
}
extern "C" {
    pub fn poppler_annot_markup_set_popup(
        poppler_annot: *mut PopplerAnnotMarkup,
        popup_rect: *mut PopplerRectangle,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_popup_is_open(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_annot_markup_set_popup_is_open(
        poppler_annot: *mut PopplerAnnotMarkup,
        is_open: gboolean,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_popup_rectangle(
        poppler_annot: *mut PopplerAnnotMarkup,
        poppler_rect: *mut PopplerRectangle,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_annot_markup_set_popup_rectangle(
        poppler_annot: *mut PopplerAnnotMarkup,
        poppler_rect: *mut PopplerRectangle,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_opacity(poppler_annot: *mut PopplerAnnotMarkup) -> gdouble;
}
extern "C" {
    pub fn poppler_annot_markup_set_opacity(
        poppler_annot: *mut PopplerAnnotMarkup,
        opacity: gdouble,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_date(poppler_annot: *mut PopplerAnnotMarkup) -> *mut GDate;
}
extern "C" {
    pub fn poppler_annot_markup_get_subject(poppler_annot: *mut PopplerAnnotMarkup) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_markup_get_reply_to(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> PopplerAnnotMarkupReplyType;
}
extern "C" {
    pub fn poppler_annot_markup_get_external_data(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> PopplerAnnotExternalDataType;
}
extern "C" {
    pub fn poppler_annot_text_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_text_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_get_is_open(poppler_annot: *mut PopplerAnnotText) -> gboolean;
}
extern "C" {
    pub fn poppler_annot_text_set_is_open(poppler_annot: *mut PopplerAnnotText, is_open: gboolean);
}
extern "C" {
    pub fn poppler_annot_text_get_icon(poppler_annot: *mut PopplerAnnotText) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_text_set_icon(poppler_annot: *mut PopplerAnnotText, icon: *const gchar);
}
extern "C" {
    pub fn poppler_annot_text_get_state(
        poppler_annot: *mut PopplerAnnotText,
    ) -> PopplerAnnotTextState;
}
extern "C" {
    pub fn poppler_annot_text_markup_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_text_markup_new_highlight(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_markup_new_squiggly(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_markup_new_strikeout(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_markup_new_underline(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_markup_set_quadrilaterals(
        poppler_annot: *mut PopplerAnnotTextMarkup,
        quadrilaterals: *mut GArray,
    );
}
extern "C" {
    pub fn poppler_annot_text_markup_get_quadrilaterals(
        poppler_annot: *mut PopplerAnnotTextMarkup,
    ) -> *mut GArray;
}
extern "C" {
    pub fn poppler_annot_free_text_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_free_text_get_quadding(
        poppler_annot: *mut PopplerAnnotFreeText,
    ) -> PopplerAnnotFreeTextQuadding;
}
extern "C" {
    pub fn poppler_annot_free_text_get_callout_line(
        poppler_annot: *mut PopplerAnnotFreeText,
    ) -> *mut PopplerAnnotCalloutLine;
}
extern "C" {
    pub fn poppler_annot_file_attachment_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_file_attachment_get_attachment(
        poppler_annot: *mut PopplerAnnotFileAttachment,
    ) -> *mut PopplerAttachment;
}
extern "C" {
    pub fn poppler_annot_file_attachment_get_name(
        poppler_annot: *mut PopplerAnnotFileAttachment,
    ) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_movie_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_movie_get_title(poppler_annot: *mut PopplerAnnotMovie) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_movie_get_movie(
        poppler_annot: *mut PopplerAnnotMovie,
    ) -> *mut PopplerMovie;
}
extern "C" {
    pub fn poppler_annot_screen_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_screen_get_action(
        poppler_annot: *mut PopplerAnnotScreen,
    ) -> *mut PopplerAction;
}
extern "C" {
    pub fn poppler_annot_line_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_line_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        start: *mut PopplerPoint,
        end: *mut PopplerPoint,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_line_set_vertices(
        poppler_annot: *mut PopplerAnnotLine,
        start: *mut PopplerPoint,
        end: *mut PopplerPoint,
    );
}
extern "C" {
    pub fn poppler_annot_callout_line_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_callout_line_new() -> *mut PopplerAnnotCalloutLine;
}
extern "C" {
    pub fn poppler_annot_callout_line_copy(
        callout: *mut PopplerAnnotCalloutLine,
    ) -> *mut PopplerAnnotCalloutLine;
}
extern "C" {
    pub fn poppler_annot_callout_line_free(callout: *mut PopplerAnnotCalloutLine);
}
extern "C" {
    pub fn poppler_annot_circle_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_circle_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_circle_set_interior_color(
        poppler_annot: *mut PopplerAnnotCircle,
        poppler_color: *mut PopplerColor,
    );
}
extern "C" {
    pub fn poppler_annot_circle_get_interior_color(
        poppler_annot: *mut PopplerAnnotCircle,
    ) -> *mut PopplerColor;
}
extern "C" {
    pub fn poppler_annot_square_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_square_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_square_set_interior_color(
        poppler_annot: *mut PopplerAnnotSquare,
        poppler_color: *mut PopplerColor,
    );
}
extern "C" {
    pub fn poppler_annot_square_get_interior_color(
        poppler_annot: *mut PopplerAnnotSquare,
    ) -> *mut PopplerColor;
}

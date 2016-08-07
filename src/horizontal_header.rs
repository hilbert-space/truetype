//! The [horizontal header][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/hhea.htm

use q32;

table! {
    #[doc = "A horizontal header."]
    #[derive(Copy)]
    pub HorizontalHeader {
        version (q32) |tape, this| { // version
            let value = try!(tape.take());
            if value != q32(0x00010000) {
                raise!("found an unknown version of the horizontal header");
            }
            Ok(value)
        },

        ascender                (i16), // Ascender
        descender               (i16), // Descender
        line_gap                (i16), // LineGap
        max_advance_width       (u16), // advanceWidthMax
        min_left_side_bearing   (i16), // minLeftSideBearing
        min_right_side_bearing  (i16), // minRightSideBearing
        max_x_extent            (i16), // xMaxExtent
        caret_slope_rise        (i16), // caretSlopeRise
        caret_slope_run         (i16), // caretSlopeRun
        caret_offset            (i16), // caretOffset
        reserved1               (i16), // reserved1
        reserved2               (i16), // reserved2
        reserved3               (i16), // reserved3
        reserved4               (i16), // reserved4
        metric_data_format      (i16), // metricDataFormat
        horizontal_metric_count (u16), // numberOfHMetrics
    }
}

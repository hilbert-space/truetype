extern crate truetype;

#[macro_use]
mod common;

mod kaushan_script {
    use truetype::{Tag, Value};

    use crate::common::setup;

    #[should_panic]
    #[test]
    fn read() {
        use truetype::OffsetTable;

        let mut file = setup!(KaushanScript);
        let OffsetTable { header, records } = ok!(OffsetTable::read(&mut file));
        assert_eq!(header.table_count, 18);
        assert_eq!(records.len(), 18);
        for record in records.iter() {
            if record.tag == Tag(*b"head") {
                assert!(ok!(record.checksum(&mut file, |i, chunk| if i == 2 {
                    0
                } else {
                    chunk
                })));
            } else {
                assert!(ok!(record.checksum(&mut file, |_, chunk| chunk)));
            }
        }
    }
}

mod source_serif {
    use truetype::{Tag, Value};

    use crate::common::setup;

    #[test]
    fn read() {
        use truetype::OffsetTable;

        let mut file = setup!(SourceSerif);
        let OffsetTable { header, records } = ok!(OffsetTable::read(&mut file));
        assert_eq!(header.table_count, 12);
        assert_eq!(header.search_range, 8 * 16);
        assert_eq!(header.entry_selector, 3);
        assert_eq!(header.range_shift, header.table_count * 16 - header.search_range);
        assert_eq!(records.len(), 12);
        for record in records.iter() {
            if record.tag == Tag(*b"head") {
                assert!(ok!(record.checksum(&mut file, |i, chunk| if i == 2 {
                    0
                } else {
                    chunk
                })));
            } else {
                assert!(ok!(record.checksum(&mut file, |_, chunk| chunk)));
            }
        }
    }
}

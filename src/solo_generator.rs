use crate::data_faking_bridge::assoc::FNVARI;

#[inline(always)]
pub fn produce_generator_value(oc: &mut String, x: &FNVARI) {
    match x {
        FNVARI::I8(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::I16(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::I32(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::I64(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::Isize(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::U8(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::U16(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::U32(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::U64(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::Usize(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::Bool(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::F32(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::F64(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::String(f) => {
            // oc.push_str("\"");
            oc.push_str(f().as_str());
            // oc.push_str("\"");
        }

        //.//
        FNVARI::CardData(_f) => {
            // oc.push_str(f().to_string().as_str());
        }

        FNVARI::InvalidCardData(_f) => {
            // oc.push_str(f().to_string().as_str());
        }

        FNVARI::TokenData(_f) => {
            // oc.push_str(f().to_string().as_str());
        }
    }
}

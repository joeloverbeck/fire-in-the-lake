use std::io::Write;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

fn does_text_contain_name_of_certain_space(text: &str, name_of_space: &str) -> bool {
    text == format!("[{}]", name_of_space) || text == format!("[{}].", name_of_space)
}

pub fn write_output_tag_for_space<'a>(
    text: &str,
    buffer: &'a mut Buffer,
) -> Result<&'a mut Buffer, String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Red))
            .set_bg(Some(Color::White)),
    ) {
        return Err(error.to_string());
    }

    let mut final_text = "";

    if does_text_contain_name_of_certain_space(text, "Saigon") {
        final_text = "SAIGON"
    } else if does_text_contain_name_of_certain_space(text, "AnLoc") {
        final_text = "AN LOC"
    } else if does_text_contain_name_of_certain_space(text, "Hue") {
        final_text = "HUE"
    } else if does_text_contain_name_of_certain_space(text, "KienGiangAnXuyen") {
        final_text = "KIEN GIANG"
    } else if does_text_contain_name_of_certain_space(text, "BaXuyen") {
        final_text = "BA XUYEN"
    } else if does_text_contain_name_of_certain_space(text, "QuangNam") {
        final_text = "QUANG NAM"
    } else if does_text_contain_name_of_certain_space(text, "BinhDinh") {
        final_text = "BINH DINH"
    } else if does_text_contain_name_of_certain_space(text, "Route4") {
        final_text = "ROUTE 4"
    } else if does_text_contain_name_of_certain_space(text, "CanTho") {
        final_text = "CAN THO"
    } else if does_text_contain_name_of_certain_space(text, "Mekong") {
        final_text = "MEKONG"
    } else if does_text_contain_name_of_certain_space(text, "KienPhong") {
        final_text = "KIEN PHONG"
    } else if does_text_contain_name_of_certain_space(text, "QuangTriThuaThien") {
        final_text = "QUANG TRI"
    } else if does_text_contain_name_of_certain_space(text, "NorthVietnam") {
        final_text = "NORTH VIETNAM"
    } else if does_text_contain_name_of_certain_space(text, "TheParrotsBeak") {
        final_text = "THE PARROT'S BEAK"
    } else if does_text_contain_name_of_certain_space(text, "QuangTinQuangNgai") {
        final_text = "QUANG TIN"
    } else if does_text_contain_name_of_certain_space(text, "QuangDucLongKhanh") {
        final_text = "QUANG DUC"
    } else if does_text_contain_name_of_certain_space(text, "BinhTuyBinhThuan") {
        final_text = "BINH TUY"
    } else if does_text_contain_name_of_certain_space(text, "PleikuDarlac") {
        final_text = "PLEIKU"
    } else if does_text_contain_name_of_certain_space(text, "CentralLaos") {
        final_text = "CENTRAL LAOS"
    } else if does_text_contain_name_of_certain_space(text, "SouthernLaos") {
        final_text = "SOUTHERN LAOS"
    } else if does_text_contain_name_of_certain_space(text, "QuiNhon") {
        final_text = "QUI NHON"
    } else if does_text_contain_name_of_certain_space(text, "KhanhHoa") {
        final_text = "KHANH HOA"
    } else if does_text_contain_name_of_certain_space(text, "KienHoaVinhBinh") {
        final_text = "KIEN HOA"
    } else if does_text_contain_name_of_certain_space(text, "PhuBonPhuYen") {
        final_text = "PHU BON"
    } else if does_text_contain_name_of_certain_space(text, "TayNinh") {
        final_text = "TAY NINH"
    } else if does_text_contain_name_of_certain_space(text, "Kontum") {
        final_text = "KONTUM"
    } else if does_text_contain_name_of_certain_space(text, "DaNang") {
        final_text = "DA NANG"
    } else if does_text_contain_name_of_certain_space(text, "CamRanh") {
        final_text = "CAM RANH"
    }

    if let Err(error) = write!(buffer, " {} ", final_text.to_uppercase()) {
        return Err(error.to_string());
    }

    Ok(buffer)
}

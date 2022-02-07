use crate::protos::proto as pb;
use hex;
use std::collections::HashMap;

lazy_static! {
    static ref MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("ffd8ffe000104a464946", "jpg");//JPEG (jpg)
        m.insert("89504e470d0a1a0a0000", "png");//PNG (png)
        m.insert("47494638396126026f01", "gif");//GIF (gif)
        m.insert("49492a00227105008037", "tif");//TIFF (tif)
        m.insert("424d228c010000000000", "bmp");//16色位图(bmp)
        m.insert("424d8240090000000000", "bmp");//24位位图(bmp)
        m.insert("424d8e1b030000000000", "bmp");//256色位图(bmp)
        m.insert("41433130313500000000", "dwg"); //CAD (dwg)
        m.insert("3c21444f435459504520", "html"); //HTML (html)
        m.insert("3c68746d6c3e0", "html"); //HTML (html)
        m.insert("3c21646f637479706520", "htm");  //HTM (htm)
        m.insert("48544d4c207b0d0a0942", "css");  //css
        m.insert("696b2e71623d696b2e71", "js") ;  //js
        m.insert("7b5c727466315c616e73", "rtf");  //Rich Text Format (rtf)
        m.insert("38425053000100000000", "psd");  //Photoshop (psd)
        m.insert("46726f6d3a203d3f6762", "eml");  //Email [Outlook Express 6] (eml)
        m.insert("d0cf11e0a1b11ae10000", "vsd");  //Visio 绘图
        m.insert("5374616E64617264204A", "mdb");  //MS Access (mdb)
        m.insert("252150532D41646F6265", "ps") ;
        m.insert("255044462d312e350d0a", "pdf");//Adobe Acrobat (pdf)
        m.insert("D0CF11E0", "xls");//xls
        m.insert("504B030414000600080000002100", "xlsx");//xls
        m.insert("d0cf11e0a1b11ae10000", "doc");//MS Excel 注意：word、msi 和 excel的文件头一样
        m.insert("504b0304140006000800", "docx");//docx文件
        m.insert("d0cf11e0a1b11ae10000", "wps");//WPS文字wps、表格et、演示dps都是一样的
        m.insert("2e524d46000000120001", "rmvb"); //rmvb/rm相同
        m.insert("464c5601050000000900", "flv"); //flv与f4v相同
        m.insert("00000020667479706d70", "mp4");
        m.insert("49443303000000002176", "mp3");
        m.insert("000001ba210001000180", "mpg");//
        m.insert("3026b2758e66cf11a6d9", "wmv");//wmv与asf相同
        m.insert("52494646e27807005741", "wav");//Wave (wav)
        m.insert("52494646246009005741", "wav");//Wave (wav)
        m.insert("52494646", "wav");//Wave (wav)
        m.insert("52494646d07d60074156", "avi");
        m.insert("1a45dfa3a34286810142", "webm");
        m.insert("4d546864000000060001", "mid");//MIDI (mid)
        m.insert("504b0304140000000800", "zip");
        m.insert("526172211a0700cf9073", "rar");
        m.insert("235468697320636f6e66", "ini");
        m.insert("504b03040a0000000000", "jar");
        m.insert("4d5a9000030000000400", "exe"); //可执行文件
        m.insert("3c25402070616765206c", "jsp"); //jsp文件
        m.insert("4d616e69666573742d56", "mf"); //MF文件
        m.insert("3c3f786d6c2076657273", "xml"); //xml文件
        m.insert("494e5345525420494e54", "sql"); //xml文件
        m.insert("7061636b616765207765", "java");       //java文件
        m.insert("406563686f206f66660d", "bat");       //bat文件
        m.insert("1f8b0800000000000000", "gz");       //gz文件
        m.insert("6c6f67346a2e726f6f74", "properties"); //bat文件
        m.insert("cafebabe0000002e0041", "class"); //bat文件
        m.insert("49545346030000006000", "chm"); //bat文件
        m.insert("04000000010000001300", "mxp"); //bat文件
        m.insert("6431303a637265617465", "torrent");
        m.insert("6D6F6F76", "mov");   //Quicktime (mov)
        m.insert("FF575043", "wpd");   //WordPerfect (wpd)
        m.insert("CFAD12FEC5FD746F", "dbx"); //Outlook Express (dbx)
        m.insert("2142444E", "pst");//Outlook (pst)
        m.insert("AC9EBD8F", "qdf");//Quicken (qdf)
        m.insert("E3828596", "pwl");//Windows Password (pwl)
        m.insert("2E7261FD", "ram");//Real Audio (ram)

        m
    };
}

pub fn get_file_type(prefix: &[u8]) -> String {
    let string = hex::encode(prefix);
    match MAP.iter().find(|(&k, _)| string.starts_with(&k)) {
        Some((_, &v)) => v.to_string(),
        None => String::from(""),
    }
}

pub fn get_content_type_by_suffix(suffix: &str) -> pb::MessageType {
    let img_list = vec!["jpeg", "jpg", "png", "gif", "tif", "bmp", "dwg"];
    let audio_list = vec!["mp3", "wma", "wav", "mid", "ape", "flac"];
    let video_list = vec![
        "rmvb", "flv", "mp4", "mpg", "mpeg", "avi", "rm", "mov", "wmv", "webm",
    ];

    if img_list.contains(&suffix) {
        pb::MessageType::Image
    } else if audio_list.contains(&suffix) {
        pb::MessageType::Audio
    } else if video_list.contains(&suffix) {
        pb::MessageType::Video
    } else {
        pb::MessageType::File
    }
}

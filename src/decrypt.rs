use aes::Aes128;
use aes::cipher::{BlockDecrypt, KeyInit, generic_array::GenericArray};
use base64;
use byteorder::{LittleEndian, ReadBytesExt};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::error::Error;
use std::fs::{File, create_dir_all, metadata};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

fn unpad(data: &[u8]) -> &[u8] {
    let pad_len = *data.last().unwrap() as usize;
    &data[..data.len() - pad_len]
}

fn decrypt_aes_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut result = Vec::with_capacity(data.len());
    for chunk in data.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        result.extend_from_slice(&block);
    }
    result
}

pub fn dump(file_path: &Path, output_dir: &Path) -> Result<String, Box<dyn Error>> {
    let core_key = hex::decode("687A4852416D736F356B496E62617857")?;
    let meta_key = hex::decode("2331346C6A6B5F215C5D2630553C2728")?;

    let file_size = metadata(file_path)?.len();
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} {msg:25} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
            )
            .unwrap()
            .progress_chars("=> "),
    );
    pb.set_message(format!(
        "{}",
        file_path.file_name().unwrap().to_string_lossy()
    ));

    let mut f = File::open(file_path)?;
    let mut header = [0u8; 8];
    f.read_exact(&mut header)?;
    if hex::encode(header) != "4354454e4644414d" {
        pb.finish_with_message("❌ 非 NCM 文件");
        return Err("不是有效的 NCM 文件".into());
    }

    f.seek(SeekFrom::Current(2))?;
    pb.inc(10);

    let key_length = f.read_u32::<LittleEndian>()?;
    let mut key_data = vec![0u8; key_length as usize];
    f.read_exact(&mut key_data)?;
    for b in key_data.iter_mut() {
        *b ^= 0x64;
    }

    let mut key_data = decrypt_aes_ecb(&key_data, &core_key);
    key_data = unpad(&key_data)[17..].to_vec();

    // 生成 key_box
    let key_len = key_data.len();
    let mut key_box: Vec<u8> = (0..=255).collect();
    let (mut last_byte, mut key_offset) = (0u8, 0usize);
    for i in 0..256 {
        let swap = key_box[i];
        last_byte = swap
            .wrapping_add(last_byte)
            .wrapping_add(key_data[key_offset])
            & 0xff;
        key_offset = (key_offset + 1) % key_len;
        key_box.swap(i, last_byte as usize);
    }

    let meta_length = f.read_u32::<LittleEndian>()?;
    let mut meta_data = vec![0u8; meta_length as usize];
    f.read_exact(&mut meta_data)?;
    for b in meta_data.iter_mut() {
        *b ^= 0x63;
    }

    let meta_data_b64 = &meta_data[22..];
    let meta_data_dec = base64::decode(meta_data_b64)?;
    let meta_data_dec = decrypt_aes_ecb(&meta_data_dec, &meta_key);
    let meta_str = String::from_utf8(unpad(&meta_data_dec)[6..].to_vec())?;
    let meta_json: Value = serde_json::from_str(&meta_str)?;

    f.seek(SeekFrom::Current(4 + 5))?;
    let image_size = f.read_u32::<LittleEndian>()?;
    f.seek(SeekFrom::Current(image_size as i64))?;

    let format = meta_json["format"].as_str().unwrap_or("mp3");
    let file_stem = file_path.file_stem().unwrap().to_string_lossy();
    let out_path: PathBuf = output_dir.join(format!("{}.{}", file_stem, format));

    create_dir_all(output_dir)?;
    let mut out = File::create(&out_path)?;

    let mut chunk = [0u8; 0x8000];
    loop {
        let read_len = f.read(&mut chunk)?;
        if read_len == 0 {
            break;
        }
        let mut data = chunk[..read_len].to_vec();
        for i in 0..read_len {
            let j = (i + 1) & 0xff;
            let t =
                (key_box[j] as usize + key_box[(key_box[j] as usize + j) & 0xff] as usize) & 0xff;
            data[i] ^= key_box[t];
        }
        out.write_all(&data)?;
        pb.inc(read_len as u64);
    }

    pb.finish_with_message(format!(
        "✅ {}",
        out_path.file_name().unwrap().to_string_lossy()
    ));
    Ok(out_path.display().to_string())
}

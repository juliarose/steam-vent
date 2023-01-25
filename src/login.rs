// todo I'd like this to require less external modules
// bytebuffer_new can probably be accomplished with bytes::BytesMut

use std::{
    fs::File,
    io::prelude::*,
    path::PathBuf,
};
use sha1::{Sha1, Digest};

use bytebuffer_new::{ByteBuffer, Endian};
use rand::Rng;
use crate::proto::{
    steammessages_base::CMsgIPAddress,
    steammessages_clientserver_login::CMsgClientLogon,
};

fn bytes_to_hex_string(input: &[u8]) -> String {
    use std::fmt::Write;

    let mut s = String::with_capacity(2 * input.len());

    for byte in input {
        write!(s, "{:02X}", byte).unwrap();
    }

    s
}

fn create_sha1(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    
    hasher.update(input);
    hasher.finalize().to_vec()
}

pub fn create_logon(
    account_name: String,
    password: String,
    machine_id_filepath: &PathBuf,
) -> CMsgClientLogon {
    let mut logon = CMsgClientLogon::new();
    
    logon.set_protocol_version(65580);
    logon.set_client_os_type(203);
    logon.set_should_remember_password(true);
    logon.set_supports_rate_limit_response(true);
    logon.set_anon_user_target_account_name(String::new());
    
    let mut ip = CMsgIPAddress::new();
    
    ip.set_v4(0);
    logon.set_obfuscated_private_ip(ip);
    logon.set_client_language(String::from("english"));
    logon.set_machine_id(get_machine_id(machine_id_filepath));
    logon.set_machine_name(String::new());
    logon.set_steamguard_dont_remember_computer(false);
    logon.set_chat_mode(2);
    logon.set_account_name(account_name);
    logon.set_password(password);
    logon.set_cell_id(79);
    logon
}

fn get_random_machine_id() -> Vec<u8> {
    fn get_machine_id_from_values(val_bb3: &str, val_ff2: &str, val_3b3: &str) -> Vec<u8> {
        fn get_c_string_bytes(input: &str) -> Vec<u8> {
            let mut bytes = input.as_bytes().to_vec();
            
            bytes.push(0);
            bytes
        }
        
        fn create_sha1_str(input: &str) -> String {
            let sha_bytes = create_sha1(input.as_bytes());

            bytes_to_hex_string(&sha_bytes)
        }
        
        let mut buffer = ByteBuffer::new();
        
        buffer.set_endian(Endian::LittleEndian);
        
        buffer.write_i8(0); // 1 byte, total 1
        buffer.write_bytes(&get_c_string_bytes("MessageObject"));
        
        buffer.write_i8(1); // 1 byte, total 16
        buffer.write_bytes(&get_c_string_bytes("BB3")); // 4 bytes, total 20
        buffer.write_bytes(&get_c_string_bytes(&create_sha1_str(val_bb3))); // 41 bytes, total 61
        
        buffer.write_i8(1); // 1 byte, total 62
        buffer.write_bytes(&get_c_string_bytes("FF2")); // 4 bytes, total 66
        buffer.write_bytes(&get_c_string_bytes(&create_sha1_str(val_ff2))); // 41 bytes, total 107
        
        buffer.write_i8(1); // 1 byte, total 108
        buffer.write_bytes(&get_c_string_bytes("3B3")); // 4 bytes, total 112
        buffer.write_bytes(&get_c_string_bytes(&create_sha1_str(val_3b3))); // 41 bytes, total 153
        
        buffer.write_i8(8); // 1 byte, total 154
        buffer.write_i8(8); // 1 byte, total 155
        buffer.to_bytes()
    }
    
    fn get_random_str() -> String {
        let mut rng = rand::thread_rng();
        
        rng.gen::<f32>().to_string()
    }
    
    get_machine_id_from_values(
        &get_random_str(),
        &get_random_str(),
        &get_random_str(),
    )
}

fn get_machine_id(filepath: &PathBuf) -> Vec<u8> {
    if let Ok(machine_id) = get_machine_id_from_file(filepath) {
        machine_id
    } else {
        let machine_id = get_random_machine_id();
        // It should be OK if this panics
        save_file(
            filepath,
            &machine_id,
        ).unwrap();
        
        machine_id
    }
}

fn get_machine_id_from_file(filepath: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let mut data: Vec<u8> = Vec::new();
    
    file.read_to_end(&mut data)?;
    
    Ok(data)
}

/// Performs a basic atomic file write.
fn save_file(
    filepath: &PathBuf,
    data: &[u8],
) -> std::io::Result<()> {
    let mut temp_filepath = filepath.clone();
    
    temp_filepath.set_extension(".temp");
    
    let mut temp_file = File::create(&temp_filepath)?;
    
    match temp_file.write_all(data) {
        Ok(_) => {
            temp_file.flush()?;
            std::fs::rename(temp_filepath, filepath)?;

            Ok(())
        },
        Err(error) => {
            // something went wrong writing to this file...
            std::fs::remove_file(&temp_filepath)?;
            
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn creates_a_machine_id() {
        let machine_id = get_random_machine_id();
        
        assert!(!machine_id.is_empty());
    }
}
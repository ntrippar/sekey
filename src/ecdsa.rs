use std::io::{Cursor, Write, Read, BufRead};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use eagre_asn1::der::DER;

#[derive(Debug)]
pub struct ECDSASign {
	pub r: Vec<u8>,
	pub s: Vec<u8>,
}

der_sequence!{
	ECDSASign:
		r: NOTAG TYPE Vec<u8>,
		s: NOTAG TYPE Vec<u8>,
}

pub static CURVE_IDENTIFIER: &'static str = "nistp256";
pub static CURVE_TYPE: &'static str = "ecdsa-sha2-nistp256";

pub struct EcdsaSha2Nistp256;
impl EcdsaSha2Nistp256 {

	// write to SSH-Key Format
	pub fn write(key: Vec<u8>) -> Vec<u8> {
		let curvetype = String::from(CURVE_TYPE);
		let identifier = String::from(CURVE_IDENTIFIER);
		let mut data = vec![];
		//write curve type
		data.write_u32::<BigEndian>(curvetype.len() as u32).unwrap();
		data.write_all(curvetype.as_bytes()).expect("error writing data!");
		//write identifier
		data.write_u32::<BigEndian>(identifier.len() as u32).unwrap();
		data.write_all(identifier.as_bytes()).expect("error writing data!");
		//write key
		data.write_u32::<BigEndian>(key.len() as u32).unwrap();
		data.write_all(key.as_slice()).expect("error writing data!") ;
		data 
	}

	//read from SSH-key Format
	pub fn read(data: Vec<u8>) -> Vec<u8> {
		let mut cursor = Cursor::new(data);
		let len = cursor.read_u32::<BigEndian>().unwrap();
		//cursor.read(len);
		cursor.consume(len as usize);
		let len = cursor.read_u32::<BigEndian>().unwrap();
		cursor.consume(len as usize);
		let len = cursor.read_u32::<BigEndian>().unwrap();
		let mut buffer = vec![0; len as usize];
		cursor.read(&mut buffer).expect("error reading SSH Key") ;
		buffer
	}

	pub fn parse_asn1(signed_data: Vec<u8>) -> ECDSASign{
		ECDSASign::der_from_bytes(signed_data).unwrap()
	}
}

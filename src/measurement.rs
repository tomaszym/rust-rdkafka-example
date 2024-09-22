use mac_address::MacAddress;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct DeviceId(Ulid);

impl Into<String> for DeviceId {
    fn into(self) -> String {
        self.0.to_string()
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Measurement {
    pub device: DeviceId,
    pub mac: MacAddress,
    pub device_group: i32,
    pub submeasurements: Vec<Submeasurements>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Submeasurements {
    pub sub_index: u8,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: i32,
    pub f: i32,
    pub g: i32,
    pub h: i32,
    pub i: i32,
}

pub fn gen_measurement() -> Measurement {
    let mut rng = rand::thread_rng();

    let mac_bytes: [u8; 6] = rng.gen();

    let mut submeasurements: Vec<Submeasurements> = Vec::with_capacity(100);

    for i in 0u8..99 {
        submeasurements.insert(usize::try_from(i).unwrap(), Submeasurements {
            sub_index: i / 10,
            a: rng.gen::<f32>(),
            b: rng.gen::<f32>(),
            c: rng.gen::<f32>(),
            d: rng.gen::<f32>(),
            e: rng.gen::<i32>(),
            f: rng.gen::<i32>(),
            g: rng.gen::<i32>(),
            h: rng.gen::<i32>(),
            i: rng.gen::<i32>(),
        })
    }
    
    Measurement {
        device: DeviceId(Ulid::from(rng.gen::<u128>())),
        mac: MacAddress::new(mac_bytes),
        device_group: rng.gen::<i32>(),
        submeasurements: submeasurements,
    }

    
}
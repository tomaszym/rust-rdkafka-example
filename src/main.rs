mod measurement;
use std::{sync::{Arc, Mutex}, time::Duration};
use futures::{stream::FuturesUnordered, StreamExt};
use measurement::gen_measurement;
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use ulid::Ulid;

#[tokio::main]
async fn main() -> () {

    let num_workers = 1;

    (0..num_workers)
        .map(|producer_number| {
            tokio::spawn(run_async_processor(
                producer_number
            ))
        })
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await

}

async fn run_async_processor(
    producer_number: i8
) {

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    loop {
            
        // let producer = producer.clone();
        async_std::task::sleep(Duration::from_millis(3000)).await;
        

        let measurement = gen_measurement();
        let key: String = measurement.device.into();

        let json = serde_json::to_string(&measurement).unwrap();

        let produce_future = producer.send(
            FutureRecord::to("measurements.json")
                .key(&key)
                .payload(&json),
            Duration::from_secs(0),
        );
        match produce_future.await {
            Ok(delivery) => println!("Sent: {:?}", delivery),
            Err((e, _)) => println!("Error: {:?}", e),
        }
    }
     
}
    
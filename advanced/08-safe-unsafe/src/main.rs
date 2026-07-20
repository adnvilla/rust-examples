use safe_unsafe::window;

fn main() {
    let packet = b"HEADpayloadTAIL";
    println!(
        "Ventana: {:?}",
        window(packet, 4, 7).expect("checked range")
    );
}

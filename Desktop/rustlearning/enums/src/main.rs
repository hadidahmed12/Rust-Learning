fn main() {

    let cash_payment = Payment::Cash(1500.50);
    process_payment(cash_payment);

    let _cc_payment = Payment::Creditcard("l1f16bscs0441".to_string(),1000.);
    process_payment(_cc_payment);

    let _dc_payment = Payment::Debitcard(Debitcarddata {
        cardnumber: "L1F16bscs0441".to_string(),
        amount: 1200.40,
    });
    process_payment(_dc_payment);

    let _crypto_payment = Payment::Crypto{account_id:"0x0000fx".to_string(),amount: 0.1};
    process_payment(_crypto_payment);

    
}

struct Debitcarddata{
        pub cardnumber: String,
        pub amount: f32,
}

enum Payment {
    Cash(f32),
    Creditcard(String, f32),
    Debitcard(Debitcarddata),
    Crypto{account_id:String, amount:f32},
}

fn process_payment(somepayment: Payment){

    match somepayment {
        Payment::Cash(amount) =>{
            println!("Hi my payment is in cash :: and the total bill is :: {}",amount);
        }
        Payment::Creditcard(some_string, _) =>{
            println!("Hi my payment is in creditcard Number is::{} ", some_string);
        }
        Payment::Debitcard(data) => {
            println!("Hi my payment is in debitcard :: CardNumber is :: {}  Amount Price is {}",data.cardnumber, data.amount);
        }
        Payment::Crypto{account_id,amount} =>{
            println!("Hi my payment is in Crypto AccountHash: {} Price: {} Ether",account_id,amount);
        }
        // _ => {}

    }

}

extern crate rand; 
use rand::Rng;

struct PartySeatsPercentage {
    aup: f64,
    op: f64,
    xrb: f64,
    ssm: f64
}

struct PartySeatsAmount {
    aup: f64,
    op: f64,
    xrb: f64,
    ssm: f64
}

struct PartySectorAmount {
    aup: f64,
    op: f64,
    xrb: f64,
    ssm: f64
}

fn main() {
    //This is the rng valuable
    let mut rng = rand::thread_rng();

    //This calculates the amount of seats in parlement based on the amount of pops in the empire
    let pops: f64 = 20.0; //amount of pops in the empire
    let seats: f64 = 200.0 + pops*10.0; //this is equal to the 200 + pops*10

    //This takes the amount of sectors per party and calculates the total amount of sectors to devide by
    let sectors_party = PartySectorAmount {
        aup: 8.0 * rng.gen_range(1.0..1.2),
        op: 4.0 * rng.gen_range(1.0..1.2),
        xrb: 6.0 * rng.gen_range(1.0..1.2),
        ssm: 2.0 * rng.gen_range(1.0..1.2)
    };
    let total_sectors: f64 = sectors_party.aup + sectors_party.op  + sectors_party.xrb + sectors_party.ssm; //this is equal to the size of all planets + "phantom sectors" combined

    // This calculates the percentage of seats the parties get from the amount of sectors they have
    let seats_percentage = PartySeatsPercentage {
        aup: percentage_of_seats(total_sectors, sectors_party.aup),
        op: percentage_of_seats(total_sectors, sectors_party.op),
        xrb: percentage_of_seats(total_sectors, sectors_party.xrb),
        ssm: percentage_of_seats(total_sectors, sectors_party.ssm),
    };

    let seats_amount = PartySeatsAmount {
        aup: seats*seats_percentage.aup,
        op: seats*seats_percentage.op,
        xrb: seats*seats_percentage.xrb,
        ssm: seats*seats_percentage.ssm,
    };

    println!("The election results are in and...
    
    The percentage of seats won by the All Union Party is {}% and the amount of seats in parlement is {}
    
    The percentage of seats won by the Orange Party is {}% and the amount of seats in parlement is {}
    
    The percentage of seats won by the Xeno Representation Bloc is {}% and the amount of seats in parlement is {}
    
    The percentage of seats won by the Steadfast Stones movement is {}% and the amount of seats in parlement is {}", (seats_percentage.aup*100.0).round(), seats_amount.aup.round(), (seats_percentage.op*100.0).round(), seats_amount.op.round(), (seats_percentage.xrb*100.0).round(), seats_amount.xrb.round(), (seats_percentage.ssm*100.0).round(), seats_amount.ssm.round());
    

}

fn percentage_of_seats(total_sectors: f64, supporting_sectors: f64) -> f64 {
    let support_percentage: f64 = supporting_sectors / total_sectors;
    support_percentage
}

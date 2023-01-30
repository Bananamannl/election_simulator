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

fn main() {
    let mut seats: f64 = 400.0; //this is equal to the 200 + pops*10
    let mut sectors: f64 = 20.0; //this is equal to the size of all planets combined
    let seats_percentage = PartySeatsPercentage {
        aup: percentage_of_seats(sectors, 8.0),
        op: percentage_of_seats(sectors, 4.0),
        xrb: percentage_of_seats(sectors, 6.0),
        ssm: percentage_of_seats(sectors, 2.0),
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
    
    The percentage of seats won by the Steadfast Stones movement is {}% and the amount of seats in parlement is {}", seats_percentage.aup*100.0, seats_amount.aup, seats_percentage.op*100.0, seats_amount.op, seats_percentage.xrb*100.0, seats_amount.xrb, seats_percentage.ssm*100.0, seats_amount.ssm);
    

}

fn percentage_of_seats(total_sectors: f64, supporting_sectors: f64) -> f64 {
    let support_percentage: f64 = supporting_sectors / total_sectors;
    support_percentage
}

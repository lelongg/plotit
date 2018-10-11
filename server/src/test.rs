use csv::ReaderBuilder;
use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<Error>> {
    let _data = "\
0.644217687237611,  0.768421872844885
0.7173560908995227, 0.6967067093471655
0.7833269096274833, 0.6216099682706645
0.8414709848078964, 0.5403023058681398
0.8912073600614353, 0.4535961214255775
";
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

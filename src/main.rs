use pest;
use pest_derive::*;
use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    println!("current working directory: {:?}", std::env::current_dir());

    let f = std::fs::read_to_string("numbers.csv").expect("cannot read file");
    // the parse result at the topmost level is rule `file`, who's span is 
    // the whole file, and it's inner list is of all the record types in the file.
    // This is why `pr.clone().next()` is a list of pairs.
    // NOTE: the clne of the parse result is needed because the later 
    // `parse_result.token()` would consume the parse result and not allow the 
    // file to be used later in the code.
    let (parse_result, file) = {
        let pr = CSVParser::parse(Rule::file, &f).expect("unsuccessful parse");
        println!("{:?}", pr);
        let f = pr.clone().next().unwrap();
        (pr, f)
    };

    // The parse result's "lowest" form are pairs of tokens that mark the
    // start and end position of the matched rule in the source material.
    // This can be useful but is also burdensome.
    let tokens = parse_result.tokens();
    println!(" tokens\n--------");
    for t in tokens {
        println!("{:?}", t);
    }

    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;

    // `file` is 
    for record in file.into_inner() {   // outermost rule is the file. This
                                        // returns the iterator of the children
                                        // of the current pair. It could have 
                                        // been called children_iterator. In
                                        // this case, child of file is the
                                        // records, or lines of the csv
        match record.as_rule() {
            Rule::record => {
                record_count += 1;
                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}

//Import standard library I/O module for stdin
extern crate rand;
use rand::Rng;

#[no_mangle]
pub extern "C" fn read_csv() {
    let mut rdr = csv::Reader::from_path("example1.csv").expect("UH OH");
    let mut rrdr = csv::Reader::from_path("example2.csv").expect("UH OH");
    let mut wdw = csv::Writer::from_path("results.csv").expect("UH OH");
    for (result, result2) in rdr.records().zip(rrdr.records()) {
        let mut diff: i32 = 0;
        let record = result.expect("a CSV record");
        let record2 = result2.expect("a CSV record");
        let mut vec = Vec::new();
        for i in 0..4 {
            let num1 = &record[i];
            let num2 = &record2[i];
            let numm1 = num1.parse::<i32>().unwrap();
            let numm2 = num2.parse::<i32>().unwrap();
            diff = (numm1 - numm2);
            vec.push(diff);
        }
        wdw.serialize(vec);
        //println!("{:?}", diff);
    }
}

#[no_mangle]
pub extern "C" fn write_csv(diff: i32) {
    let mut wdw = csv::Writer::from_path("example1.csv").expect("UH OH");
    let mut wwdw = csv::Writer::from_path("example2.csv").expect("UH OH");
    let mut rng = rand::thread_rng();
    for _x in 0..132 {
        let num: i32 = rng.gen();
        wdw.serialize(&[num, num, num, num]);
        wwdw.serialize(&[(num+diff), (num+diff),(num+diff),(num+diff)]);
    }
}



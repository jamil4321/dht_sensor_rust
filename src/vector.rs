fn main() {
    // let bin_idx = "01110011001";
    // let intval = isize:: (bin_idx, 2).unwrap();
    // println!("{}", intval);
// 

    let mut data_from_dth = vec![0,0,1,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,1,0,1];
    let mut parity:Vec<_> =data_from_dth.drain(32..).collect();
    let mut low_temp:Vec<_> = data_from_dth.drain(24..).collect();
    let mut high_tmp:Vec<_> = data_from_dth.drain(16..).collect();
    let mut low_rh:Vec<_> = data_from_dth.drain(8..).collect();
    let mut high_tmp:Vec<_> = data_from_dth.drain(0..).collect();

    

    if 29 > 30 {
        println!("{:?}",data_from_dth);
    }
}

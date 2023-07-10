fn main() {
    let nums=[2,7,11,15];
    let target=9;
    for i in 0..nums.len(){
        for j in 0..nums.len(){
            if j==nums.len() && j==target{
                println!("[{}]",j);
            }
            else if i != j{
                let shn=nums[i as usize]+nums[j as usize];
                match shn{
                    target => println!("[{},{}]",i, j),
                    _ => break,
                };
            }
            else {
                continue;
            }
        }
    }
}
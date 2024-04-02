pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut index_1: usize = 0;
    let mut index_2: usize = 0;

    let mut nums_res: Vec<i32> = Vec::new();

    for _ in 0..nums1.len()+nums2.len() {
        match nums1.get(index_1) {
            Some(num1) => {
                match nums2.get(index_2) {
                    Some(num2) => {
                        if num1 < num2 {
                            nums_res.push(num1.clone());
                            index_1 += 1;
                        }
                        else {
                            nums_res.push(num2.clone());
                            index_2 += 1;
                        }
                    },
                    None => {
                        let nums1_remains: Vec<i32> = nums1[index_1..].iter().map(|x| x.clone()).collect();
                        nums_res.extend(nums1_remains);
                        break;
                    }
                }
            },
            None => {
                let nums2_remains: Vec<i32> = nums2[index_2..].iter().map(|x| x.clone()).collect();
                nums_res.extend(nums2_remains);
                break;
            }
        }
        
    }
    
    println!("{:?}", nums_res);
    let med_index = (nums_res.len() as f32)/2.0;
    if med_index != med_index.floor() {
        return nums_res[(med_index.floor()) as usize] as f64;
    }
    else {
        return ((nums_res[(med_index-1.0) as usize] + nums_res[med_index as usize]) as f64) / 2.0;
    }

}

fn main() {
    let res = find_median_sorted_arrays(vec![1,3], vec![2]);
    println!("{res}");
}

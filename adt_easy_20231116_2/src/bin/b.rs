use proconio::input;

fn main() {
    input! {
        mut nums: [i32; 5]
    };

    nums.sort();

    if nums[0] == nums[4] {
        print!("No");
        return;
    }

    if nums[0] == nums[2] && nums[3] == nums[4] {
        print!("Yes");
        return;
    }

    if nums[0] == nums[1] && nums[2] == nums[4] {
        print!("Yes")
    } else {
        print!("No")
    }
}

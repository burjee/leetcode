struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut output = vec![];
        for n in nums {
            if let Err(i) = output.binary_search(&n) {
                if i >= output.len() {
                    output.push(n);
                } else {
                    output[i] = n;
                }
            }
        }
        output.len() as i32
    }
}

fn main() {
    let input = vec![
        vec![
            -147, -171, -584, 590, 501, 13, 489, -938, 396, -544, -229, 697, 157, -933, -264, -436,
            -691, -469, 49, -574, 694, 50, 672, -858, -923, 974, -157, -507, -907, 429, 529, -591,
            802, -351, -606, 296, -485, 454, 540, 300, -1000, 408, 923, 0, -975, -548, 62, -990,
            835, 650, 733, -611, -385, -580, 330, 394, 566, -191, 612, -608, -478, -104, -425, 58,
            -849, 601, 851, -208, -810, 400, 412, 571, -535, -995, 627, -481, -702, 457, -29, 375,
            792, -186, -921, -275, 654, -356, -322, -28, -843, 527, -266, -970, 556, 852, -890,
            169, -413, 2, -958, -651, 371, 895, -994, 671, 243, -605, -556, 735, -246, 179, -104,
            -771, 658, -554, 932, -829, -455, -981, -731, -148, 512, -547, -946, -997, -197, 864,
            870, 629, -961, 659, 574, 543, -501, 582, -799, -428, 876, -334, 115, 759, 197, -905,
            275, 76, 242, 357, 694, -254, -361, -338, -57, 596, 786, -710, -51, -496, -100, 246,
            -969, 874, 504, 938, 931, -365, 175, -40, -616, 596, 440, 567, 999, 15, -363, -256,
            -578, -869, -653, 78, -352, 882, 749, -33, 462, -592, -751, 761, -96, 206, 489, 34,
            367, 960, 68, 837, 37, -764, -897, 72, 639, -69, 353, 836, -67, 491, 126, -171, -532,
            -757, -358, 217, 806, 712, -32, 843, -790, -691, -381, -138, 6, -712, 153, -184, -544,
            3, 840, -561, 917, -704, -126, -230, 468, 963, -993, 445, -892, -543, 941, -665, 58,
            268, -362, 181, -529, 684, 313, -380, -712, 700, 601, -962, -886, 702, 439, 153, -87,
            140, 583, -323, 70, -460, -863, -859, -784, 571, 169, 44, -460, 181, 883, 600, 982,
            -367, -191, 815, -84, 961, -791, -713, 149, -499, 330, -351, -442, -989, -662, -183,
            -220, -617, -638, -916, 454, 604, 559, -304, -812, 526, -891, 984, -762, -669, -414,
            -481, -219, -776, 690, -72, -250, -282, -961,
        ],
        vec![10, 9, 2, 5, 3, 7, 101, 18],
        vec![10, 9, 2, 5, 3, 7, 4, 5, 6, 101, 18],
        vec![0, 3, 1, 6, 2, 2, 7],
        vec![0, 1, 0, 3, 2, 3],
        vec![7, 7, 7, 7, 7, 7, 7],
    ];
    for nums in input {
        println!("{}", Solution::length_of_lis(nums));
    }
}

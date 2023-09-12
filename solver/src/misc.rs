pub fn c_nk(n: u16, mut k: u16) -> u16 {
    if n < k {
        return 0;
    };

    if k > n / 2 {
        k = n - k;
    };

    let mut s = 1;
    let mut i = n;
    let mut j = 1;

    while i != n - k {
        s *= i;
        s /= j;
        i -= 1;
        j += 1;
    }

    s
}

pub fn rotate_left<T>(arr: &mut [T], left: usize, right: usize) where T: Copy {
    let tmp = arr[left];
    for i in left..right {
        arr[i] = arr[i + 1]
    }
    arr[right] = tmp;
}

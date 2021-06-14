use std::io::Write;
use std::str::FromStr;

fn main() {
    print!("Nearby chunk X: ");
    let nearby_x: i32 = match read_typed() {
        None => {
            println!("Invalid integer");
            return;
        }
        Some(x) => x
    };
    print!("Nearby chunk Z: ");
    let nearby_z: i32 = match read_typed() {
        None => {
            println!("Invalid integer");
            return;
        }
        Some(z) => z
    };
    print!("Target hash: ");
    let target_hash: u32 = match read_typed() {
        None => {
            println!("Invalid integer");
            return;
        }
        Some(hash) => hash
    };
    print!("Hash size: ");
    let hash_size: u32 = match read_typed() {
        None => {
            println!("Invalid integer");
            return;
        }
        Some(hash_size) => hash_size
    };

    if !hash_size.is_power_of_two() {
        println!("Hash size must be a power of 2");
        return;
    }
    if hash_size < 4 {
        println!("Hash size is too small");
        return;
    }

    if target_hash >= hash_size {
        println!("Target hash must be in [0, hash_size)");
        return;
    }

    print!("Max count: ");
    let max_count: u32 = match read_typed() {
        None => {
            println!("Invalid integer");
            return;
        }
        Some(count) => count
    };

    let mut radius = 1;
    let mut count = 0;

    if check(nearby_x, nearby_z, target_hash, hash_size) {
        count += 1;
    }

    while count < max_count {
        for dx in -radius..radius {
            if check(nearby_x + dx, nearby_z + radius - dx.abs(), target_hash, hash_size) {
                count += 1;
            }
            if check(nearby_x - dx, nearby_z - radius + dx.abs(), target_hash, hash_size) {
                count += 1;
            }
        }
        radius += 1;
    }
}

fn check(x: i32, z: i32, target_hash: u32, hash_size: u32) -> bool {
    let as_long = ((z as u32 as u64) << 32) | (x as u32 as u64);
    let mut long_hash = as_long.wrapping_mul(0x9E3779B97F4A7C15);
    long_hash ^= long_hash >> 32;
    long_hash ^= long_hash >> 16;
    let hash = (long_hash & (hash_size - 1) as u64) as u32;
    return if hash == target_hash {
        println!("{}, {}", x, z);
        true
    } else {
        false
    }
}

fn read_line() -> Option<String> {
    std::io::stdout().flush().ok()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok()?;
    return Some(String::from(line.trim_end()));
}

fn read_typed<T: FromStr>() -> Option<T> {
    let line = read_line()?;
    return line.parse().ok();
}

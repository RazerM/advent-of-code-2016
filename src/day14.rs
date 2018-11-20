use std::collections::HashSet;
use std::str;

use crypto::md5::Md5;
use crypto::digest::Digest;
use iterslide::SlideIterator;
use rayon::prelude::*;

use super::util::stdin_as_string;

fn compute_hash(salt: &[u8], index: i32) -> String {
    let mut hasher = Md5::new();
    hasher.input(salt);
    hasher.input(index.to_string().as_bytes());
    hasher.result_str()
}

fn compute_stretched_hash(salt: &[u8], index: i32) -> String {
    let mut hasher = Md5::new();
    hasher.input(salt);
    hasher.input(index.to_string().as_bytes());

    let mut hash = hasher.result_str();

    for _ in 0..2016 {
        hasher.reset();
        hasher.input(hash.as_bytes());
        hash = hasher.result_str()
    }
    hash
}

#[derive(Debug)]
struct HashResult {
    triplet: Option<char>,
    quintuplets: HashSet<char>,
}

fn first_triplet(s: &str) -> Option<char> {
    s.chars()
        .slide(3)
        .filter(|c| c[0] == c[1] && c[0] == c[2])
        .map(|c| c[0])
        .next()
}

fn quintuplets(s: &str) -> HashSet<char> {
    s.chars()
        .slide(5)
        .filter(|c| c[0] == c[1] && c[0] == c[2] && c[0] == c[3] && c[0] == c[4])
        .map(|c| c[0])
        .collect::<HashSet<_>>()
}

fn make_hashes<F>(salt: &[u8], start: i32, num: i32, compute_hash: &F) -> Vec<HashResult>
where
    F: Fn(&[u8], i32) -> String + Send + Sync,
{
    let end = start + num;
    (start..end).into_par_iter()
        .map(|x| compute_hash(salt, x))
        .map(|h| HashResult {
            triplet: first_triplet(&h),
            quintuplets: quintuplets(&h),
        })
        .collect::<Vec<_>>()
}

fn find_64th_key<F>(salt: &[u8], compute_hash: &F) -> Option<usize>
where
    F: Fn(&[u8], i32) -> String + Send + Sync
{
    let chunk_size = 10_000;
    let mut hashes = Vec::with_capacity(chunk_size as usize);

    let mut found = 0;

    for i in 0.. {
        let start = i + 1;
        let end = start + 1000;
        if end >= hashes.len() {
            let new_start = hashes.len() as i32;
            hashes.append(&mut make_hashes(salt, new_start, chunk_size, compute_hash));
        }

        let hash = &hashes[i];
        if let Some(triplet) = hash.triplet {
            let next_1000 = &hashes[start..end];
            let is_key = next_1000.iter().any(|h| h.quintuplets.contains(&triplet));

            if is_key {
                found += 1;
                if found == 64 {
                    return Some(i)
                }
            }
        }
    }
    None
}

pub(crate) fn solve() {
    let input = stdin_as_string();
    let salt = input.as_bytes();
    println!("Part 1: {}", find_64th_key(salt, &compute_hash).unwrap());
    println!("Part 2: {}", find_64th_key(salt, &compute_stretched_hash).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let salt = b"abc";
        let part1 = find_64th_key(salt, &compute_hash);
        let part2 = find_64th_key(salt, &compute_stretched_hash);
        assert_eq!(part1, Some(22728));
        assert_eq!(part2, Some(22551));
    }
}
